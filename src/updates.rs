//! Self-update from GitHub releases, through fastframe-update.
//!
//! The crate checks for a newer release, refuses package-managed copies,
//! downloads and verifies the update against the publisher signature, and
//! hands it to a helper that installs it and rolls back if it does not start.
//! The Windows visual edition uses its own repository and publisher key;
//! official upstream packages cannot replace the custom appearance.

pub use fastframe_update::{
    CHECK_INTERVAL, DownloadState, Installation, Kind, Prepared, Release, Source, Unsupported,
    Updater,
};
use fastframe_update::{MacConfig, ReqwestTransport, UpdateConfig};

/// ZapFast's releases and the names its installations have had.
pub const CONFIG: UpdateConfig = UpdateConfig {
    // Cask and bundle names from before the rename. This also accepts
    // fastsapp-* marker files and `fastsapp <version>` answers, which no
    // release produces.
    legacy_names: &["fastsapp"],
    macos: MacConfig {
        bundle_ids: &["me.paolino.fastsapp"],
        executable_names: &[],
        legacy_bundle_names: &["FastsApp.app"],
    },
    publisher_key: Some(if cfg!(windows) {
        include_str!("../assets/windows11-update-public-key.hex")
    } else {
        include_str!("../assets/update-public-key.hex")
    }),
    // The next release key, backed up outside GitHub. Releases stay signed
    // with the current key until installs trust this one too.
    additional_publisher_keys: if cfg!(windows) {
        &[]
    } else {
        &[include_str!("../assets/update-public-key-next.hex")]
    },
    ..UpdateConfig::new(
        if cfg!(windows) {
            "walmirramos/zapfast"
        } else {
            "crmne/zapfast"
        },
        "ZapFast",
        "zapfast",
        env!("CARGO_PKG_VERSION"),
    )
};

/// An updater on the proxy-aware reqwest client.
pub fn updater() -> anyhow::Result<Updater> {
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(proxy) = crate::proxy::reqwest_proxy() {
        builder = builder.proxy(proxy);
    }
    Ok(Updater::new(CONFIG, ReqwestTransport::new(builder)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_config_is_valid() {
        CONFIG.validate().unwrap();
        assert_eq!(CONFIG.current_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(CONFIG.slug, "zapfast");
    }

    #[test]
    fn the_updater_starts_on_github() {
        assert!(updater().unwrap().source().is_github());
    }
}
