//! GPU backend boundary built around wgpu.
pub mod context;
pub mod glyph_atlas;
pub mod pipeline;
pub mod renderer;
pub mod shaders;
pub mod surface;
pub mod texture;
/// Marker for the wgpu backend.
#[derive(Debug, Default)]
pub struct GpuBackend;
