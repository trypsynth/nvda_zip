#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use reqwest::Client;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

const CACHE_TTL: Duration = Duration::from_secs(30);

/// Direct download link for NVDA 2017.3 (Windows XP).
pub const XP_URL: &str = "https://download.nvaccess.org/releases/2017.3/nvda_2017.3.exe";
/// Direct download link for NVDA 2023.3.4 (Windows 7).
pub const WIN7_URL: &str = "https://download.nvaccess.org/releases/2023.3.4/nvda_2023.3.4.exe";

/// NV Access has their own custom format for NVDA's update API, this lets us parse only the fields we care about out of it.
#[derive(Debug)]
struct UpdateInfo {
    pub launcher_url: Option<String>,
}

impl UpdateInfo {
    #[must_use]
    fn parse(data: &str) -> Self {
        let mut launcher_url = None;
        for line in data.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                if key == "launcherUrl" {
                    launcher_url = Some(value.to_string());
                }
            }
        }
        Self {
            launcher_url,
        }
    }
}

/// Represents the different NVDA release channels.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum VersionType {
    /// Official stable releases.
    Stable,
    /// Pre-release beta versions.
    Beta,
    /// Snapshot alpha builds.
    Alpha,
}

impl VersionType {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Alpha => "snapshot:alpha",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }
}

/// Fetches and caches NVDA download URLs.
#[derive(Default)]
pub struct NvdaUrl {
    client: Client,
    cache: Mutex<HashMap<VersionType, (String, Instant)>>,
}

impl NvdaUrl {
    /// Retrieves the latest download URL for the specified NVDA version type.
    ///
    /// If a cached URL is still valid, it is returned. Otherwise, a new request is made.
    ///
    /// # Arguments
    ///
    /// * `version_type` - The type of NVDA version to fetch.
    ///  
    /// # Returns
    ///
    /// An `Option<String>` containing the URL if successful, or `None` if an error occurs.
    pub async fn get_url(&self, version_type: VersionType) -> Option<String> {
        let mut cache = self.cache.lock().await;
        if let Some((url, timestamp)) = cache.get(&version_type) {
            if timestamp.elapsed() < CACHE_TTL {
                return Some(url.clone());
            }
        }
        let url = self.fetch_url(&version_type).await?;
        cache.insert(version_type, (url.clone(), Instant::now()));
        drop(cache);
        Some(url)
    }

    async fn fetch_url(&self, version_type: &VersionType) -> Option<String> {
        let url = format!(
            "https://api.nvaccess.org/nvdaUpdateCheck?versionType={}",
            version_type.as_str()
        );
        let body = self.client.get(&url).send().await.ok()?.text().await.ok()?;
        let info = UpdateInfo::parse(&body);
        info.launcher_url
    }
}
