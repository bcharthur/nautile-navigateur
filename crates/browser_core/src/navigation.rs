#[derive(Debug, Clone, PartialEq)]
pub enum NavigationState {
    Idle,
    Started { url: String },
    Redirected { from: String, to: String },
    ResponseStarted,
    Committed,
    Failed { error: String },
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct NavigationRequest {
    pub url: String,
    pub method: HttpMethod,
    pub referrer: Option<String>,
    pub user_gesture: bool,
    pub target_frame: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod { Get, Post, Put, Delete, Head, Options, Patch }
