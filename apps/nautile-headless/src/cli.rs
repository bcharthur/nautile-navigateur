/// Headless command-line options.
#[derive(Debug, Clone)]
pub struct Cli {
    pub url: String,
    pub dump: bool,
}
impl Cli {
    pub fn parse() -> Self {
        let mut url = "about:blank".to_string();
        let mut dump = false;
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--url" => {
                    if let Some(v) = args.next() {
                        url = v
                    }
                }
                "--dump" | "--dump-state" => dump = true,
                other if !other.starts_with('-') => url = other.into(),
                _ => {}
            }
        }
        Self { url, dump }
    }
}
