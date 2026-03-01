# Wireframe Rendering in Bevy - Thought Experiment

## Current State

Bevy 0.18 does **not** have native wireframe rendering. We currently simulate it by:
- Setting material base color to transparent
- Adding emissive glow to edges

This is a hack and doesn't show true wireframe (triangle edges).

## What is Wireframe Rendering?

Wireframe shows the **edges of triangles** rather than filled polygons. Each triangle has 3 edges, and in wireframe mode, we draw lines for each edge instead of filling the triangle.

## Implementation Approaches

### Option 1: Render Pass Modification (Most Flexible)

**Concept**: Add a new render pass that draws lines instead of triangles.

**How it works**:
1. Extract mesh vertex/index data
2. Generate line list from triangle indices (0→1, 1→2, 2→0 per triangle)
3. Render as `PrimitiveTopology::LineList` instead of `TriangleList`
4. Use a simple line shader

**Pros**:
- True wireframe
- Works with any mesh
- Can toggle per-mesh

**Cons**:
- Requires modifying Bevy's render pipeline
- Needs new shaders
- Potential performance overhead

**Code sketch**:
```rust
// In render pass
let line_indices: Vec<u32> = triangle_indices
    .chunks(3)
    .flat_map(|tri| vec![tri[0], tri[1], tri[1], tri[2], tri[2], tri[0]])
    .collect();

// Render with line topology
render_pass.set_pipeline(line_pipeline);
render_pass.draw_indexed(0..line_indices.len() as u32, 0, 0..1);
```

---

### Option 2: Geometry Shader Approach (GPU-side)

**Concept**: Use a geometry shader to convert triangles to lines on the GPU.

**How it works**:
1. Render triangles normally
2. Geometry shader takes each triangle, outputs 3 lines
3. Fragment shader draws lines

**Pros**:
- No CPU-side index generation
- Fast
- Dynamic

**Cons**:
- Requires geometry shader support (not all GPUs)
- More complex shader code
- Bevy's shader system doesn't expose this easily

---

### Option 3: Post-Process Edge Detection (Sobel Filter)

**Concept**: Use screen-space edge detection on the depth/normal buffer.

**How it works**:
1. Render scene normally to depth/normal texture
2. Post-process with Sobel filter to detect edges
3. Overlay lines on final image

**Pros**:
- No mesh modification needed
- Works with all geometry
- Can control line thickness

**Cons**:
- Not true geometric wireframe
- May miss thin geometry
- Requires extra render pass
- Edge artifacts

---

### Option 4: Bevy Plugin (Recommended for Contribution)

**Concept**: Create a `bevy_wireframe` plugin as a crate.

**How it works**:
1. Add `Wireframe` component
2. System extracts meshes marked with component
3. Generate line geometry in extract phase
4. Custom render node draws lines

**API Design**:
```rust
// User API
commands.spawn((
    Mesh3d(mesh),
    Wireframe::default(),  // Enables wireframe for this mesh
    WireframeColor(Color::GREEN),  // Optional color
));

// Or global toggle
app.insert_resource(WireframeConfig {
    global: true,
    color: Color::RED,
});
```

---

## Technical Challenges

### 1. Depth Fighting
Wireframe lines at triangle edges will z-fight with the solid mesh.

**Solutions**:
- Offset lines slightly (polygon offset)
- Don't render solid mesh (wireframe only)
- Use depth bias in pipeline

### 2. Line Thickness
WebGPU/wgpu has limited line width support (often only 1px).

**Solutions**:
- Render quads instead of lines (billboarded)
- Use compute shader to expand lines
- Accept 1px lines

### 3. Performance
Generating line indices every frame is expensive.

**Solutions**:
- Cache line index buffers
- Only regenerate when mesh changes
- Use GPU-side generation

### 4. Skinning/Animation
Animated meshes need wireframe to follow animation.

**Solutions**:
- Apply same skinning to line vertices
- Use transform feedback
- Generate after skinning in shader

---

## Contribution Path to Bevy

### Phase 1: External Crate
Create `bevy_wireframe` as a third-party plugin.
- Prove the API
- Gather community feedback
- Iterate on design

### Phase 2: Official Example
Add to Bevy's official examples.
- Shows best practices
- Easier to discover

### Phase 3: Core Integration
Propose RFC for native wireframe support.
- Core team review
- Integration into `bevy_pbr`
- Built-in component

---

## Prior Art

- **Unity**: `GL.wireframe` toggle (immediate mode, deprecated)
- **Unreal**: Wireframe view mode in editor
- **Godot**: `SurfaceMaterial.wireframe = true`
- **Three.js**: `wireframe: true` in material
- **bevy_wireframe crate**: Already exists! (community implementation)

---

## Recommendation

**For immediate use**: Check if `bevy_wireframe` crate exists and works with 0.18.

**For contribution**: Start with Option 4 (plugin approach) to prove the concept, then propose for core integration.

---

## References

- Bevy Render Graph: https://bevyengine.org/learn/book/rendering/render-graph/
- wgpu Line Rendering: https://github.com/gfx-rs/wgpu/issues/2812
- GPU Gems 2 - Fast Wireframe: https://developer.nvidia.com/gpugems/gpugems2/part_i_-_geometric_complexity/chapter_4_fast_wi
