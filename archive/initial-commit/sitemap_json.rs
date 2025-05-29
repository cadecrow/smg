use anyhow::Result;
use serde_json::{from_reader, to_string_pretty};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use crate::RouteInfo;

pub fn generate(routes: &[RouteInfo], output_path: &Path) -> Result<()> {
    // Read existing sitemap.json if it exists
    let mut old_labels: HashMap<String, (String, String)> = HashMap::new();
    if output_path.exists() {
        let file = File::open(output_path)?;
        let reader = BufReader::new(file);
        if let Ok(existing_routes) = from_reader::<_, Vec<RouteInfo>>(reader) {
            for route in existing_routes {
                old_labels.insert(route.route, (route.label, route.description));
            }
        }
    }

    // Merge
    let mut merged_routes = Vec::with_capacity(routes.len());
    for mut route in routes.to_owned() {
        if let Some((label, description)) = old_labels.get(&route.route) {
            route.label = label.clone();
            route.description = description.clone();
        }
        merged_routes.push(route);
    }

    // Write merged routes and descriptions
    let json = to_string_pretty(&merged_routes)?;
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
