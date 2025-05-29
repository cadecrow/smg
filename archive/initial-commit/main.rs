use anyhow::{Context, Result};
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub route: String,
    pub path: String,
    pub label: String,
    pub description: String,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

mod scanner;
mod sitemap_xml;
mod sitemap_json;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to Next.js project
    #[arg(short, long)]
    project: Option<String>,

    /// Output path for sitemap.xml
    #[arg(long)]
    xml_output: Option<String>,

    /// Output path for sitemap.json
    #[arg(long)]
    json_output: Option<String>,

    /// Base URL for sitemap
    #[arg(short, long)]
    base_url: Option<String>,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Load config (or create default if not exists)
    let config = config::Config::load()?;
    
    // Merge config with command line args (CLI args take precedence)
    let project_path = args.project.unwrap_or(config.project.clone());
    let xml_output = args.xml_output.unwrap_or(config.xml_output.clone());
    let json_output = args.json_output.unwrap_or(config.json_output.clone());
    let base_url = args.base_url.unwrap_or(config.base_url.clone());
    
    // Convert to PathBuf where needed
    let project_path = PathBuf::from(project_path);
    let xml_output_path = PathBuf::from(xml_output);
    let json_output_path = PathBuf::from(json_output);
    
    // Scan project for all routes
    let all_routes = scanner::scan_project(&project_path)
        .context("Failed to scan project")?;
    
    // Filter routes for main sitemaps
    let mut main_json_routes: Vec<RouteInfo> = all_routes.iter()
        .filter(|route| config.include_in_main_json(&route.route))
        .cloned()
        .collect();
    
    let mut main_xml_routes: Vec<RouteInfo> = all_routes.iter()
        .filter(|route| config.include_in_main_xml(&route.route))
        .cloned()
        .collect();
    
    // Sort routes for consistent output
    main_json_routes.sort_by(|a, b| a.route.cmp(&b.route));
    main_xml_routes.sort_by(|a, b| a.route.cmp(&b.route));
    
    // Generate main sitemap.xml
    sitemap_xml::generate(&main_xml_routes, &xml_output_path, &base_url)
        .context("Failed to generate sitemap.xml")?;
    
    // Generate main sitemap.json
    sitemap_json::generate(&main_json_routes, &json_output_path)
        .context("Failed to generate sitemap.json")?;
    
    println!("Generated sitemap.xml at {}", xml_output_path.display());
    println!("Generated sitemap.json at {}", json_output_path.display());
    
    // Generate custom sitemaps
    for (key, custom_config) in &config.custom_sitemaps {
        // Filter routes for this custom sitemap
        let custom_routes: Vec<RouteInfo> = all_routes.iter()
            .filter(|route| {
                let matching_sitemaps = config.get_matching_custom_sitemaps(&route.route);
                matching_sitemaps.contains(key)
            })
            .cloned()
            .collect();
        
        if !custom_routes.is_empty() {
            // Sort custom routes
            let mut sorted_custom_routes = custom_routes.clone();
            sorted_custom_routes.sort_by(|a, b| a.route.cmp(&b.route));
            
            // Generate custom sitemap JSON
            let custom_output_path = PathBuf::from(&custom_config.output);
            sitemap_json::generate(&sorted_custom_routes, &custom_output_path)
                .with_context(|| format!("Failed to generate custom sitemap for key '{}'", key))?;
            
            println!("Generated custom sitemap for '{}' at {}", key, custom_output_path.display());
        } else {
            println!("No routes found for custom sitemap '{}'", key);
        }
    }
    
    Ok(())
}
