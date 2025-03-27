use regex::Regex;
use reqwest::Client;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const CACHE_TTL: Duration = Duration::from_secs(30);

pub const XP_URL: &str = "https://download.nvaccess.org/download/releases/2017.3/nvda_2017.3.exe";
pub const WIN7_URL: &str =
    "https://download.nvaccess.org/download/releases/2023.3.4/nvda_2023.3.4.exe";

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum VersionType {
    Stable,
    Beta,
    Alpha,
}

pub struct VersionEntry {
    url: String,
    last_refresh: Instant,
}

pub struct NvdaUrl {
    client: Client,
    versions: HashMap<VersionType, VersionEntry>,
}

impl Default for NvdaUrl {
    fn default() -> Self {
        Self {
            client: Client::new(),
            versions: HashMap::new(),
        }
    }
}

impl NvdaUrl {
    pub async fn get_url(&mut self, version_type: VersionType) -> Option<String> {
        if let Some(entry) = self.versions.get(&version_type) {
            if entry.last_refresh.elapsed() < CACHE_TTL {
                return Some(entry.url.clone());
            }
        }
        let url = self.fetch_url(version_type.clone()).await?;
        self.versions.insert(
            version_type,
            VersionEntry {
                url: url.clone(),
                last_refresh: Instant::now(),
            },
        );
        Some(url)
    }

    async fn fetch_url(&self, version_type: VersionType) -> Option<String> {
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
        let regex = match version_type {
            VersionType::Alpha => Regex::new(r"launcherUrl:\s*(.*)").expect("valid regex"),
            VersionType::Beta | VersionType::Stable => {
                Regex::new(r"version:\s*(.*)").expect("valid regex")
            }
        };
        let captured = regex.captures(&body)?;
        Some(match version_type {
            VersionType::Alpha => captured.get(1)?.as_str().to_string(),
            VersionType::Beta | VersionType::Stable => {
                let version = captured.get(1)?.as_str().trim();
                format!(
                    "https://download.nvaccess.org/download/releases/{}/nvda_{}.exe",
                    version, version
                )
            }
        })
    }
}
