# Poet

According to the [StackOverflow Developer Survey](https://survey.stackoverflow.co/2025/stack-overflow#participation-and-feedback-so-dev-content), most developers prefer both interactive formats and long-form articles when learning new technologies. 

Why not provide both in one go?

Poet is a modern static site generator designed to provide interactivity to your content with LLM SEO features, RAG, and a built-in MCP server.

## For whom?

- Technical writing and documentation teams that need to create and maintain documentation for their tools and products.
- Data scientists who need to perform an NLP analysis of the written content.
- Operational teams building internal wikis

## Key features

Poet's alpha version is live and provides a modern static site generation with a custom JSX-like syntax based on the [Rhai scripting language](https://rhai.rs/). This custom syntax has been optimized specifically for working well with content.

**Upcoming features include:**
- LLM SEO optimization
- Exposing an MCP server from your content
- AI content analysis features (indicating sources of truth, link reshaping, linting-like suggestions)
- Making content RAG enabled

## Documentation

Visit our documentation page at: https://poet.intentee.com/

## Installation

You can install Poet from Cargo (requires Rust and Nightly) by running:

```bash
cargo +nightly install poet
```

## Getting started

To get started quickly with Poet, use one of the template repositories: 

We currently provide two templates:
- [Minimal template](https://github.com/intentee/poet-template-minimal), a basic template with no content.
- [Documentation template](https://github.com/intentee/poet-template-docs), a template for creating documentation websites. This template comes with a few example pages organized into a collection with a navigation menu and a few other components typical for documentation pages to help you get started quickly. Poet's documentation is built with this template.

## Quick tutorials

Our documentation page provides a set of quick tutorials and articles that guide you through creating content with Poet step by step:

- [Starting out with a template project](https://poet.intentee.com/static-site-generator/starting-out/starting-out-with-a-template-project/)
- [Create your first page](https://poet.intentee.com/static-site-generator/starting-out/create-your-first-page/)
- [Add more pages](https://poet.intentee.com/static-site-generator/starting-out/add-more-pages/)
- [Organize your pages as collections](https://poet.intentee.com/static-site-generator/starting-out/organize-your-pages-as-collections/)
- [Style your content](https://poet.intentee.com/static-site-generator/starting-out/style-your-content/)
