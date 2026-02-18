//! A wrapper atlas that captures tile pixel data for web streaming.
//!
//! `MirroringAtlas` wraps any `PlatformAtlas` implementation (MetalAtlas,
//! WgpuAtlas, etc.) and intercepts the build callback on cache misses to
//! capture a CPU copy of each tile's pixel data. Captured data is stored
//! in a persistent map keyed by (texture_id, bounds) so it can be served
//! on subsequent frames without re-rasterization.
//!
//! After each frame, `take_frame_tiles()` returns deduplicated tile data
//! for all sprites that were painted this frame. The scene observer uses
//! this to include atlas tile data in the frame message.

use std::borrow::Cow;
use std::sync::Arc;

use anyhow::Result;
use collections::FxHashMap;
use parking_lot::Mutex;

use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Size,
};

/// Stored pixel data for a single atlas tile.
#[derive(Debug, Clone)]
pub struct CachedTileData {
    pub texture_id: AtlasTextureId,
    pub bounds: Bounds<DevicePixels>,
    pub format: AtlasTextureKind,
    pub bytes: Vec<u8>,
}

/// Simple 6-field key for tile identity. Two tiles at the same position
/// in the same texture are the same tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TileKey {
    tex_index: u32,
    tex_kind: u32,
    ox: i32,
    oy: i32,
    w: i32,
    h: i32,
}

impl TileKey {
    fn from_tile(tile: &AtlasTile) -> Self {
        Self {
            tex_index: tile.texture_id.index,
            tex_kind: tile.texture_id.kind as u32,
            ox: tile.bounds.origin.x.0,
            oy: tile.bounds.origin.y.0,
            w: tile.bounds.size.width.0,
            h: tile.bounds.size.height.0,
        }
    }
}

struct MirroringAtlasState {
    /// Persistent cache: tile identity → pixel data.
    /// Populated on cache miss, never evicted.
    cache: FxHashMap<TileKey, CachedTileData>,

    /// Tiles newly rasterized during this frame (cache misses only).
    /// These are the only tiles the browser doesn't already have.
    new_tiles: Vec<CachedTileData>,
}

/// A `PlatformAtlas` wrapper that captures tile pixel data for streaming.
///
/// On cache miss in the inner atlas, the build callback runs and we capture
/// the rasterized bytes into a persistent map. On cache hit, the bytes are
/// already in the map from a previous miss. After each frame, the scene
/// observer calls `take_frame_tiles()` to get deduplicated tile data for
/// all sprites painted this frame.
pub struct MirroringAtlas {
    inner: Arc<dyn PlatformAtlas>,
    state: Mutex<MirroringAtlasState>,
}

impl MirroringAtlas {
    /// Create a new mirroring atlas wrapping the given platform atlas.
    pub fn new(inner: Arc<dyn PlatformAtlas>) -> Self {
        Self {
            inner,
            state: Mutex::new(MirroringAtlasState {
                cache: FxHashMap::default(),
                new_tiles: Vec::new(),
            }),
        }
    }

    /// Drain tiles that were newly rasterized since the last call.
    /// Only returns cache misses -- tiles the browser doesn't have yet.
    /// The browser's atlas textures persist between frames, so previously
    /// sent tiles don't need re-sending.
    pub fn take_frame_tiles(&self) -> Vec<CachedTileData> {
        std::mem::take(&mut self.state.lock().new_tiles)
    }

    /// Get a reference to the underlying platform atlas.
    pub fn inner(&self) -> &Arc<dyn PlatformAtlas> {
        &self.inner
    }
}

impl PlatformAtlas for MirroringAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        let texture_kind = key.texture_kind();

        // Wrap the build callback to intercept pixel data on cache miss.
        let mut captured_bytes: Option<Vec<u8>> = None;

        let mut intercepting_build = || {
            let result = build()?;
            if let Some((_, ref bytes)) = result {
                captured_bytes = Some(bytes.to_vec());
            }
            Ok(result)
        };

        let tile = self
            .inner
            .get_or_insert_with(key, &mut intercepting_build)?;

        if let Some(ref tile) = tile {
            let tile_key = TileKey::from_tile(tile);
            let mut state = self.state.lock();

            // On cache miss (build ran), store in persistent cache AND
            // add to new_tiles for this frame. Only cache misses produce
            // new tile data that the browser needs.
            if let Some(bytes) = captured_bytes {
                let data = CachedTileData {
                    texture_id: tile.texture_id,
                    bounds: tile.bounds,
                    format: texture_kind,
                    bytes,
                };
                state.cache.insert(tile_key, data.clone());
                state.new_tiles.push(data);
            }
        }

        Ok(tile)
    }

    fn remove(&self, key: &AtlasKey) {
        self.inner.remove(key);
        // Intentionally keep the cache entry -- the tile data might still
        // be referenced by in-flight frames.
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
