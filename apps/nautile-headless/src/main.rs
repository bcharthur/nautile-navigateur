mod cli;
use nautile_browser_core::{Browser, BrowserConfig};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let mut browser = Browser::new(BrowserConfig {
        headless: true,
        ..Default::default()
    });
    let tab = browser.create_tab();
    browser.navigate_tab(tab, cli.url)?;
    let dump = browser.dump_state();
    if cli.dump {
        println!("{}", dump.product);
        for tab in dump.tabs {
            println!("{tab}");
        }
    } else {
        println!("loaded {}", browser.tabs[0].current_url);
    }
    Ok(())
}
