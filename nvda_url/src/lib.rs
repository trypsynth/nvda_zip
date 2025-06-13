#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use regex::Regex;
use reqwest::Client;
use std::{
    collections::HashMap,
    sync::LazyLock,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

const CACHE_TTL: Duration = Duration::from_secs(30);

/// Direct download link for NVDA 2017.3 (Windows XP).
pub const XP_URL: &str =
    "https://download.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe";
/// Direct download link for NVDA 2023.3.4 (Windows 7).
pub const WIN7_URL: &str =
    "https://download.nvaccess.org/download/nvda/releases/2023.3.4/nvda_2023.3.4.exe";

/// Represents the different NVDA release channels.
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum VersionType {
    /// Official stable releases.
    Stable,
    /// Pre-release beta versions.
    Beta,
    /// Snapshot alpha builds.
    Alpha,
}

impl VersionType {
    const fn api_param(&self) -> &'static str {
        match self {
            Self::Alpha => "snapshot:alpha",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    fn regex(&self) -> &'static Regex {
        static ALPHA: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"launcherUrl:\s*(\S+)").unwrap());
        static VERSION: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"version:\s*(\S+)").unwrap());
        match self {
            Self::Alpha => &ALPHA,
            Self::Beta | Self::Stable => &VERSION,
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
            "https://download.nvaccess.org/nvdaUpdateCheck?versionType={}",
            version_type.api_param()
        );
        let body = self.client.get(&url).send().await.ok()?.text().await.ok()?;
        let version = version_type.regex().captures(&body)?.get(1)?.as_str();
        Some(match version_type {
            VersionType::Alpha => version.to_owned(),
            _ => format!(
                "https://download.nvaccess.org/download/nvda/releases/{0}/nvda_{0}.exe",
                version.trim()
            ),
        })
    }
}
