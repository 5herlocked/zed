// Client-side atlas texture mirror.
//
// The server-side GPUI atlas packs rasterized glyphs, SVGs, and images into
// texture atlases using the etagere bin-packing allocator. Atlas textures are
// 1024x1024 by default and come in three kinds:
//
//   - Monochrome (r8unorm): text glyphs and SVG icons
//   - Subpixel (bgra8unorm): subpixel-rendered text glyphs
//   - Polychrome (bgra8unorm): emoji and raster images
//
// This module maintains a mirror of those atlas textures on the browser side
// as WebGPU textures. When the server streams AtlasDelta messages containing
// new tile pixel data, we upload them into the corresponding texture region.
// Sprite primitives in the scene reference tiles by AtlasTextureId + bounds,
// which this mirror can resolve to a GPUTexture + GPUTextureView for binding
// during rendering.

import { AtlasTextureId, AtlasTextureKind, AtlasDelta, AtlasBounds } from "./protocol";

const DEFAULT_ATLAS_SIZE = 1024;

interface AtlasTexture {
  id: AtlasTextureId;
  texture: GPUTexture;
  view: GPUTextureView;
  format: GPUTextureFormat;
  width: number;
  height: number;
}

function formatForKind(kind: AtlasTextureKind): GPUTextureFormat {
  switch (kind) {
    case AtlasTextureKind.Monochrome:
      return "r8unorm";
    case AtlasTextureKind.Subpixel:
    case AtlasTextureKind.Polychrome:
      return "bgra8unorm";
  }
}

function bytesPerPixelForKind(kind: AtlasTextureKind): number {
  switch (kind) {
    case AtlasTextureKind.Monochrome:
      return 1;
    case AtlasTextureKind.Subpixel:
    case AtlasTextureKind.Polychrome:
      return 4;
  }
}

// Texture key string for indexing into our maps. We need to distinguish
// textures by both their kind and their index within that kind.
function textureKey(id: AtlasTextureId): string {
  return `${id.kind}:${id.index}`;
}

export class Atlas {
  private device: GPUDevice;
  private queue: GPUQueue;
  private textures: Map<string, AtlasTexture> = new Map();
  private sampler_: GPUSampler;

  constructor(device: GPUDevice) {
    this.device = device;
    this.queue = device.queue;

    this.sampler_ = device.createSampler({
      magFilter: "linear",
      minFilter: "linear",
      addressModeU: "clamp-to-edge",
      addressModeV: "clamp-to-edge",
    });
  }

  get sampler(): GPUSampler {
    return this.sampler_;
  }

  // Apply a batch of atlas deltas received from the server. Each delta
  // represents a rectangular region of pixel data that should be uploaded
  // into the corresponding atlas texture. If the texture doesn't exist yet,
  // we create it.
  applyDeltas(deltas: AtlasDelta[]): void {
    for (const delta of deltas) {
      this.uploadTile(delta.texture_id, delta.bounds, delta.format, delta.bytes);
    }
  }

  // Upload raw pixel data for a single tile into the atlas.
  uploadTile(textureId: AtlasTextureId, bounds: AtlasBounds, kind: AtlasTextureKind, bytes: Uint8Array): void {
    const texture = this.ensureTexture(textureId, kind, bounds);
    const bytesPerPixel = bytesPerPixelForKind(kind);
    const bytesPerRow = bounds.size.width * bytesPerPixel;

    // WebGPU requires bytesPerRow to be a multiple of 256. If the tile's
    // row stride doesn't meet that requirement, we need to copy row by row
    // into a padded buffer.
    const alignedBytesPerRow = Math.ceil(bytesPerRow / 256) * 256;

    let uploadBytes: Uint8Array<ArrayBuffer>;
    if (alignedBytesPerRow === bytesPerRow) {
      uploadBytes = new Uint8Array(bytes);
    } else {
      uploadBytes = new Uint8Array(alignedBytesPerRow * bounds.size.height);
      for (let row = 0; row < bounds.size.height; row++) {
        uploadBytes.set(bytes.subarray(row * bytesPerRow, row * bytesPerRow + bytesPerRow), row * alignedBytesPerRow);
      }
    }

    this.queue.writeTexture(
      {
        texture: texture.texture,
        origin: {
          x: bounds.origin.x,
          y: bounds.origin.y,
          z: 0,
        },
      },
      uploadBytes,
      {
        offset: 0,
        bytesPerRow: alignedBytesPerRow,
        rowsPerImage: bounds.size.height,
      },
      {
        width: bounds.size.width,
        height: bounds.size.height,
        depthOrArrayLayers: 1,
      },
    );
  }

  // Get the GPUTextureView for a given atlas texture id. Returns null if the
  // texture hasn't been created yet (which shouldn't happen in normal operation
  // since atlas deltas arrive before the scene references them).
  getTextureView(id: AtlasTextureId): GPUTextureView | null {
    const entry = this.textures.get(textureKey(id));
    return entry?.view ?? null;
  }

  // Get the GPUTexture for a given atlas texture id.
  getTexture(id: AtlasTextureId): GPUTexture | null {
    const entry = this.textures.get(textureKey(id));
    return entry?.texture ?? null;
  }

  // Get the dimensions of an atlas texture, needed for computing UV coordinates.
  getTextureSize(id: AtlasTextureId): { width: number; height: number } | null {
    const entry = this.textures.get(textureKey(id));
    if (!entry) return null;
    return { width: entry.width, height: entry.height };
  }

  // Create a placeholder 1x1 white texture view, useful as a fallback when
  // a sprite references a texture that hasn't arrived yet.
  createPlaceholderView(kind: AtlasTextureKind): GPUTextureView {
    const format = formatForKind(kind);
    const bytesPerPixel = bytesPerPixelForKind(kind);
    const texture = this.device.createTexture({
      label: "atlas-placeholder",
      size: { width: 1, height: 1, depthOrArrayLayers: 1 },
      format,
      usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
    });
    const white = new Uint8Array(bytesPerPixel);
    white.fill(255);
    this.queue.writeTexture({ texture }, white, { bytesPerRow: bytesPerPixel }, { width: 1, height: 1 });
    return texture.createView();
  }

  // Ensure a texture exists for the given id and kind. If the tile bounds
  // exceed the current texture size, we recreate the texture at a larger size
  // (this handles the case where the server allocates atlas textures larger
  // than the default 1024x1024).
  private ensureTexture(id: AtlasTextureId, kind: AtlasTextureKind, tileBounds: AtlasBounds): AtlasTexture {
    const key = textureKey(id);
    const existing = this.textures.get(key);

    const requiredWidth = tileBounds.origin.x + tileBounds.size.width;
    const requiredHeight = tileBounds.origin.y + tileBounds.size.height;

    if (existing && existing.width >= requiredWidth && existing.height >= requiredHeight) {
      return existing;
    }

    const width = Math.max(existing?.width ?? DEFAULT_ATLAS_SIZE, requiredWidth);
    const height = Math.max(existing?.height ?? DEFAULT_ATLAS_SIZE, requiredHeight);
    const format = formatForKind(kind);

    const texture = this.device.createTexture({
      label: `atlas-${kind}-${id.index}`,
      size: { width, height, depthOrArrayLayers: 1 },
      format,
      usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC,
    });

    // If we're replacing an existing texture, copy its contents into the new one.
    if (existing) {
      const encoder = this.device.createCommandEncoder({
        label: "atlas-resize-copy",
      });
      encoder.copyTextureToTexture(
        { texture: existing.texture },
        { texture },
        { width: existing.width, height: existing.height, depthOrArrayLayers: 1 },
      );
      this.queue.submit([encoder.finish()]);
      existing.texture.destroy();
    }

    const view = texture.createView();
    const entry: AtlasTexture = { id, texture, view, format, width, height };
    this.textures.set(key, entry);
    return entry;
  }

  // Release all GPU resources.
  destroy(): void {
    for (const entry of this.textures.values()) {
      entry.texture.destroy();
    }
    this.textures.clear();
  }
}
