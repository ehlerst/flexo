// Fallback strategy in case the JSON endpoint cannot be reached: The selected mirrors are stored in a text file
// so that we can simply retrieve and reuse the previously selected mirrors from this file, instead of fetching
// the mirrors from the JSON endpoint.

use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::mirror_config::MirrorConfig;
use crate::mirror_flexo::DownloadProvider;
use crate::fs_utils::create_dir_unless_exists;

// Bump this version if a non-backwards compatible change has occurred.
const TIMESTAMPED_DOWNLOAD_PROVIDERS_VERSION: u32 = 3;

#[derive(Deserialize, Serialize)]
pub struct TimestampedDownloadProviders {
    pub version: Option<u32>,
    pub timestamp: String,
    pub download_providers: Vec<DownloadProvider>,
}

#[derive(Deserialize, Serialize)]
pub struct VersionOnly {
    pub version: Option<u32>,
}

fn latency_test_results_file(properties: &MirrorConfig) -> String {
    match &properties.mirrorlist_latency_test_results_file {
        None => {
            let cache_dir = Path::new(&properties.cache_directory);
            // If cache_directory is /var/cache/flexo/pkg, we want to store it in /var/cache/flexo/state
            let state_dir = cache_dir.parent().unwrap_or(cache_dir).join("state");
            state_dir.join("latency_test_results.json").to_str().unwrap().to_owned()
        }
        Some(p) => p.clone(),
    }
}

pub fn store_latency_test_results(
    properties: &MirrorConfig,
    download_providers: Vec<DownloadProvider>
) -> io::Result<Vec<DownloadProvider>> {
    let timestamped = TimestampedDownloadProviders {
        version: Some(TIMESTAMPED_DOWNLOAD_PROVIDERS_VERSION),
        timestamp: format!("{:?}", chrono::Utc::now()),
        download_providers,
    };
    let serialized = serde_json::to_string_pretty(&timestamped).unwrap();
    let file_path = latency_test_results_file(properties);
    let path = Path::new(&file_path);
    if let Some(directory) = path.parent() {
        create_dir_unless_exists(directory)?;
    }
    fs::write(&file_path, serialized)?;

    // Return the providers so that ownership is given back to the caller. This way, we can avoid
    // copying the providers.
    Ok(timestamped.download_providers)
}

#[derive(Debug)]
pub enum DemarshallError {
    VersionMismatch,
    SerdeError(serde_json::Error),
    IoError(io::Error),
}

impl From<io::Error> for DemarshallError {
    fn from(e: io::Error) -> Self {
        DemarshallError::IoError(e)
    }
}

impl From<serde_json::Error> for DemarshallError {
    fn from(e: serde_json::Error) -> Self {
        DemarshallError::SerdeError(e)
    }
}

pub fn fetch_download_providers(
    properties: &MirrorConfig
) -> Result<TimestampedDownloadProviders, DemarshallError> {
    let file_path = latency_test_results_file(properties);
    let contents = fs::read_to_string(file_path)?;
    match serde_json::from_str::<VersionOnly>(&contents)? {
        VersionOnly { version: Some(TIMESTAMPED_DOWNLOAD_PROVIDERS_VERSION) } => {
            Ok(serde_json::from_str::<TimestampedDownloadProviders>(&contents)?)
        },
        _ => Err(DemarshallError::VersionMismatch),

    }
}
