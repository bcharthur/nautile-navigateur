
#[derive(Debug, Clone)]
pub enum IpcMessage {
    // Browser -> Renderer
    Navigate { url: String, frame_id: u32 },
    CommitNavigation { frame_id: u32 },
    Input(InputEvent),
    ResizeViewport { width: u32, height: u32 },
    SetVisibility(bool),
    Shutdown,

    // Renderer -> Browser
    DidCommitNavigation { url: String, title: String },
    DidFirstPaint,
    RequestResource { url: String, request_id: u32 },
    ConsoleMessage { level: ConsoleLevel, text: String },
    ShowPermissionPrompt { permission: String, origin: String },
    OpenPopup { url: String },
    CrashReport { reason: String },

    // Renderer -> GPU
    SubmitFrame { frame_id: u64 },

    // Network responses
    ResourceResponse { request_id: u32, status: u16, body: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub kind: InputKind,
    pub x: f32,
    pub y: f32,
    pub modifiers: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum InputKind { MouseMove, MouseDown, MouseUp, Click, KeyDown, KeyUp, Scroll }

#[derive(Debug, Clone, Copy)]
pub enum ConsoleLevel { Log, Info, Warn, Error, Debug }
