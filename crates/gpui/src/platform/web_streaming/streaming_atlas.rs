//! A CPU-side atlas that implements `PlatformAtlas` without any GPU dependency.
//!
//! The native GPUI renderers use `WgpuAtlas` or `MetalAtlas` which pack
//! rasterized glyphs, SVGs, and images into GPU textures. This atlas does
//! the same bin-packing and tile allocation but keeps the pixel data in
//! plain `Vec<u8>` buffers so it can be serialized and streamed to the
//! browser-side renderer.
//!
//! After each frame, the caller should drain `take_deltas()` to get the
//! list of newly allocated tiles and their pixel data. These deltas are
//! sent to the browser as `AtlasDeltaMessage` values so the browser can
//! update its local atlas mirror.

use std::borrow::Cow;
use std::ops;

use anyhow::Result;
use collections::FxHashMap;
use etagere::{BucketedAtlasAllocator, size2};
use parking_lot::Mutex;

use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Point, Size,
};

const DEFAULT_ATLAS_SIZE: i32 = 1024;

fn device_size_to_etagere(size: Size<DevicePixels>) -> etagere::Size {
    size2(size.width.0, size.height.0)
}

fn etagere_point_to_device(point: etagere::Point) -> Point<DevicePixels> {
    Point {
        x: DevicePixels(point.x),
        y: DevicePixels(point.y),
    }
}

/// A pending tile upload that was allocated during this frame. The caller
/// retrieves these via `take_deltas()` after each frame to stream them
/// to the browser.
#[derive(Debug, Clone)]
pub struct AtlasTileDelta {
    pub texture_id: AtlasTextureId,
    pub bounds: Bounds<DevicePixels>,
    pub format: AtlasTextureKind,
    pub bytes: Vec<u8>,
}

pub struct StreamingAtlas(Mutex<StreamingAtlasState>);

struct StreamingAtlasState {
    storage: StreamingAtlasStorage,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
    pending_deltas: Vec<AtlasTileDelta>,
}

impl StreamingAtlas {
    pub fn new() -> Self {
        StreamingAtlas(Mutex::new(StreamingAtlasState {
            storage: StreamingAtlasStorage::default(),
            tiles_by_key: Default::default(),
            pending_deltas: Vec::new(),
        }))
    }

    /// Called at the start of each frame. Currently a no-op but mirrors the
    /// `WgpuAtlas::before_frame` API for consistency.
    pub fn before_frame(&self) {
        // Nothing to do -- the CPU atlas doesn't need per-frame bookkeeping
        // like flushing pending GPU uploads.
    }

    /// Drain all tile deltas that were allocated since the last call.
    /// The caller sends these to the browser so it can update its atlas mirror.
    pub fn take_deltas(&self) -> Vec<AtlasTileDelta> {
        let mut lock = self.0.lock();
        std::mem::take(&mut lock.pending_deltas)
    }
}

impl PlatformAtlas for StreamingAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            Ok(Some(tile.clone()))
        } else {
            let Some((size, bytes)) = build()? else {
                return Ok(None);
            };
            let kind = key.texture_kind();
            let tile = lock.allocate(size, kind);
            lock.store_tile_data(tile.texture_id, tile.bounds, kind, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(Some(tile))
        }
    }

    fn remove(&self, key: &AtlasKey) {
        let mut lock = self.0.lock();

        let Some(id) = lock.tiles_by_key.remove(key).map(|tile| tile.texture_id) else {
            return;
        };

        let Some(texture_slot) = lock.storage[id.kind].textures.get_mut(id.index as usize) else {
            return;
        };

        if let Some(mut texture) = texture_slot.take() {
            texture.decrement_ref_count();
            if texture.is_unreferenced() {
                lock.storage[id.kind]
                    .free_list
                    .push(texture.id.index as usize);
            } else {
                *texture_slot = Some(texture);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StreamingAtlasState {
    fn allocate(&mut self, size: Size<DevicePixels>, kind: AtlasTextureKind) -> AtlasTile {
        // Try to allocate from an existing texture.
        if let Some(tile) = self.storage[kind]
            .textures
            .iter_mut()
            .flatten()
            .rev()
            .find_map(|texture| texture.allocate(size))
        {
            return tile;
        }

        // No existing texture has space -- create a new one.
        let texture = self.push_texture(size, kind);
        texture
            .allocate(size)
            .expect("Failed to allocate from newly created texture")
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut StreamingAtlasTexture {
        let default_size = Size {
            width: DevicePixels(DEFAULT_ATLAS_SIZE),
            height: DevicePixels(DEFAULT_ATLAS_SIZE),
        };

        let size = min_size.max(&default_size);
        let bytes_per_pixel = bytes_per_pixel_for_kind(kind);

        let pixel_count = size.width.0 as usize * size.height.0 as usize;
        let pixels = vec![0u8; pixel_count * bytes_per_pixel];

        let texture_list = &mut self.storage[kind];
        let index = texture_list.free_list.pop();

        let texture = StreamingAtlasTexture {
            id: AtlasTextureId {
                index: index.unwrap_or(texture_list.textures.len()) as u32,
                kind,
            },
            allocator: BucketedAtlasAllocator::new(device_size_to_etagere(size)),
            width: size.width.0,
            height: size.height.0,
            bytes_per_pixel,
            pixels,
            live_atlas_keys: 0,
        };

        if let Some(ix) = index {
            texture_list.textures[ix] = Some(texture);
            texture_list
                .textures
                .get_mut(ix)
                .and_then(|t| t.as_mut())
                .expect("texture must exist")
        } else {
            texture_list.textures.push(Some(texture));
            texture_list
                .textures
                .last_mut()
                .and_then(|t| t.as_mut())
                .expect("texture must exist")
        }
    }

    fn store_tile_data(
        &mut self,
        id: AtlasTextureId,
        bounds: Bounds<DevicePixels>,
        kind: AtlasTextureKind,
        bytes: &[u8],
    ) {
        let bpp = bytes_per_pixel_for_kind(kind);
        let tile_width = bounds.size.width.0 as usize;
        let tile_height = bounds.size.height.0 as usize;
        let origin_x = bounds.origin.x.0 as usize;
        let origin_y = bounds.origin.y.0 as usize;

        // Copy tile data into the texture's pixel buffer.
        let texture = &mut self.storage[id];
        let tex_width = texture.width as usize;

        for row in 0..tile_height {
            let src_start = row * tile_width * bpp;
            let src_end = src_start + tile_width * bpp;
            let dst_start = ((origin_y + row) * tex_width + origin_x) * bpp;
            let dst_end = dst_start + tile_width * bpp;

            if src_end <= bytes.len() && dst_end <= texture.pixels.len() {
                texture.pixels[dst_start..dst_end].copy_from_slice(&bytes[src_start..src_end]);
            }
        }

        // Queue a delta for the browser. We send just the tile region, not
        // the entire texture, to minimize bandwidth.
        self.pending_deltas.push(AtlasTileDelta {
            texture_id: id,
            bounds,
            format: kind,
            bytes: bytes.to_vec(),
        });
    }
}

fn bytes_per_pixel_for_kind(kind: AtlasTextureKind) -> usize {
    match kind {
        AtlasTextureKind::Monochrome => 1,
        AtlasTextureKind::Subpixel | AtlasTextureKind::Polychrome => 4,
    }
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

#[derive(Default)]
struct StreamingAtlasStorage {
    monochrome_textures: StreamingAtlasTextureList,
    subpixel_textures: StreamingAtlasTextureList,
    polychrome_textures: StreamingAtlasTextureList,
}

impl ops::Index<AtlasTextureKind> for StreamingAtlasStorage {
    type Output = StreamingAtlasTextureList;
    fn index(&self, kind: AtlasTextureKind) -> &Self::Output {
        match kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Subpixel => &self.subpixel_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
        }
    }
}

impl ops::IndexMut<AtlasTextureKind> for StreamingAtlasStorage {
    fn index_mut(&mut self, kind: AtlasTextureKind) -> &mut Self::Output {
        match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Subpixel => &mut self.subpixel_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
        }
    }
}

impl ops::Index<AtlasTextureId> for StreamingAtlasStorage {
    type Output = StreamingAtlasTexture;
    fn index(&self, id: AtlasTextureId) -> &Self::Output {
        let textures = &self[id.kind];
        textures.textures[id.index as usize]
            .as_ref()
            .expect("texture must exist")
    }
}

impl ops::IndexMut<AtlasTextureId> for StreamingAtlasStorage {
    fn index_mut(&mut self, id: AtlasTextureId) -> &mut Self::Output {
        let textures = &mut self[id.kind];
        textures.textures[id.index as usize]
            .as_mut()
            .expect("texture must exist")
    }
}

// ---------------------------------------------------------------------------
// Texture list (mirrors WgpuAtlas's AtlasTextureList)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct StreamingAtlasTextureList {
    textures: Vec<Option<StreamingAtlasTexture>>,
    free_list: Vec<usize>,
}

// ---------------------------------------------------------------------------
// Individual atlas texture
// ---------------------------------------------------------------------------

struct StreamingAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    width: i32,
    height: i32,
    bytes_per_pixel: usize,
    /// Raw pixel data for the entire texture. Tiles are written into this
    /// buffer as they are allocated, and the relevant sub-regions are sent
    /// to the browser as deltas.
    pixels: Vec<u8>,
    live_atlas_keys: u32,
}

impl StreamingAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(device_size_to_etagere(size))?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            padding: 0,
            bounds: Bounds {
                origin: etagere_point_to_device(allocation.rectangle.min),
                size,
            },
        };
        self.live_atlas_keys += 1;
        Some(tile)
    }

    fn decrement_ref_count(&mut self) {
        self.live_atlas_keys -= 1;
    }

    fn is_unreferenced(&self) -> bool {
        self.live_atlas_keys == 0
    }
}
