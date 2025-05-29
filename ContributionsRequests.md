# Contributions & Requests

# --- Following Docs in transition from initial commit ---

## What the heck is in this repo?

This repo contains the source code and documentation for the smg project.

## What are some things I should know if I want to contribute?

- On help navigating and grokking the codebase
  The codebase is not huge. You should be able to read through it and get an idea behind it without a ton of documentation.
  Read through the features in the [README.md](README.md) file and that should help you intuit what is going on.

- Check out the [Requests](#requests)
  This will let you know what still needs work and provide info on some of the missing things you could expect to exist in an open source project.

- Read the [Contribution Guidelines](#contribution-guidelines)
  This will give you a quick rundown of what to do so you can make a contribution.

- Check out the [Scratch Notes](ScratchNotes.md) page
  This will let you know what is currently being worked on and some of the issues that need to be addressed.

- Run [Tests](#tests) before submitting a contribution.
  The test suite is not implemented yet, so the above point is not necessary yet.
  If you felt like completing the test as your contribution, that would be awesome.

### Requests

refer to the issues in the repo for some of the things that need to be done.
As I am typing this for the first time these are some obvious things.

- complete the test suite
- fix the buggy conflict resolution for excluded routes and custom sitemaps
- If the project feels difficult to understand, add documentation you think is missing or helped you get a sense of the project.

### Tests

I have not yet implemented the test suite for this project.
I have included a markdown file and a Next.js project with the intention it be turned into a test suite.

## Contribution Guidelines

I will provide some general guidelines here. Hopefully I do a decent enough job with my explanations and you work in good faith so you won't have to read through the super specific criteria.

For completeness, and if you get confused on why your contribution was rejected, I will have specific criteria for the next.js test app.

### Please leave detailed messages on the code you write (not in the code).

- I am not looking for comments in the code. In fact, I would probably prefer if you didn't write a bunch of comments.
- Feel free to include a markdown file with your information and the changes you made.

### Adding rust crates

- Please add the crate to the Cargo.toml file.
- Please provide information on the crate and justification, such as info on the team that maintains the crate. Please do not willy nilly add crates.

### Updating the Next.js Test App for testing.

- Only update this if it will meaningully help with tests.
- Only directories and page.tsx files are allowed to be added to the test app.
- If you are adding new pages and routes, please copy the root page.tsx or the page in example/page.tsx !!!
- Please do not try to make a real app with a nice frontend.

If contributions of significant web app code were accepted, there would be safety risks with the project that would have to monitored for.
Plus, there is no need to make some fancy app in order to test this project.
For that reason, only extremely simple content added to the test app will be accepted as contribition.

### Criteria

Seriously, just skim the basics above and you should get a sense of what to do.
But if I have to type out these excessive rules that have to be enforced because people can be assholes, here they are:

Criteria for adding rust crates:

- Please add the crate to the Cargo.toml file.
- Please provide information on the crate and justification, such as info on the team that maintains the crate.
- Contributions with crates that are not well known, or a lesser known crate without a good justification will be rejected.

Criteria for contributions to the Next.js Test App

- Only directories and page.tsx files are allowed to be added to the test app.
- No additional, new, or updates to external dependencies in the Next.js test app are allowed in your contribution.
- pages added are derived from the root page.tsx or the page in example/page.tsx
- No edits to the SitemapList.tsx file unless related to your contribution that adds a new feature to the sitemaps through Rust.
- Title: The page contents can include a single Title element using an <h1> element only.
- List: The page contents can include a List rendered from the sitemaps using <ul>, <li>, and <span> elements only.
- List: Within the <span> elements in a <li> element, you can include information from the sitemap with syntax like `{item.label}`, `{item.route}`, or plain text only.
- Title and List: The Title and List can (and should) be wrapped in a <div> element if both are present on the page.
- The pages added are valid html and React for a Next.js project.
- Styles are are allowed through tailwind with the `className` attribute only.
- The only styles allowed are `flex`, `flex-col`, `items-center`, `justify-center` and `gap-2`, `gap-4`, or other default tailwind gap classes.
- The routes to the added paths add meaningful value to the ability to test the sitemap parsing and the smg project.
- The contribution to the next app adds a meaningful way to test the sitemap parsing and the smg project.
