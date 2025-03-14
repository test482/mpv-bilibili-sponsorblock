use std::collections::HashSet;
use std::io::{Error, ErrorKind};

use regex::Regex;
use serde::{Deserialize, Deserializer};
use sponsorblock_client::{Action, Category};
use url::Url;

const SB_ADDRESS: &str = "https://bsbsb.top";
const BILI_PATTERNS: &[&str] = &[r"(?:www\.|m\.|)bilibili\.com/video/"];

fn build_domains_regex(patterns: &[&str]) -> Result<Regex, regex::Error> {
    assert!(!patterns.is_empty());

    let pattern = format!(
        // https?://(?:{})([0-9A-Za-z]*)(?:\?p=(\d+)|/)? 多P视频
        r"https?://(?:{})([0-9A-Za-z]*)",
        patterns.join("|")
    );

    Regex::new(&pattern)
}

fn default_server() -> Url {
    Url::parse(SB_ADDRESS).unwrap()
}

fn default_domains_regex() -> Regex {
    build_domains_regex(BILI_PATTERNS).unwrap()
}

fn from_domains<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let domains: HashSet<String> = Deserialize::deserialize(deserializer)?;
    let domains_escaped: Vec<String> = domains.into_iter().map(|d| regex::escape(&d)).collect();
    let domains_patterns: Vec<&str> = domains_escaped.iter().map(String::as_str).collect();

    let patterns = [BILI_PATTERNS, domains_patterns.as_slice()].concat();

    build_domains_regex(&patterns).map_err(serde::de::Error::custom)
}

#[derive(serde_derive::Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_server")]
    pub server_address: Url,
    #[serde(default)]
    pub categories: HashSet<Category>,
    #[serde(default)]
    pub action_types: HashSet<Action>,
    #[serde(default)]
    pub privacy_api: bool,
    #[serde(
        default = "default_domains_regex",
        deserialize_with = "from_domains",
        rename = "domains"
    )]
    pub bilibili_regex: Regex,
    #[serde(default)]
    pub skip_notice: bool,
}

impl Config {
    pub fn get() -> Self {
        dirs::config_dir()
            .ok_or(Error::new(ErrorKind::NotFound, "configuration directory not found"))
            .and_then(|dir| std::fs::read_to_string(dir.join("mpv/bilibili-sponsorblock.toml")))
            .and_then(|data| toml::from_str(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .unwrap_or_else(|e| {
                log::warn!("Failed to load configuration file: {}. Falling back to default", e);
                Self::default()
            })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: default_server(),
            categories: HashSet::default(),
            action_types: HashSet::default(),
            privacy_api: bool::default(),
            bilibili_regex: default_domains_regex(),
            skip_notice: bool::default(),
        }
    }
}
