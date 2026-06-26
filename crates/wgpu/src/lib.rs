#[derive(Debug, PartialEq, Eq)]
pub enum SurfaceError {
    Lost,
    OutOfMemory,
    Other,
}
impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "surface error")
    }
}
impl std::error::Error for SurfaceError {}
#[derive(Default)]
pub struct Instance;
impl Instance {
    pub fn create_surface<'w>(
        &self,
        _: &'w impl std::fmt::Debug,
    ) -> Result<Surface<'w>, Box<dyn std::error::Error>> {
        Ok(Surface(std::marker::PhantomData))
    }
}
pub struct Surface<'w>(std::marker::PhantomData<&'w ()>);
pub struct Adapter;
pub struct Device;
pub struct Queue;
pub struct Texture;
pub struct TextureView;
pub struct SurfaceTexture {
    pub texture: Texture,
}
impl SurfaceTexture {
    pub fn present(self) {}
}
pub struct RequestAdapterOptions<'a> {
    pub power_preference: PowerPreference,
    pub compatible_surface: Option<&'a Surface<'a>>,
    pub force_fallback_adapter: bool,
}
pub enum PowerPreference {
    HighPerformance,
}
pub struct DeviceDescriptor<'a> {
    pub label: Option<&'a str>,
    pub required_features: Features,
    pub required_limits: Limits,
}
#[derive(Default)]
pub struct Features;
impl Features {
    pub fn empty() -> Self {
        Self
    }
}
#[derive(Default)]
pub struct Limits;
impl Instance {
    pub fn request_adapter(&self, _: &RequestAdapterOptions<'_>) -> Option<Adapter> {
        Some(Adapter)
    }
}
impl Adapter {
    pub fn request_device(
        &self,
        _: &DeviceDescriptor<'_>,
        _: Option<()>,
    ) -> Result<(Device, Queue), Box<dyn std::error::Error>> {
        Ok((Device, Queue))
    }
}
#[derive(Clone, Copy)]
pub struct TextureFormat;
impl TextureFormat {
    pub fn is_srgb(&self) -> bool {
        true
    }
}
pub struct SurfaceCapabilities {
    pub formats: Vec<TextureFormat>,
    pub present_modes: Vec<PresentMode>,
    pub alpha_modes: Vec<CompositeAlphaMode>,
}
#[derive(Clone, Copy)]
pub struct PresentMode;
#[derive(Clone, Copy)]
pub struct CompositeAlphaMode;
pub struct TextureUsages;
impl TextureUsages {
    pub const RENDER_ATTACHMENT: Self = Self;
}
pub struct SurfaceConfiguration {
    pub usage: TextureUsages,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentMode,
    pub alpha_mode: CompositeAlphaMode,
    pub view_formats: Vec<TextureFormat>,
    pub desired_maximum_frame_latency: u32,
}
impl<'w> Surface<'w> {
    pub fn get_capabilities(&self, _: &Adapter) -> SurfaceCapabilities {
        SurfaceCapabilities {
            formats: vec![TextureFormat],
            present_modes: vec![PresentMode],
            alpha_modes: vec![CompositeAlphaMode],
        }
    }
    pub fn configure(&self, _: &Device, _: &SurfaceConfiguration) {}
    pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        Ok(SurfaceTexture { texture: Texture })
    }
}
pub struct TextureViewDescriptor;
impl Default for TextureViewDescriptor {
    fn default() -> Self {
        Self
    }
}
impl Texture {
    pub fn create_view(&self, _: &TextureViewDescriptor) -> TextureView {
        TextureView
    }
}
pub struct CommandEncoder;
pub struct CommandEncoderDescriptor<'a> {
    pub label: Option<&'a str>,
}
impl Device {
    pub fn create_command_encoder(&self, _: &CommandEncoderDescriptor<'_>) -> CommandEncoder {
        CommandEncoder
    }
}
pub struct CommandBuffer;
impl CommandEncoder {
    pub fn finish(self) -> CommandBuffer {
        CommandBuffer
    }
    pub fn begin_render_pass<'a>(&'a mut self, _: &RenderPassDescriptor<'a>) -> RenderPass<'a> {
        RenderPass(std::marker::PhantomData)
    }
}
pub struct RenderPass<'a>(std::marker::PhantomData<&'a ()>);
impl Queue {
    pub fn submit(&self, _: Option<CommandBuffer>) {}
}
pub struct RenderPassDescriptor<'a> {
    pub label: Option<&'a str>,
    pub color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    pub depth_stencil_attachment: Option<()>,
    pub occlusion_query_set: Option<()>,
    pub timestamp_writes: Option<()>,
}
pub struct RenderPassColorAttachment<'a> {
    pub view: &'a TextureView,
    pub resolve_target: Option<&'a TextureView>,
    pub ops: Operations,
}
pub struct Operations {
    pub load: LoadOp,
    pub store: StoreOp,
}
pub enum LoadOp {
    Clear(Color),
}
pub enum StoreOp {
    Store,
}
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
