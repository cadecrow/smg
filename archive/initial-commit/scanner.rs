use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::RouteInfo;

pub fn scan_project(project_path: &Path) -> Result<Vec<RouteInfo>> {
    let mut routes = Vec::new();

    // Find app directory
    let app_dir = find_app_directory(project_path)?;

    // Walk through all files
    for entry in WalkDir::new(&app_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Check if this is a page.tsx or page.js file
        if is_page_file(path) {
            // Skip if in api directory or starts with underscore
            if is_excluded_path(path) {
                continue;
            }

            let route = path_to_route(&app_dir, path)?;
            let rel_path = path
                .strip_prefix(project_path)?
                .to_string_lossy()
                .to_string();

            // Get last modified time
            let metadata = fs::metadata(path)?;
            let last_modified = metadata.modified().ok().map(|time| {
                let datetime = chrono::DateTime::<Utc>::from(time);
                datetime
            });

            let label = default_label_for_route(&route);
            routes.push(RouteInfo {
                route,
                path: rel_path,
                label,
                description: String::new(), // Empty by default
                last_modified,
            });
        }
    }

    Ok(routes)
}

fn find_app_directory(project_path: &Path) -> Result<PathBuf> {
    // Check for app directory in project root or src/
    let app_dir = project_path.join("app");
    if app_dir.exists() {
        return Ok(app_dir);
    }

    let src_app_dir = project_path.join("src").join("app");
    if src_app_dir.exists() {
        return Ok(src_app_dir);
    }

    anyhow::bail!("Could not find app directory in project")
}

fn is_page_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        let file_name = file_name.to_string_lossy();
        file_name == "page.tsx" || file_name == "page.js" || file_name == "page.jsx"
    } else {
        false
    }
}

fn is_excluded_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains("/api/")
        || path_str.contains("/_")
        || path.components().any(|c| {
            if let std::path::Component::Normal(name) = c {
                let name = name.to_string_lossy();
                name.starts_with('_') || name == "api"
            } else {
                false
            }
        })
}

fn path_to_route(app_dir: &Path, page_path: &Path) -> Result<String> {
    // Remove app_dir prefix and page.tsx filename
    let parent = page_path.parent().unwrap_or(page_path);
    let relative = parent.strip_prefix(app_dir)?;

    // Convert to route
    let mut route = String::from("/");
    for component in relative.components() {
        if let std::path::Component::Normal(name) = component {
            let name = name.to_string_lossy();

            // Skip directories wrapped in parentheses
            if name.starts_with('(') && name.ends_with(')') {
                continue;
            }

            // Handle dynamic routes
            let segment = if name.starts_with('[') && name.ends_with(']') {
                // For [id] or [...slug]
                format!(":{}", &name[1..name.len() - 1])
            } else {
                name.to_string()
            };

            if !route.ends_with('/') {
                route.push('/');
            }
            route.push_str(&segment);
        }
    }

    // Handle root route
    if route == "/" && page_path.file_name().unwrap_or_default() == "page.tsx" {
        return Ok("/".to_string());
    }

    Ok(route)
}

fn default_label_for_route(route: &str) -> String {
    if route == "/" {
        return "Home".to_string();
    }

    // Get the last segment of the route
    let segments: Vec<&str> = route.split('/').filter(|s| !s.is_empty()).collect();
    if let Some(last) = segments.last() {
        // Handle dynamic routes
        if last.starts_with(':') {
            return format!("{} Detail", capitalize_words(&last[1..].replace('-', " ")));
        }

        // Replace hyphens with spaces and capitalize each word
        return capitalize_words(&last.replace('-', " "));
    }

    "Page".to_string()
}

fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
