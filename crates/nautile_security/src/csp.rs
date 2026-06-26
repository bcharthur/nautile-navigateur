/// Parsed Content Security Policy.
#[derive(Debug, Clone, Default)]
pub struct ContentSecurityPolicy {
    pub directives: Vec<String>,
}
