//! Product/version metadata shared by shell, core, resources and diagnostics.

/// Human-readable browser product name.
pub const PRODUCT_NAME: &str = "Nautile Navigateur";

/// Workspace package version compiled into Nautile binaries.
pub const NAUTILE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Browser version string suitable for window titles and dumps.
pub fn browser_version_string() -> String {
    format!("{PRODUCT_NAME} {NAUTILE_VERSION}")
}

/// Initial user-agent product token used until the network stack owns UA policy.
pub fn user_agent_product() -> String {
    format!("Nautile/{}", NAUTILE_VERSION)
}
