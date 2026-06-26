/// Desktop command-line options.
#[derive(Debug, Clone)]
pub struct Cli {
    pub url: String,
}
impl Cli {
    pub fn parse() -> Self {
        let url = std::env::args()
            .nth(1)
            .unwrap_or_else(|| "about:blank".into());
        Self { url }
    }
}
