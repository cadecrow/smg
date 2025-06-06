use anyhow::Result;
use std::fs::File;
use std::path::Path;
use xml::writer::{EventWriter, XmlEvent};

use crate::RouteInfo;

pub fn generate(routes: &[RouteInfo], output_path: &Path, base_url: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = EventWriter::new(file);

    // XML declaration
    writer.write(XmlEvent::StartDocument {
        version: xml::common::XmlVersion::Version10,
        encoding: Some("UTF-8"),
        standalone: None,
    })?;

    // Urlset start tag with namespaces
    writer.write(XmlEvent::start_element("urlset")
        .default_ns("http://www.sitemaps.org/schemas/sitemap/0.9")
        .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xsi:schemaLocation", "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd")
    )?;

    // Write each URL entry
    for route in routes {
        writer.write(XmlEvent::start_element("url"))?;

        // Location
        writer.write(XmlEvent::start_element("loc"))?;
        let full_url = format!("{}{}", base_url.trim_end_matches('/'), route.route);
        writer.write(XmlEvent::characters(&full_url))?;
        writer.write(XmlEvent::end_element())?;

        // Last modified
        if let Some(last_mod) = route.last_modified {
            writer.write(XmlEvent::start_element("lastmod"))?;
            writer.write(XmlEvent::characters(
                &last_mod.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            ))?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?; // Close url
    }

    writer.write(XmlEvent::end_element())?; // Close urlset

    Ok(())
}
