# Test Writeups

Write markdown to describe tests and include example config files to run the tests

## All Exclusion Rules applied
- Expected Result: All intended paths excluded
- Note: 
```json
{
	"project": ".",
	"xml_output": "public/sitemap.xml",
	"json_output": "public/sitemap.json",
	"base_url": "https://example.com",
	"excluded_routes": {
		"exact": ["/exclude"],
		"children": ["/exclude-children/"],
		"patterns": ["*sneaky*$"]
	}
}
```

## Write custom sitemap file and main sitemap file to same location
- Expected Result: main sitemap appropriately output. custom file given random name
- Note: ! No implementation attempted
```json
{
	"project": ".",
	"xml_output": "public/sitemap.xml",
	"json_output": "public/sitemap.json",
	"base_url": "https://example.com",
	"excluded_routes": {
		"exact": ["/exclude"],
		"children": ["/exclude-children/"],
		"patterns": ["*sneaky*$"]
	},
	"custom_sitemaps": {
		"custom": {
			"output": "public/sitemap.json",
			"include_in_main_json": false,
			"include_in_main_xml": false,
			"routes": {
				"exact": ["/include"],
				"children": ["/include-children/"],
				"patterns": ["*sneaky*include*$"]
			}
		}
	}
}
```

## write custom key with exclusions - no overlap
- Expected Result: main sitemap appropriately output. custom file given random name
- Note:
- Test Config:


## Overlapping paths - overlap in exclude and custom - custom dictates output to main
- Expected Result: no output to main - output to custom
- Note:
- Test Config:

## Overlapping paths - no overlap in excluded and custom - 4 custom overlaps - 3 dictate output to main - final dictates no output to main
- Expected Result: no output to main - output to custom
- Note:
- Test Config:

## Overlapping paths - no overlap in excluded and custom - 4 custom overlaps - 3 dictate output to main - first dictates no output to main
- Expected Result: no output to main - output to custom
- Note:
- Test Config: