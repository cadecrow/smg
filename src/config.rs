use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    #[serde(default)]
    pub excluded_routes: ExcludedRoutes,

    #[serde(default)]
    pub custom_sitemaps: HashMap<String, CustomSitemap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExcludedRoutes {
    #[serde(default)]
    pub exact: Vec<String>,

    #[serde(default)]
    pub children: Vec<String>,

    #[serde(default)]
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSitemap {
    pub output: String,

    #[serde(default)]
    pub include_in_main_json: bool,

    #[serde(default)]
    pub include_in_main_xml: bool,

    #[serde(default)]
    pub routes: CustomRoutes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomRoutes {
    #[serde(default)]
    pub exact: Vec<String>,

    #[serde(default)]
    pub children: Vec<String>,

    #[serde(default)]
    pub patterns: Vec<String>,
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
            excluded_routes: ExcludedRoutes::default(),
            custom_sitemaps: HashMap::new(),
        }
    }
}

impl Default for CustomSitemap {
    fn default() -> Self {
        Self {
            output: "custom_sitemap.json".to_string(),
            include_in_main_json: false,
            include_in_main_xml: false,
            routes: CustomRoutes::default(),
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

    // Check if a route should be excluded based on Category 1 rules
    pub fn is_excluded(&self, route: &str) -> bool {
        // Check exact matches
        if self.excluded_routes.exact.contains(&route.to_string()) {
            return true;
        }

        // Check children routes
        for parent in &self.excluded_routes.children {
            if route == parent || route.starts_with(&format!("{}/", parent)) {
                return true;
            }
        }

        // Check regex patterns
        for pattern in &self.excluded_routes.patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(route) {
                    return true;
                }
            }
        }

        false
    }

    // Check if a route belongs to a custom sitemap (Category 2)
    pub fn get_matching_custom_sitemaps(&self, route: &str) -> Vec<String> {
        let mut matches = Vec::new();

        for (key, custom) in &self.custom_sitemaps {
            // Check exact matches
            if custom.routes.exact.contains(&route.to_string()) {
                matches.push(key.clone());
                continue;
            }

            // Check children routes
            for parent in &custom.routes.children {
                if route == parent || route.starts_with(&format!("{}/", parent)) {
                    matches.push(key.clone());
                    continue;
                }
            }

            // Check regex patterns
            for pattern in &custom.routes.patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(route) {
                        matches.push(key.clone());
                        break;
                    }
                }
            }
        }

        matches
    }

    // Check if a route should be included in the main JSON sitemap
    pub fn include_in_main_json(&self, route: &str) -> bool {
        // If it's in Category 1, exclude it
        if self.is_excluded(route) {
            return false;
        }

        // Check all matching custom sitemaps
        let matching_sitemaps = self.get_matching_custom_sitemaps(route);
        if !matching_sitemaps.is_empty() {
            // If any matching sitemap excludes from main JSON, exclude it
            for key in matching_sitemaps {
                if let Some(custom) = self.custom_sitemaps.get(&key) {
                    if !custom.include_in_main_json {
                        return false;
                    }
                }
            }
        }

        true
    }

    // Check if a route should be included in the main XML sitemap
    pub fn include_in_main_xml(&self, route: &str) -> bool {
        // If it's in Category 1, exclude it
        if self.is_excluded(route) {
            return false;
        }

        // Check all matching custom sitemaps
        let matching_sitemaps = self.get_matching_custom_sitemaps(route);
        if !matching_sitemaps.is_empty() {
            // If any matching sitemap excludes from main XML, exclude it
            for key in matching_sitemaps {
                if let Some(custom) = self.custom_sitemaps.get(&key) {
                    if !custom.include_in_main_xml {
                        return false;
                    }
                }
            }
        }

        true
    }
}
