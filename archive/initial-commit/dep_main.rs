use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub route: String,
    pub path: String,
    pub label: String,
    pub description: String,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

mod config;
mod scanner;
mod sitemap_json;
mod sitemap_xml;

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
    let args = Args::parse();

    // Load config (or create default if not exists)
    let config = config::Config::load()?;

    // Merge config with command line args (CLI args take precedence)
    let project_path = args.project.unwrap_or(config.project);
    let xml_output = args.xml_output.unwrap_or(config.xml_output);
    let json_output = args.json_output.unwrap_or(config.json_output);
    let base_url = args.base_url.unwrap_or(config.base_url);

    // Convert to PathBuf where needed
    let project_path = PathBuf::from(project_path);
    let xml_output_path = PathBuf::from(xml_output);
    let json_output_path = PathBuf::from(json_output);

    // Scan project for routes
    let mut routes = scanner::scan_project(&project_path).context("Failed to scan project")?;

    // Sort routes for consistent output
    routes.sort_by(|a, b| a.route.cmp(&b.route));

    // Generate sitemap.xml
    sitemap_xml::generate(&routes, &xml_output_path, &base_url)
        .context("Failed to generate sitemap.xml")?;

    // Generate sitemap.json
    sitemap_json::generate(&routes, &json_output_path)
        .context("Failed to generate sitemap.json")?;

    println!("Generated sitemap.xml at {}", xml_output_path.display());
    println!("Generated sitemap.json at {}", json_output_path.display());

    Ok(())
}
