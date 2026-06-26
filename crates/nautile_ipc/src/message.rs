/// Message types exchanged between browser, renderer, network, GPU, storage, and DevTools processes.
#[derive(Debug, Clone)]
pub enum IpcMessage {
    Navigate,
    CommitNavigation,
    InputEvent,
    FetchRequest,
    FetchResponse,
    SubmitCompositorFrame,
    ConsoleMessage,
    PermissionRequest,
    Shutdown,
}
