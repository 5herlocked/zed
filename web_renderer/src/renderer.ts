// WebGPU renderer for the Zed web streaming client.
//
// This module creates one render pipeline per primitive type (matching the
// native GPUI wgpu renderer in crates/gpui/src/platform/wgpu/wgpu_renderer.rs)
// and draws each frame by iterating over batched primitives in draw-order,
// uploading instance data to storage buffers, and issuing instanced draw calls.
//
// The rendering model is intentionally close to the Rust side: each primitive
// type gets its own storage buffer, and the shader reads instances by
// @builtin(instance_index). Vertices are generated from vertex_index (a unit
// quad: 4 vertices, 6 indices via triangle strip with degenerate handling, or
// simply 2 triangles from 4 verts indexed 0-1-2, 2-1-3).

import { Atlas } from "./atlas";
import {
  Scene,
  Quad,
  Shadow,
  Underline,
  MonochromeSprite,
  SubpixelSprite,
  PolychromeSprite,
  AtlasTextureId,
  AtlasTextureKind,
  PrimitiveKind,
  FrameMessage,
  Hsla,
  Background,
  Bounds,
  Corners,
  Edges,
  TransformationMatrix,
  AtlasTile,
} from "./protocol";
import shaderSource from "./shaders.wgsl";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

// Initial storage buffer capacity in bytes. Grows as needed.
const INITIAL_BUFFER_CAPACITY = 256 * 1024;

// ---------------------------------------------------------------------------
// GPU struct sizes (in bytes, matching shader repr)
//
// These must match the WGSL struct layouts exactly. All structs use std430-like
// packing since they live in storage buffers.
// ---------------------------------------------------------------------------

// Background: tag(4) + color_space(4) + solid(Hsla=16) + gradient_angle(4) + pad_to_16(12) + colors(2*32=64) + pad(4) + more_pad(12)
// Let's compute carefully from the WGSL:
//   tag: u32 (4)
//   color_space: u32 (4)
//   solid: Hsla (16)
//   gradient_angle_or_pattern_height: f32 (4)
//   colors: array<LinearColorStop, 2> -- each LinearColorStop is {color: Hsla(16), percentage: f32(4)} = 20, padded to 32 for alignment
//   pad: u32 (4)
// Total: 4 + 4 + 16 + 4 + (2 * 32) + 4 = 96
// But we need to account for WGSL padding rules. Let's be precise with std430:
// Actually for storage buffers WGSL uses "natural alignment". LinearColorStop has alignment of 16 (from Hsla).
// So LinearColorStop = Hsla(16) + f32(4) + 12 padding = 32 bytes.
// Background = u32(4) + u32(4) + Hsla(offset 8, but Hsla has align 16, so pad to 16) = offset 16 + Hsla(16) = offset 32
//   + f32(4) = offset 36, then array<LinearColorStop,2> has align 16, so pad to 48.
//   array = 2*32 = 64, offset = 48+64 = 112. Then pad: u32(4) = offset 112+4 = 116.
//   Struct must be aligned to max member alignment = 16, so size = 128.
// Hmm, this is getting complex. Let me compute from the Rust repr(C) side instead.

// For JSON-based Phase 1, we don't need exact byte layouts for the storage buffers
// because we'll pack the data ourselves. What matters is that we write the fields
// in the same order and at the correct offsets that the WGSL shader expects.
//
// We'll define packing functions that write each struct field by field, computing
// offsets from the WGSL struct definitions.

// Quad GPU size: measured from the WGSL struct
// order: u32(4) + border_style: u32(4) + bounds: Bounds(16) + content_mask: Bounds(16)
//   + background: Background(??) + border_color: Hsla(16) + corner_radii: Corners(16)
//   + border_widths: Edges(16)
// We need to know Background size on GPU. Let's just measure it empirically
// by counting fields with padding.

// For now, let's define sizes by direct computation matching std430 / WGSL rules.
// The safest approach: define a helper that packs into an ArrayBuffer and let
// the GPU validate.

// ---------------------------------------------------------------------------
// Renderer
// ---------------------------------------------------------------------------

export interface RendererOptions {
  canvas: HTMLCanvasElement;
  device: GPUDevice;
  atlas: Atlas;
}

interface PipelineSet {
  quads: GPURenderPipeline;
  shadows: GPURenderPipeline;
  underlines: GPURenderPipeline;
  monoSprites: GPURenderPipeline;
  polySprites: GPURenderPipeline;
  subpixelSprites: GPURenderPipeline;
  pathRasterization: GPURenderPipeline;
  pathComposite: GPURenderPipeline;
}

interface BindGroupLayouts {
  globals: GPUBindGroupLayout;
  instances: GPUBindGroupLayout;
  instancesWithTexture: GPUBindGroupLayout;
}

// Data for a single managed GPU buffer that grows as needed.
interface DynamicBuffer {
  buffer: GPUBuffer;
  capacity: number;
}

export interface Batch {
  kind: PrimitiveKind;
  start: number;
  end: number;
  textureId?: AtlasTextureId;
}

export class Renderer {
  private device: GPUDevice;
  private context: GPUCanvasContext;
  private atlas: Atlas;
  private pipelines: PipelineSet;
  private layouts: BindGroupLayouts;

  private globalsBuffer: GPUBuffer;
  private gammaBuffer: GPUBuffer;
  private globalsBindGroup: GPUBindGroup;

  private instanceBuffer: DynamicBuffer;

  private canvasFormat: GPUTextureFormat;
  private viewportWidth = 0;
  private viewportHeight = 0;

  // Path intermediate texture for two-pass path rendering
  private pathIntermediateTexture: GPUTexture | null = null;

  // Placeholder texture views for when atlas textures aren't available
  private monoPlaceholderView: GPUTextureView;
  private polyPlaceholderView: GPUTextureView;

  constructor(options: RendererOptions) {
    this.device = options.device;
    this.atlas = options.atlas;

    this.canvasFormat = navigator.gpu.getPreferredCanvasFormat();
    this.context = options.canvas.getContext("webgpu") as GPUCanvasContext;
    this.context.configure({
      device: this.device,
      format: this.canvasFormat,
      alphaMode: "premultiplied",
    });

    // Create globals uniform buffers
    // GlobalParams: viewport_size(2f) + premultiplied_alpha(u32) + pad(u32) = 16 bytes
    this.globalsBuffer = this.device.createBuffer({
      label: "globals",
      size: 16,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    // GammaParams: gamma_ratios(4f) + grayscale_enhanced_contrast(f) + subpixel_enhanced_contrast(f) + pad(2f) = 32 bytes
    this.gammaBuffer = this.device.createBuffer({
      label: "gamma",
      size: 32,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    // Instance storage buffer
    this.instanceBuffer = this.createDynamicBuffer(
      "instances",
      INITIAL_BUFFER_CAPACITY,
      GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    );

    // Placeholder textures
    this.monoPlaceholderView = this.atlas.createPlaceholderView(AtlasTextureKind.Monochrome);
    this.polyPlaceholderView = this.atlas.createPlaceholderView(AtlasTextureKind.Polychrome);

    // Bind group layouts
    this.layouts = this.createBindGroupLayouts();

    // Globals bind group
    this.globalsBindGroup = this.device.createBindGroup({
      label: "globals",
      layout: this.layouts.globals,
      entries: [
        { binding: 0, resource: { buffer: this.globalsBuffer } },
        { binding: 1, resource: { buffer: this.gammaBuffer } },
      ],
    });

    // Shader module
    const shaderModule = this.device.createShaderModule({
      label: "zed-shaders",
      code: shaderSource,
    });

    // Pipelines
    this.pipelines = this.createPipelines(shaderModule);
  }

  // -----------------------------------------------------------------------
  // Public API
  // -----------------------------------------------------------------------

  resize(width: number, height: number, scaleFactor: number): void {
    const physicalWidth = Math.floor(width * scaleFactor);
    const physicalHeight = Math.floor(height * scaleFactor);

    if (physicalWidth === this.viewportWidth && physicalHeight === this.viewportHeight) {
      return;
    }

    this.viewportWidth = physicalWidth;
    this.viewportHeight = physicalHeight;

    const canvas = this.context.canvas as HTMLCanvasElement;
    canvas.width = physicalWidth;
    canvas.height = physicalHeight;

    // Recreate path intermediate texture at new size
    this.recreatePathIntermediate();
  }

  drawFrame(frame: FrameMessage): void {
    // Apply atlas deltas
    this.atlas.applyDeltas(frame.atlas_deltas);

    // Update globals
    const scaleFactor = frame.scale_factor;
    const vpWidth = frame.viewport_size.width * scaleFactor;
    const vpHeight = frame.viewport_size.height * scaleFactor;

    if (Math.floor(vpWidth) !== this.viewportWidth || Math.floor(vpHeight) !== this.viewportHeight) {
      this.resize(frame.viewport_size.width, frame.viewport_size.height, scaleFactor);
    }

    this.writeGlobals(vpWidth, vpHeight);

    const scene = frame.scene;
    const batches = this.buildBatches(scene);

    const textureView = this.context.getCurrentTexture().createView();

    const encoder = this.device.createCommandEncoder({ label: "frame" });

    const pass = encoder.beginRenderPass({
      label: "main",
      colorAttachments: [
        {
          view: textureView,
          clearValue: { r: 0, g: 0, b: 0, a: 0 },
          loadOp: "clear" as GPULoadOp,
          storeOp: "store" as GPUStoreOp,
        },
      ],
    });

    for (const batch of batches) {
      this.drawBatch(batch, scene, pass);
    }

    pass.end();
    this.device.queue.submit([encoder.finish()]);
  }

  destroy(): void {
    this.globalsBuffer.destroy();
    this.gammaBuffer.destroy();
    this.instanceBuffer.buffer.destroy();
    this.pathIntermediateTexture?.destroy();
    this.atlas.destroy();
  }

  // -----------------------------------------------------------------------
  // Bind group layouts
  // -----------------------------------------------------------------------

  private createBindGroupLayouts(): BindGroupLayouts {
    const globals = this.device.createBindGroupLayout({
      label: "globals",
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
          buffer: { type: "uniform" as GPUBufferBindingType },
        },
        {
          binding: 1,
          visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
          buffer: { type: "uniform" as GPUBufferBindingType },
        },
      ],
    });

    const instances = this.device.createBindGroupLayout({
      label: "instances",
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
          buffer: { type: "read-only-storage" as GPUBufferBindingType },
        },
      ],
    });

    const instancesWithTexture = this.device.createBindGroupLayout({
      label: "instances-with-texture",
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
          buffer: { type: "read-only-storage" as GPUBufferBindingType },
        },
        {
          binding: 1,
          visibility: GPUShaderStage.FRAGMENT,
          texture: { sampleType: "float" as GPUTextureSampleType },
        },
        {
          binding: 2,
          visibility: GPUShaderStage.FRAGMENT,
          sampler: { type: "filtering" as GPUSamplerBindingType },
        },
      ],
    });

    return { globals, instances, instancesWithTexture };
  }

  // -----------------------------------------------------------------------
  // Pipeline creation
  // -----------------------------------------------------------------------

  private createPipelines(shader: GPUShaderModule): PipelineSet {
    const blendPremultiplied: GPUBlendState = {
      color: {
        srcFactor: "one" as GPUBlendFactor,
        dstFactor: "one-minus-src-alpha" as GPUBlendFactor,
        operation: "add" as GPUBlendOperation,
      },
      alpha: {
        srcFactor: "one" as GPUBlendFactor,
        dstFactor: "one-minus-src-alpha" as GPUBlendFactor,
        operation: "add" as GPUBlendOperation,
      },
    };

    const target: GPUColorTargetState = {
      format: this.canvasFormat,
      blend: blendPremultiplied,
      writeMask: GPUColorWrite.ALL,
    };

    const makeSimplePipeline = (
      label: string,
      vs: string,
      fs: string,
      layout: GPUBindGroupLayout,
    ): GPURenderPipeline => {
      return this.device.createRenderPipeline({
        label,
        layout: this.device.createPipelineLayout({
          bindGroupLayouts: [this.layouts.globals, layout],
        }),
        vertex: {
          module: shader,
          entryPoint: vs,
        },
        fragment: {
          module: shader,
          entryPoint: fs,
          targets: [target],
        },
        primitive: {
          topology: "triangle-strip" as GPUPrimitiveTopology,
          stripIndexFormat: undefined,
        },
      });
    };

    return {
      quads: makeSimplePipeline("quads", "vs_quad", "fs_quad", this.layouts.instances),
      shadows: makeSimplePipeline("shadows", "vs_shadow", "fs_shadow", this.layouts.instances),
      underlines: makeSimplePipeline("underlines", "vs_underline", "fs_underline", this.layouts.instances),
      monoSprites: makeSimplePipeline(
        "mono-sprites",
        "vs_mono_sprite",
        "fs_mono_sprite",
        this.layouts.instancesWithTexture,
      ),
      polySprites: makeSimplePipeline(
        "poly-sprites",
        "vs_poly_sprite",
        "fs_poly_sprite",
        this.layouts.instancesWithTexture,
      ),
      subpixelSprites: makeSimplePipeline(
        "subpixel-sprites",
        "vs_subpixel_sprite",
        "fs_subpixel_sprite",
        this.layouts.instancesWithTexture,
      ),
      pathRasterization: this.device.createRenderPipeline({
        label: "path-rasterization",
        layout: this.device.createPipelineLayout({
          bindGroupLayouts: [this.layouts.globals, this.layouts.instances],
        }),
        vertex: {
          module: shader,
          entryPoint: "vs_path_rasterization",
        },
        fragment: {
          module: shader,
          entryPoint: "fs_path_rasterization",
          targets: [
            {
              format: "rgba8unorm" as GPUTextureFormat,
              blend: {
                color: {
                  srcFactor: "one" as GPUBlendFactor,
                  dstFactor: "one" as GPUBlendFactor,
                  operation: "add" as GPUBlendOperation,
                },
                alpha: {
                  srcFactor: "one" as GPUBlendFactor,
                  dstFactor: "one" as GPUBlendFactor,
                  operation: "add" as GPUBlendOperation,
                },
              },
            },
          ],
        },
        primitive: {
          topology: "triangle-list" as GPUPrimitiveTopology,
        },
      }),
      pathComposite: makeSimplePipeline("path-composite", "vs_path", "fs_path", this.layouts.instancesWithTexture),
    };
  }

  // -----------------------------------------------------------------------
  // Globals
  // -----------------------------------------------------------------------

  private writeGlobals(viewportWidth: number, viewportHeight: number): void {
    const globals = new ArrayBuffer(16);
    const view = new DataView(globals);
    view.setFloat32(0, viewportWidth, true);
    view.setFloat32(4, viewportHeight, true);
    view.setUint32(8, 1, true); // premultiplied_alpha = true for browser
    view.setUint32(12, 0, true); // pad
    this.device.queue.writeBuffer(this.globalsBuffer, 0, globals);

    // Default gamma params -- these can be tuned later
    const gamma = new ArrayBuffer(32);
    const gv = new DataView(gamma);
    // gamma_ratios: reasonable defaults for grayscale text
    gv.setFloat32(0, 0.0, true);
    gv.setFloat32(4, 0.0, true);
    gv.setFloat32(8, 0.0, true);
    gv.setFloat32(12, 0.0, true);
    gv.setFloat32(16, 0.0, true); // grayscale_enhanced_contrast
    gv.setFloat32(20, 0.0, true); // subpixel_enhanced_contrast
    gv.setFloat32(24, 0.0, true); // pad
    gv.setFloat32(28, 0.0, true); // pad
    this.device.queue.writeBuffer(this.gammaBuffer, 0, gamma);
  }

  // -----------------------------------------------------------------------
  // Path intermediate texture
  // -----------------------------------------------------------------------

  private recreatePathIntermediate(): void {
    this.pathIntermediateTexture?.destroy();
    if (this.viewportWidth === 0 || this.viewportHeight === 0) return;

    this.pathIntermediateTexture = this.device.createTexture({
      label: "path-intermediate",
      size: {
        width: this.viewportWidth,
        height: this.viewportHeight,
        depthOrArrayLayers: 1,
      },
      format: "rgba8unorm",
      usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING,
    });
    this.pathIntermediateTexture.createView();
  }

  // -----------------------------------------------------------------------
  // Buffer management
  // -----------------------------------------------------------------------

  private createDynamicBuffer(label: string, capacity: number, usage: GPUBufferUsageFlags): DynamicBuffer {
    const buffer = this.device.createBuffer({
      label,
      size: capacity,
      usage,
    });
    return { buffer, capacity };
  }

  private ensureInstanceCapacity(needed: number): void {
    if (needed <= this.instanceBuffer.capacity) return;

    this.instanceBuffer.buffer.destroy();
    let newCapacity = this.instanceBuffer.capacity;
    while (newCapacity < needed) {
      newCapacity *= 2;
    }
    this.instanceBuffer = this.createDynamicBuffer(
      "instances",
      newCapacity,
      GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    );
  }

  // -----------------------------------------------------------------------
  // Batch building (mirrors crates/gpui/src/scene.rs BatchIterator)
  // -----------------------------------------------------------------------

  private buildBatches(scene: Scene): Batch[] {
    // Each primitive list is already sorted by order. We merge them into
    // draw-order-correct batches, just like the Rust BatchIterator.

    const iterators = [
      { kind: PrimitiveKind.Shadow, items: scene.shadows, index: 0 },
      { kind: PrimitiveKind.Quad, items: scene.quads, index: 0 },
      { kind: PrimitiveKind.Path, items: scene.paths, index: 0 },
      { kind: PrimitiveKind.Underline, items: scene.underlines, index: 0 },
      { kind: PrimitiveKind.MonochromeSprite, items: scene.monochrome_sprites, index: 0 },
      { kind: PrimitiveKind.SubpixelSprite, items: scene.subpixel_sprites, index: 0 },
      { kind: PrimitiveKind.PolychromeSprite, items: scene.polychrome_sprites, index: 0 },
    ];

    const batches: Batch[] = [];

    while (true) {
      // Find the iterator with the smallest current order
      let minOrder = Infinity;
      let minKind = -1;
      let secondMinOrder = Infinity;
      let secondMinKind = -1;

      for (const it of iterators) {
        if (it.index >= it.items.length) continue;
        const order = (it.items[it.index] as { order: number }).order;
        if (order < minOrder || (order === minOrder && it.kind < minKind)) {
          secondMinOrder = minOrder;
          secondMinKind = minKind;
          minOrder = order;
          minKind = it.kind;
        } else if (order < secondMinOrder || (order === secondMinOrder && it.kind < secondMinKind)) {
          secondMinOrder = order;
          secondMinKind = it.kind;
        }
      }

      if (minKind === -1) break;

      const it = iterators.find((i) => i.kind === minKind)!;
      const batchStart = it.index;

      // For sprite types, we also batch by texture_id
      const isSprite =
        minKind === PrimitiveKind.MonochromeSprite ||
        minKind === PrimitiveKind.SubpixelSprite ||
        minKind === PrimitiveKind.PolychromeSprite;

      let textureId: AtlasTextureId | undefined;
      if (isSprite) {
        textureId = (it.items[it.index] as { tile: AtlasTile }).tile.texture_id;
      }

      // Consume items from this iterator while their order is less than the
      // second-smallest order (the next type that needs to draw).
      it.index++;
      while (it.index < it.items.length) {
        const item = it.items[it.index] as { order: number; tile?: AtlasTile };
        const itemOrder = item.order;

        if (itemOrder > secondMinOrder || (itemOrder === secondMinOrder && minKind >= secondMinKind)) {
          break;
        }

        // Sprite batches also break on texture changes
        if (isSprite && textureId) {
          const itemTex = (item as { tile: AtlasTile }).tile.texture_id;
          if (itemTex.index !== textureId.index || itemTex.kind !== textureId.kind) {
            break;
          }
        }

        it.index++;
      }

      batches.push({
        kind: minKind,
        start: batchStart,
        end: it.index,
        textureId,
      });
    }

    return batches;
  }

  // -----------------------------------------------------------------------
  // Drawing
  // -----------------------------------------------------------------------

  private drawBatch(batch: Batch, scene: Scene, pass: GPURenderPassEncoder): void {
    switch (batch.kind) {
      case PrimitiveKind.Quad:
        this.drawQuads(scene.quads, batch.start, batch.end, pass);
        break;
      case PrimitiveKind.Shadow:
        this.drawShadows(scene.shadows, batch.start, batch.end, pass);
        break;
      case PrimitiveKind.Underline:
        this.drawUnderlines(scene.underlines, batch.start, batch.end, pass);
        break;
      case PrimitiveKind.MonochromeSprite:
        this.drawMonoSprites(scene.monochrome_sprites, batch.start, batch.end, batch.textureId!, pass);
        break;
      case PrimitiveKind.SubpixelSprite:
        this.drawSubpixelSprites(scene.subpixel_sprites, batch.start, batch.end, batch.textureId!, pass);
        break;
      case PrimitiveKind.PolychromeSprite:
        this.drawPolySprites(scene.polychrome_sprites, batch.start, batch.end, batch.textureId!, pass);
        break;
      case PrimitiveKind.Path:
        // Path rendering is a two-pass process that requires ending and
        // restarting the render pass. For Phase 1 with fixture data, we
        // skip paths since they're relatively rare in typical editor UIs.
        // TODO: implement two-pass path rendering
        break;
    }
  }

  // --- Quads ---

  private drawQuads(quads: Quad[], start: number, end: number, pass: GPURenderPassEncoder): void {
    const count = end - start;
    if (count === 0) return;

    const stride = this.quadStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const quad = quads[start + i];
      const offset = i * stride;
      this.packQuad(view, offset, quad);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const bindGroup = this.device.createBindGroup({
      label: "quads",
      layout: this.layouts.instances,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
      ],
    });

    pass.setPipeline(this.pipelines.quads);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // --- Shadows ---

  private drawShadows(shadows: Shadow[], start: number, end: number, pass: GPURenderPassEncoder): void {
    const count = end - start;
    if (count === 0) return;

    const stride = this.shadowStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const shadow = shadows[start + i];
      const offset = i * stride;
      this.packShadow(view, offset, shadow);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const bindGroup = this.device.createBindGroup({
      label: "shadows",
      layout: this.layouts.instances,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
      ],
    });

    pass.setPipeline(this.pipelines.shadows);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // --- Underlines ---

  private drawUnderlines(underlines: Underline[], start: number, end: number, pass: GPURenderPassEncoder): void {
    const count = end - start;
    if (count === 0) return;

    const stride = this.underlineStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const underline = underlines[start + i];
      const offset = i * stride;
      this.packUnderline(view, offset, underline);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const bindGroup = this.device.createBindGroup({
      label: "underlines",
      layout: this.layouts.instances,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
      ],
    });

    pass.setPipeline(this.pipelines.underlines);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // --- Monochrome sprites ---

  private drawMonoSprites(
    sprites: MonochromeSprite[],
    start: number,
    end: number,
    textureId: AtlasTextureId,
    pass: GPURenderPassEncoder,
  ): void {
    const count = end - start;
    if (count === 0) return;

    const stride = this.monoSpriteStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const sprite = sprites[start + i];
      const offset = i * stride;
      this.packMonoSprite(view, offset, sprite);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const textureView = this.atlas.getTextureView(textureId) ?? this.monoPlaceholderView;

    const bindGroup = this.device.createBindGroup({
      label: "mono-sprites",
      layout: this.layouts.instancesWithTexture,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
        { binding: 1, resource: textureView },
        { binding: 2, resource: this.atlas.sampler },
      ],
    });

    pass.setPipeline(this.pipelines.monoSprites);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // --- Subpixel sprites (grayscale fallback) ---

  private drawSubpixelSprites(
    sprites: SubpixelSprite[],
    start: number,
    end: number,
    textureId: AtlasTextureId,
    pass: GPURenderPassEncoder,
  ): void {
    const count = end - start;
    if (count === 0) return;

    // SubpixelSprite has the same GPU layout as MonochromeSprite in our
    // fallback shader (both have bounds, content_mask, color, tile, transform).
    const stride = this.subpixelSpriteStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const sprite = sprites[start + i];
      const offset = i * stride;
      this.packSubpixelSprite(view, offset, sprite);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const textureView = this.atlas.getTextureView(textureId) ?? this.polyPlaceholderView;

    const bindGroup = this.device.createBindGroup({
      label: "subpixel-sprites",
      layout: this.layouts.instancesWithTexture,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
        { binding: 1, resource: textureView },
        { binding: 2, resource: this.atlas.sampler },
      ],
    });

    pass.setPipeline(this.pipelines.subpixelSprites);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // --- Polychrome sprites ---

  private drawPolySprites(
    sprites: PolychromeSprite[],
    start: number,
    end: number,
    textureId: AtlasTextureId,
    pass: GPURenderPassEncoder,
  ): void {
    const count = end - start;
    if (count === 0) return;

    const stride = this.polySpriteStride();
    const totalBytes = count * stride;
    this.ensureInstanceCapacity(totalBytes);

    const buffer = new ArrayBuffer(totalBytes);
    const view = new DataView(buffer);

    for (let i = 0; i < count; i++) {
      const sprite = sprites[start + i];
      const offset = i * stride;
      this.packPolySprite(view, offset, sprite);
    }

    this.device.queue.writeBuffer(this.instanceBuffer.buffer, 0, buffer);

    const textureView = this.atlas.getTextureView(textureId) ?? this.polyPlaceholderView;

    const bindGroup = this.device.createBindGroup({
      label: "poly-sprites",
      layout: this.layouts.instancesWithTexture,
      entries: [
        {
          binding: 0,
          resource: { buffer: this.instanceBuffer.buffer, size: totalBytes },
        },
        { binding: 1, resource: textureView },
        { binding: 2, resource: this.atlas.sampler },
      ],
    });

    pass.setPipeline(this.pipelines.polySprites);
    pass.setBindGroup(0, this.globalsBindGroup);
    pass.setBindGroup(1, bindGroup);
    pass.draw(4, count, 0, 0);
  }

  // -----------------------------------------------------------------------
  // Struct packing
  //
  // These functions write GPUI primitive structs into an ArrayBuffer at the
  // byte layout expected by the WGSL storage buffer structs. Each pack
  // function returns the number of bytes written and the corresponding stride
  // function returns the fixed byte stride for one instance.
  //
  // WGSL storage buffer layout rules (effectively std430):
  //   - scalars: natural alignment (f32=4, u32=4, i32=4)
  //   - vec2: 8-byte aligned
  //   - vec3: 16-byte aligned
  //   - vec4: 16-byte aligned
  //   - struct: aligned to its largest member
  //   - array<T, N>: element stride rounded up to alignment of T
  //
  // We carefully match the field order and padding from the WGSL source.
  // -----------------------------------------------------------------------

  // --- Bounds (16 bytes) ---
  private packBounds(view: DataView, offset: number, bounds: Bounds): number {
    view.setFloat32(offset + 0, bounds.origin.x, true);
    view.setFloat32(offset + 4, bounds.origin.y, true);
    view.setFloat32(offset + 8, bounds.size.width, true);
    view.setFloat32(offset + 12, bounds.size.height, true);
    return 16;
  }

  // --- Corners (16 bytes) ---
  private packCorners(view: DataView, offset: number, corners: Corners): number {
    view.setFloat32(offset + 0, corners.top_left, true);
    view.setFloat32(offset + 4, corners.top_right, true);
    view.setFloat32(offset + 8, corners.bottom_right, true);
    view.setFloat32(offset + 12, corners.bottom_left, true);
    return 16;
  }

  // --- Edges (16 bytes) ---
  private packEdges(view: DataView, offset: number, edges: Edges): number {
    view.setFloat32(offset + 0, edges.top, true);
    view.setFloat32(offset + 4, edges.right, true);
    view.setFloat32(offset + 8, edges.bottom, true);
    view.setFloat32(offset + 12, edges.left, true);
    return 16;
  }

  // --- Hsla (16 bytes) ---
  private packHsla(view: DataView, offset: number, color: Hsla): number {
    view.setFloat32(offset + 0, color.h, true);
    view.setFloat32(offset + 4, color.s, true);
    view.setFloat32(offset + 8, color.l, true);
    view.setFloat32(offset + 12, color.a, true);
    return 16;
  }

  // --- Background ---
  // WGSL struct Background:
  //   tag: u32           (offset 0, 4 bytes)
  //   color_space: u32   (offset 4, 4 bytes)
  //   solid: Hsla        (offset 8? No, Hsla has align 16, so offset 16)
  //   -- actually, struct members in WGSL pack sequentially with alignment.
  //   -- u32 at 0, u32 at 4, then Hsla (align 16) starts at 16.
  //   -- Wait, in WGSL with storage buffers Hsla = struct{f32,f32,f32,f32} has align 4 actually.
  //   -- No: in std430 layout (storage buffers), struct alignment = max of member alignments.
  //   -- Hsla members are all f32 (align 4), so Hsla align = 4, size = 16.
  //   -- So: tag at 0, color_space at 4, solid at 8 (align 4, ok), size 16, ends at 24.
  //   gradient_angle_or_pattern_height: f32 (offset 24, 4 bytes)
  //   colors: array<LinearColorStop, 2>
  //     LinearColorStop = {Hsla(align4, size16), f32(4)} => align=4, size=20.
  //     Array stride for std430: roundUp(20, align=4) = 20. Hmm, but we used 32 above...
  //     Actually in WGSL, array element stride is roundUp(element size, element alignment).
  //     LinearColorStop alignment = max(Hsla align, f32 align) = 4. Size = 20. Stride = 20.
  //     But wait, I need to double-check against what the actual WGSL compiler does.
  //     The Rust side uses repr(C) which may differ...
  //
  // OK -- the safest approach for Phase 1 is to match exactly what Naga/wgpu produces
  // for storage buffer layout. Let me look at the sizes from the Rust side:
  //   Rust Hsla: 4 f32s = 16 bytes, align 4
  //   Rust LinearColorStop: Hsla(16) + f32(4) = 20 bytes
  //   Rust Background: u32(4) + u32(4) + Hsla(16) + f32(4) + [LinearColorStop; 2](40) + u32(4) = 72 bytes
  //
  // In WGSL std430 (storage buffer) layout:
  //   offset 0: tag (u32, 4)
  //   offset 4: color_space (u32, 4)
  //   offset 8: solid (Hsla = 4xf32, 16)
  //   offset 24: gradient_angle_or_pattern_height (f32, 4)
  //   offset 28: colors[0] (LinearColorStop)
  //     offset 28: color (Hsla, 16)
  //     offset 44: percentage (f32, 4)
  //   offset 48: colors[1] (LinearColorStop)
  //     offset 48: color (Hsla, 16)
  //     offset 64: percentage (f32, 4)
  //   offset 68: pad (u32, 4)
  //   total = 72
  //
  // But struct size must be rounded to struct alignment. Struct alignment = 4.
  // So total size = 72.

  private packBackground(view: DataView, offset: number, bg: Background): number {
    view.setUint32(offset + 0, bg.tag, true);
    view.setUint32(offset + 4, bg.color_space, true);
    this.packHsla(view, offset + 8, bg.solid);
    view.setFloat32(offset + 24, bg.gradient_angle_or_pattern_height, true);
    // colors[0]
    this.packHsla(view, offset + 28, bg.colors[0].color);
    view.setFloat32(offset + 44, bg.colors[0].percentage, true);
    // colors[1]
    this.packHsla(view, offset + 48, bg.colors[1].color);
    view.setFloat32(offset + 64, bg.colors[1].percentage, true);
    // pad
    view.setUint32(offset + 68, 0, true);
    return 72;
  }

  // --- AtlasTile (32 bytes) ---
  // AtlasTextureId: index(u32, 4) + kind(u32, 4) = 8
  // AtlasBounds: origin(vec2<i32>, 8) + size(vec2<i32>, 8) = 16, align 8
  // AtlasTile: texture_id(8) + tile_id(u32, 4) + padding(u32, 4) + bounds(align 8, 16) = 32
  private packAtlasTile(view: DataView, offset: number, tile: AtlasTile): number {
    view.setUint32(offset + 0, tile.texture_id.index, true);
    view.setUint32(offset + 4, tile.texture_id.kind, true);
    view.setUint32(offset + 8, tile.tile_id, true);
    view.setUint32(offset + 12, tile.padding, true);
    view.setInt32(offset + 16, tile.bounds.origin.x, true);
    view.setInt32(offset + 20, tile.bounds.origin.y, true);
    view.setInt32(offset + 24, tile.bounds.size.width, true);
    view.setInt32(offset + 28, tile.bounds.size.height, true);
    return 32;
  }

  // --- TransformationMatrix (24 bytes) ---
  // rotation_scale: mat2x2<f32> = 4xf32 = 16 bytes, align 8
  // translation: vec2<f32> = 8 bytes, align 8
  // Total: 24 bytes. Struct align = 8, so size = 24.
  private packTransformationMatrix(view: DataView, offset: number, xform: TransformationMatrix): number {
    // WGSL mat2x2 is stored as 2 column vectors (column-major)
    // But the Rust side stores row-major and the shader transposes.
    // So we write it row-major here (same as Rust) and the shader transposes.
    view.setFloat32(offset + 0, xform.rotation_scale[0][0], true);
    view.setFloat32(offset + 4, xform.rotation_scale[0][1], true);
    view.setFloat32(offset + 8, xform.rotation_scale[1][0], true);
    view.setFloat32(offset + 12, xform.rotation_scale[1][1], true);
    view.setFloat32(offset + 16, xform.translation[0], true);
    view.setFloat32(offset + 20, xform.translation[1], true);
    return 24;
  }

  // --- Quad ---
  // WGSL struct Quad:
  //   order: u32 (4)
  //   border_style: u32 (4)
  //   bounds: Bounds (16)
  //   content_mask: Bounds (16)
  //   background: Background (72)
  //   border_color: Hsla (16)
  //   corner_radii: Corners (16)
  //   border_widths: Edges (16)
  //   Total: 4+4+16+16+72+16+16+16 = 160

  private quadStride(): number {
    return 160;
  }

  private packQuad(view: DataView, offset: number, quad: Quad): void {
    let o = offset;
    view.setUint32(o, quad.order, true);
    o += 4;
    view.setUint32(o, quad.border_style, true);
    o += 4;
    o += this.packBounds(view, o, quad.bounds);
    o += this.packBounds(view, o, quad.content_mask.bounds);
    o += this.packBackground(view, o, quad.background);
    o += this.packHsla(view, o, quad.border_color);
    o += this.packCorners(view, o, quad.corner_radii);
    this.packEdges(view, o, quad.border_widths);
  }

  // --- Shadow ---
  // WGSL struct Shadow:
  //   order: u32 (4)
  //   blur_radius: f32 (4)
  //   bounds: Bounds (16)
  //   corner_radii: Corners (16)
  //   content_mask: Bounds (16)
  //   color: Hsla (16)
  //   Total: 4+4+16+16+16+16 = 72

  private shadowStride(): number {
    return 72;
  }

  private packShadow(view: DataView, offset: number, shadow: Shadow): void {
    let o = offset;
    view.setUint32(o, shadow.order, true);
    o += 4;
    view.setFloat32(o, shadow.blur_radius, true);
    o += 4;
    o += this.packBounds(view, o, shadow.bounds);
    o += this.packCorners(view, o, shadow.corner_radii);
    o += this.packBounds(view, o, shadow.content_mask.bounds);
    this.packHsla(view, o, shadow.color);
  }

  // --- Underline ---
  // WGSL struct Underline:
  //   order: u32 (4)
  //   pad: u32 (4)
  //   bounds: Bounds (16)
  //   content_mask: Bounds (16)
  //   color: Hsla (16)
  //   thickness: f32 (4)
  //   wavy: u32 (4)
  //   Total: 4+4+16+16+16+4+4 = 64

  private underlineStride(): number {
    return 64;
  }

  private packUnderline(view: DataView, offset: number, underline: Underline): void {
    let o = offset;
    view.setUint32(o, underline.order, true);
    o += 4;
    view.setUint32(o, 0, true);
    o += 4; // pad
    o += this.packBounds(view, o, underline.bounds);
    o += this.packBounds(view, o, underline.content_mask.bounds);
    o += this.packHsla(view, o, underline.color);
    view.setFloat32(o, underline.thickness, true);
    o += 4;
    view.setUint32(o, underline.wavy ? 1 : 0, true);
  }

  // --- MonochromeSprite ---
  // WGSL struct MonochromeSprite:
  //   order: u32 (4)
  //   pad: u32 (4)
  //   bounds: Bounds (16)
  //   content_mask: Bounds (16)
  //   color: Hsla (16)
  //   tile: AtlasTile (32)
  //   transformation: TransformationMatrix (24)
  //   Total: 4+4+16+16+16+32+24 = 112
  //   But TransformationMatrix has align 8 (mat2x2). 4+4+16+16+16 = 56, AtlasTile ends at 56+32=88.
  //   88 is already 8-byte aligned, so TransformationMatrix starts at 88.
  //   Total = 88 + 24 = 112.

  private monoSpriteStride(): number {
    return 112;
  }

  private packMonoSprite(view: DataView, offset: number, sprite: MonochromeSprite): void {
    let o = offset;
    view.setUint32(o, sprite.order, true);
    o += 4;
    view.setUint32(o, 0, true);
    o += 4; // pad
    o += this.packBounds(view, o, sprite.bounds);
    o += this.packBounds(view, o, sprite.content_mask.bounds);
    o += this.packHsla(view, o, sprite.color);
    o += this.packAtlasTile(view, o, sprite.tile);
    this.packTransformationMatrix(view, o, sprite.transformation);
  }

  // --- SubpixelSprite (same layout as MonochromeSprite) ---
  private subpixelSpriteStride(): number {
    return 112;
  }

  private packSubpixelSprite(view: DataView, offset: number, sprite: SubpixelSprite): void {
    let o = offset;
    view.setUint32(o, sprite.order, true);
    o += 4;
    view.setUint32(o, 0, true);
    o += 4; // pad
    o += this.packBounds(view, o, sprite.bounds);
    o += this.packBounds(view, o, sprite.content_mask.bounds);
    o += this.packHsla(view, o, sprite.color);
    o += this.packAtlasTile(view, o, sprite.tile);
    this.packTransformationMatrix(view, o, sprite.transformation);
  }

  // --- PolychromeSprite ---
  // WGSL struct PolychromeSprite:
  //   order: u32 (4)
  //   pad: u32 (4)
  //   grayscale: u32 (4)
  //   opacity: f32 (4)
  //   bounds: Bounds (16)
  //   content_mask: Bounds (16)
  //   corner_radii: Corners (16)
  //   tile: AtlasTile (32)
  //   Total: 4+4+4+4+16+16+16+32 = 96

  private polySpriteStride(): number {
    return 96;
  }

  private packPolySprite(view: DataView, offset: number, sprite: PolychromeSprite): void {
    let o = offset;
    view.setUint32(o, sprite.order, true);
    o += 4;
    view.setUint32(o, 0, true);
    o += 4; // pad
    view.setUint32(o, sprite.grayscale ? 1 : 0, true);
    o += 4;
    view.setFloat32(o, sprite.opacity, true);
    o += 4;
    o += this.packBounds(view, o, sprite.bounds);
    o += this.packBounds(view, o, sprite.content_mask.bounds);
    o += this.packCorners(view, o, sprite.corner_radii);
    this.packAtlasTile(view, o, sprite.tile);
  }
}
