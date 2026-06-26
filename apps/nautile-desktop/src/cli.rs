use nautile_common::version;

/// Desktop command-line options.
#[derive(Debug, Clone)]
pub struct Cli {
    pub url: String,
    pub print_version: bool,
}
impl Cli {
    pub fn parse() -> Self {
        let mut url = "about:blank".to_string();
        let mut print_version = false;
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--version" | "-V" => print_version = true,
                "--url" => {
                    if let Some(value) = args.next() {
                        url = value;
                    }
                }
                other if !other.starts_with('-') => url = other.into(),
                _ => {}
            }
        }
        Self { url, print_version }
    }

    pub fn version_text() -> String {
        format!(
            "{} ({})",
            version::browser_version_string(),
            version::user_agent_product()
        )
    }
}
