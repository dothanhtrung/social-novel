
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use tracing::log::info;
use web_misc::db::DBConfig;

const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0";
const DEFAULT_LISTEN_PORT: u32 = 9876;

const DEFAULT_API_PER_PAGE: u32 = 20;
const DEFAULT_PARALLEL: usize = 8;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct APIConfig {
    pub per_page: u32,
}

impl Default for APIConfig {
    fn default() -> Self {
        Self {
            per_page: DEFAULT_API_PER_PAGE,
        }
    }
}

pub const DEFAULT_CHAT_SETUP_PROMPT: &str = "You are writing a Facebook Messenger reply for a fictional story.

Receiver (you):
Name: {receiver_name}
Username: {receiver_username}
Back story: {receiver_back_story}

Sender:
Name: {sender_name}
Username: {sender_username}
Back story: {sender_back_story}

Recent messages (oldest first):
{recent_messages}

The sender just said:
{current_message}

Reply as {receiver_name} in a short, natural chat style. Output only the reply text, no name prefix, no quotes.";

fn default_chat_setup_prompt() -> String {
    DEFAULT_CHAT_SETUP_PROMPT.to_string()
}

#[derive(Debug, Serialize, Clone)]
pub struct LLMConfig {
    /// Base URLs of LLM servers, tried in order. The first one that answers is used.
    /// An older config may still say `url` (one string, or a list). That loads into this field.
    pub urls: Vec<String>,
    /// Overall prompt for an LLM chat reply. Placeholders are replaced at send time.
    pub chat_setup_prompt: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LlmUrlField {
    Many(Vec<String>),
    One(String),
}

impl Default for LlmUrlField {
    fn default() -> Self {
        Self::One(String::new())
    }
}

impl<'de> Deserialize<'de> for LLMConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(default)]
            urls: Vec<String>,
            #[serde(default)]
            url: LlmUrlField,
            #[serde(default = "default_chat_setup_prompt", alias = "character_setup_prompt")]
            chat_setup_prompt: String,
        }

        let raw = Raw::deserialize(deserializer)?;
        let mut urls = raw.urls;
        if urls.iter().all(|url| url.trim().is_empty()) {
            urls = match raw.url {
                LlmUrlField::Many(urls) => urls,
                LlmUrlField::One(url) => vec![url],
            };
        }
        Ok(Self {
            urls: urls.into_iter().map(|url| url.trim().to_string()).filter(|url| !url.is_empty()).collect(),
            chat_setup_prompt: raw.chat_setup_prompt,
        })
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            urls: Vec::new(),
            chat_setup_prompt: default_chat_setup_prompt(),
        }
    }
}

pub struct ChatSetupVars<'a> {
    pub sender_name: &'a str,
    pub sender_username: &'a str,
    pub sender_back_story: &'a str,
    pub receiver_name: &'a str,
    pub receiver_username: &'a str,
    pub receiver_back_story: &'a str,
    pub recent_messages: &'a str,
    pub current_message: &'a str,
}

impl LLMConfig {
    /// Fill `chat_setup_prompt` placeholders for a receiver reply.
    /// An empty prompt uses `DEFAULT_CHAT_SETUP_PROMPT`.
    pub fn chat_setup_message(&self, vars: &ChatSetupVars<'_>) -> String {
        let template = if self.chat_setup_prompt.trim().is_empty() {
            DEFAULT_CHAT_SETUP_PROMPT
        } else {
            self.chat_setup_prompt.as_str()
        };
        template
            .replace("{sender_name}", vars.sender_name)
            .replace("{sender_username}", vars.sender_username)
            .replace("{sender_back_story}", vars.sender_back_story)
            .replace("{receiver_name}", vars.receiver_name)
            .replace("{receiver_username}", vars.receiver_username)
            .replace("{receiver_back_story}", vars.receiver_back_story)
            .replace("{recent_messages}", vars.recent_messages)
            .replace("{current_message}", vars.current_message)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub db: DBConfig,
    #[serde(default)]
    pub listen_addr: String,
    #[serde(default)]
    pub listen_port: u32,
    pub api: APIConfig,
    #[serde(default)]
    pub parallel: usize,
    pub data_dir: String,
    #[serde(default)]
    pub basic_auth_user: String,
    #[serde(default)]
    pub basic_auth_pass: String,
    #[serde(default)]
    pub llm: LLMConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: DEFAULT_LISTEN_ADDR.to_string(),
            listen_port: DEFAULT_LISTEN_PORT,
            parallel: DEFAULT_PARALLEL,
            db: DBConfig::default(),
            api: APIConfig::default(),
            data_dir: "./data".to_string(),
            basic_auth_user: "admin".to_string(),
            basic_auth_pass: "admin".to_string(),
            llm: LLMConfig::default(),
        }
    }
}

impl Config {
    /// Load config from file
    pub fn load(config_path: &Path) -> anyhow::Result<Self> {
        let file = File::open(config_path)?;
        let config = ron::de::from_reader(file)?;
        Ok(config)
    }

    /// Save config to file
    pub fn save(&self, config_path: &Path, force_overwrite: bool) -> anyhow::Result<()> {
        info!("Saving config file to: {:?}", config_path.display());
        if config_path.exists() && !force_overwrite {
            return Err(anyhow::anyhow!("Do not save. Configuration file already exists"));
        }

        let pretty = PrettyConfig::default();
        let ron_str = to_string_pretty(self, pretty)?;

        if let Some(parent_dir) = config_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        let mut file = File::create(config_path)?;
        file.write_all(ron_str.as_bytes()).map_err(|e| anyhow::anyhow!(e))
    }
}

pub struct ConfigData {
    pub config: RwLock<Config>,
    pub config_path: PathBuf,
    /// Signaled after a successful apply so the HTTP server can restart
    /// and pick up bind address, database, and static-file roots.
    pub restart: tokio::sync::Notify,
}

impl ConfigData {
    /// Write `new_config` to the RON file and into the live lock, then
    /// ask the process to reload.
    pub async fn apply(&self, new_config: Config) -> anyhow::Result<()> {
        new_config.save(&self.config_path, true)?;
        *self.config.write().await = new_config;
        self.restart.notify_one();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with_llm(llm: &str) -> String {
        format!(
            r#"(
            data_dir: "./data",
            api: (per_page: 3),
            llm: ({llm}),
        )"#
        )
    }

    #[test]
    fn llm_urls_load_from_legacy_url_string() {
        let cfg: Config = ron::from_str(&config_with_llm(r#"url: "http://127.0.0.1:8080", chat_setup_prompt: "hi""#)).unwrap();
        assert_eq!(cfg.llm.urls, vec!["http://127.0.0.1:8080"]);
        assert_eq!(cfg.llm.chat_setup_prompt, "hi");

        let empty: Config = ron::from_str(&config_with_llm(r#"url: """#)).unwrap();
        assert!(empty.llm.urls.is_empty());

        let listed: Config = ron::from_str(&config_with_llm(r#"url: ["http://a", "http://b"]"#)).unwrap();
        assert_eq!(listed.llm.urls, vec!["http://a".to_string(), "http://b".to_string()]);
    }

    #[test]
    fn llm_urls_load_from_array_and_old_prompt_name() {
        let cfg: Config = ron::from_str(&config_with_llm(
            r#"urls: ["http://a", " http://b "], character_setup_prompt: "old""#,
        ))
        .unwrap();
        assert_eq!(cfg.llm.urls, vec!["http://a".to_string(), "http://b".to_string()]);
        assert_eq!(cfg.llm.chat_setup_prompt, "old");
    }

    #[test]
    fn llm_urls_round_trip() {
        let cfg: Config = ron::from_str(&config_with_llm(r#"urls: ["http://a", "http://b"]"#)).unwrap();
        let saved = to_string_pretty(&cfg.llm, PrettyConfig::default()).unwrap();
        let loaded: LLMConfig = ron::from_str(&saved).unwrap();
        assert_eq!(loaded.urls, vec!["http://a".to_string(), "http://b".to_string()]);
        assert!(saved.lines().any(|line| line.trim_start().starts_with("urls:")));
        assert!(!saved.lines().any(|line| line.trim_start().starts_with("url:")));
    }

    #[test]
    fn live_ron_still_loads() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../social-novel.test.ron");
        let cfg = Config::load(&path).expect("social-novel.test.ron");
        assert!(!cfg.llm.chat_setup_prompt.trim().is_empty());
    }
}