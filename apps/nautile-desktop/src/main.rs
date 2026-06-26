mod app;
mod cli;
mod window;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    nautile_common::logging::init_logging();
    app::run(cli::Cli::parse())
}
