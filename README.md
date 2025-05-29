# smg

smg is a SiteMap Generator for webapps.

The goal of the project is to generate custom sitemaps for any web app, regardless of framework or meta-framework, with a single cli command. 

## Current support
- Next.js (lacking perfect implementation of some features)

## Important Notes and History

This project is evolving from a scrappy project @cadecrow made for himself to get familiar with Rust.
The original purpose was to generate sitemaps within a Next.js app router project.
The docs and related code from the "initial commit" are evolving from this use case.

### Roadmap
- convert from implementation only meant for next.js projects to a cli tool with a setup wizard. The setup wizard will allow users to config smg based on their framework, router, etc...

# --- Following Docs in transition from initial commit ---
A Next.js Sitemap Generator

A command-line tool written in Rust that scans your Next.js project to automatically generate both XML and JSON sitemaps. The tool analyzes your project's file structure, identifies all routes based on the App Router conventions, and creates sitemap files that can be used for SEO and navigation.

## Features

Stable Features:

- Automatically detects routes based on Next.js App Router conventions
- Generates standard sitemap.xml for search engines
- Creates a detailed sitemap.json with labels and descriptions for building navigation components
- Handles dynamic routes (e.g., `[id]`, `[...slug]`)
- Removes directories wrapped in parentheses from routes: `app/(root)/path/page.tsx` => `/path`
- Preserves file modification times for `lastmod` entries
- Preserves custom labels and descriptions for unchanged paths between successive executions of sitemap generation

Testing or Under Development:

- Custom output files and path matching rules. See [Advanced Usage](#advanced-usage)
- - see [Scratch Notes](ScratchNotes.md) for some very informal notes on the current state of development.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
Please review [Contributions & Requests](ContributionsRequests.md) for information on contributing, help to understand the project and its structure, and to see requests for help.

## Installation

### Prereqs

- Rust 1.70.0 or higher

### Building from source

```
# Clone the repository
git clone https://github.com/cadecrow/smg.git
cd smg

# Build the project
cargo build --release
```

### Recommended: Install with cargo

from within smg directory

```
cargo install --path .
```

If you do not use cargo, you can add the Release Directory to your PATH or copy the Binary to a PATH directory.
I will not explain this here. If you do not know what this means but you want to use one of these options, you'd be best served doing some googling or perplexitying to learn what this means.

## Basic Usage

```
smg [OPTIONS]
```

### Command Line Arguments

| Argument      | Short | Description                  | Default               |
| ------------- | ----- | ---------------------------- | --------------------- |
| --project     | -p    | Path to your Next.js project | Current directory (.) |
| --xml-output  |       | Output path for sitemap.xml  | sitemap.xml           |
| --json-output |       | Output path for sitemap.json | sitemap.json          |
| --base-url    | -b    | Base URL for your website    | https://example.com   |

### Examples

Generate sitemaps for a Next.js project in the current directory:

```
smg
```

Specify a different project directory and base URL:

```
smg --project ./my-nextjs-app --base-url https://mywebsite.com
```

Customize output file paths:

```
smg --xml-output ./public/sitemap.xml --json-output ./src/data/sitemap.json
```

### Output Files

#### sitemap.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://example.com/</loc>
    <lastmod>2023-04-29T10:42:19Z</lastmod>
  </url>
  <url>
    <loc>https://example.com/about</loc>
    <lastmod>2023-04-28T15:30:00Z</lastmod>
  </url>
</urlset>
```

#### sitemap.json

```json
[
  {
    "route": "/",
    "path": "app/page.tsx",
    "label": "Home",
    "description": "",
    "last_modified": "2023-04-29T10:42:19Z"
  },
  {
    "route": "/about",
    "path": "app/about/page.tsx",
    "label": "About",
    "description": "",
    "last_modified": "2023-04-28T15:30:00Z"
  }
]
```

## Notes

1. When the program starts, it looks for an `smg.config.json` file
2. If the file exists, it loads the settings
3. If the file doesn't exist, it creates one with default values
4. Command line arguments take precedence over config file settings
5. The merged configuration is used to run the program

This approach gives you flexibility:

- You can set project-specific defaults in the config file
- You can override specific settings via command line when needed
- The config file is automatically created with sensible defaults

The configuration file is particularly useful for CI/CD pipelines or when running the tool regularly, as users won't need to specify the same arguments each time.

- The tool follows Next.js App Router conventions, looking for page.tsx files
- Accounts for folders with names contained within parens. e.g. `app/(layout-group)/blog/page.tsx` => `/blog`
- Dynamic routes (with `[brackets]`) are detected and included in the sitemap
- Routes in directories starting with underscore (\_) or inside api directories are excluded
- You can edit the generated JSON to add custom descriptions and labels

## Advanced Usage

### Configuration

You can create an `smg.config.json` file in your project root to set default options:

#### Example:

```json
{
  "project": ".",
  "xml_output": "public/sitemap.xml",
  "json_output": "src/data/sitemap.json",
  "base_url": "https://mywebsite.com"
}
```

If no configuration file exists, the tool will create one with default values when first run.

REMEMBER: Command line arguments will always override settings in the configuration file.

### Advanced Configuration

The `smg.config.json` file supports advanced path filtering and custom sitemap generation:

### Excluding Routes

You can exclude specific routes from both the sitemap.xml and sitemap.json sitemaps:

```json
{
  "excluded_routes": {
    "exact": ["/admin", "/login"],
    "children": ["/drafts"],
    "patterns": ["^/temp-.*$"]
  }
}
```

- exact: Matches exact routes
- children: Matches the specified route and all its children
- patterns: Matches routes using regular expressions

### Custom Sitemaps and Exclusion Rules

You can generate additional sitemap files for specific groups of routes.
You can also specify whether or not these routes are added to the main sitemap.json and sitemap.xml files.

```json
{
  "custom_sitemaps": {
    "blog": {
      "output": "blog_sitemap.json",
      "include_in_main_json": true,
      "include_in_main_xml": false,
      "routes": {
        "exact": ["/blog"],
        "children": ["/posts"],
        "patterns": ["^/articles/.*$"]
      }
    },
    "products": {
      "output": "products_sitemap.json",
      "include_in_main_json": false,
      "include_in_main_xml": true,
      "routes": {
        "exact": [],
        "children": ["/products"],
        "patterns": []
      }
    }
  }
}
```

For each custom sitemap:

- output: Path where the custom sitemap JSON will be saved
- include_in_main_json: Whether to include matching routes in the main JSON sitemap
- include_in_main_xml: Whether to include matching routes in the main XML sitemap
- routes: Route patterns to include in this custom sitemap

### Route Matching Rules and Conflict Resolution

When routes match multiple rules:

!!! Note, conflict resolution is currently buggy. I (or an awesome contributor such as maybe... yourself) will fix it and update this.

If a custom sitemap sets inclusion in the main sitemap to true, it will override excluded routes.

You can still use this just fine for creating custom sitemaps and excluding paths. The bug is just that a conflict in custom_routes will override excluded_routes

#### Intended Behavior

If a path is in `excluded_routes`, it won't appear in the main sitemaps, regardless of the rules in `custom_sitemaps`
If a path matches multiple custom sitemaps, it will appear in all matching custom sitemap files
A path will only appear in the main sitemaps if all its matching custom sitemaps have the respective include*in_main*_ flag set to true. In other words, if any custom sitemap has `include*in_main*_ = false`, then that path will not make it to the main sitemap file.

#### Current Behavior

If a path is in `excluded_routes`, it won't appear in the main sitemaps, BUT rules in `custom_sitemaps` will override this.
If a path matches multiple custom sitemaps, it will appear in all matching custom sitemap files

## Example Configuration

Here's a complete example of what the configuration file might look like:

```json
{
  "project": ".",
  "xml_output": "public/sitemap.xml",
  "json_output": "public/sitemap.json",
  "base_url": "https://example.com",

  "excluded_routes": {
    "exact": ["/admin", "/login", "/logout"],
    "children": ["/internal"],
    "patterns": ["^/temp-.*$", "^/draft-.*$"]
  },

  "custom_sitemaps": {
    "blog": {
      "output": "public/blog_sitemap.json",
      "include_in_main_json": true,
      "include_in_main_xml": true,
      "routes": {
        "exact": ["/blog"],
        "children": ["/posts"],
        "patterns": ["^/articles/.*$"]
      }
    },
    "products": {
      "output": "public/products_sitemap.json",
      "include_in_main_json": false,
      "include_in_main_xml": true,
      "routes": {
        "children": ["/products"]
      }
    },
    "docs": {
      "output": "public/docs_sitemap.json",
      "include_in_main_json": false,
      "include_in_main_xml": false,
      "routes": {
        "children": ["/docs", "/guides"]
      }
    }
  }
}
```