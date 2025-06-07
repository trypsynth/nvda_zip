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

/// Fetches and caches NVDA download URLs.
#[derive(Default)]
pub struct NvdaUrl {
    client: Client,
    versions: Mutex<HashMap<VersionType, VersionEntry>>,
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
        let now = Instant::now();
        let mut cache = self.versions.lock().await;
        if let Some(entry) = cache.get(&version_type) {
            if entry.last_refresh.elapsed() < CACHE_TTL {
                return Some(entry.url.clone());
            }
        }
        let url = self.fetch_url(&version_type).await?;
        cache.insert(
            version_type,
            VersionEntry {
                url: url.clone(),
                last_refresh: now,
            },
        );
        Some(url)
    }
}

struct VersionEntry {
    url: String,
    last_refresh: Instant,
}

impl NvdaUrl {
    async fn fetch_url(&self, version_type: &VersionType) -> Option<String> {
        let version_str = match version_type {
            VersionType::Alpha => "snapshot:alpha",
            VersionType::Beta => "beta",
            VersionType::Stable => "stable",
        };
        let check_url = format!(
            "https://download.nvaccess.org/nvdaUpdateCheck?versionType={}",
            version_str
        );
        let body = self
            .client
            .get(&check_url)
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        let regex = get_regex(version_type);
        let captured = regex.captures(&body)?;
        Some(match version_type {
            VersionType::Alpha => captured.get(1)?.as_str().to_owned(),
            VersionType::Beta | VersionType::Stable => {
                format_download_url(captured.get(1)?.as_str())
            }
        })
    }
}

fn format_download_url(version: &str) -> String {
    let version = version.trim();
    format!(
        "https://download.nvaccess.org/download/nvda/releases/{}/nvda_{}.exe",
        version, version
    )
}

fn get_regex(version_type: &VersionType) -> &'static Regex {
    static ALPHA_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"launcherUrl:\s*(\S+)").unwrap());
    static STABLE_BETA_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"version:\s*(\S+)").unwrap());
    match version_type {
        VersionType::Alpha => &ALPHA_REGEX,
        VersionType::Beta | VersionType::Stable => &STABLE_BETA_REGEX,
    }
}
