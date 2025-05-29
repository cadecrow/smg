use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_project_path")]
    pub project: String,

    #[serde(default = "default_xml_output")]
    pub xml_output: String,

    #[serde(default = "default_json_output")]
    pub json_output: String,

    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_project_path() -> String {
    ".".to_string()
}

fn default_xml_output() -> String {
    "sitemap.xml".to_string()
}

fn default_json_output() -> String {
    "sitemap.json".to_string()
}

fn default_base_url() -> String {
    "https://example.com".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: default_project_path(),
            xml_output: default_xml_output(),
            json_output: default_json_output(),
            base_url: default_base_url(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Path::new("smg.config.json");

        // If config file exists, load it
        if config_path.exists() {
            let config_str =
                fs::read_to_string(config_path).context("Failed to read smg.config.json")?;

            let config: Config =
                serde_json::from_str(&config_str).context("Failed to parse smg.config.json")?;

            return Ok(config);
        }

        // If no config file exists, create a default one
        let default_config = Config::default();
        let config_str = serde_json::to_string_pretty(&default_config)
            .context("Failed to serialize default config")?;

        fs::write(config_path, config_str).context("Failed to write default smg.config.json")?;

        println!("Created default configuration file: smg.config.json");

        Ok(default_config)
    }
}
