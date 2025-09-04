fn template(context, props, content) {
  if context.is_watching {
    context.assets.add("resources/ts/global-live-reload.ts");
  }

  context.assets.add("resources/css/layout-minimal.css");

  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <link rel="canonical" href={context.reference.canonical_link}>
        <link rel="icon" href={context.assets.file("resources/media/favicon.svg")}>

        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        {if !context.front_matter.description.is_empty() {
          component {
            <meta name="description" content={context.front_matter.description}>
          }
        }}

        <title>{
          let base_title = "Poet";

          if context.front_matter.title.is_empty() {
            base_title
          } else {
            `${context.front_matter.title} - ${base_title}`
          }
        }</title>
        {context.assets.render()}
      </head>
      <body>
        <nav class="page-navigation">
          <div class="page-navigation__track">
            <div class="page-navigation__group">
              <a class="page-navigation__link" href={context.link_to("index")}>
                Poet
              </a>
              <LayoutMinimalDropdown label="Create content">
                <a
                  href={context.link_to("#static-site-generator-initial-page")}
                  class={clsx(#{
                    "page-navigation__link": true,
                    "active": context.belongs_to("static_site_generator"),
                    "section-active": context.belongs_to("create_content"),
                    "section-create-content": true,
                  })}
                >
                  Static Site Generator
                </a>
                <a
                  href={context.link_to("#templating-engine-initial-page")}
                  class={clsx(#{
                    "page-navigation__link": true,
                    "active": context.belongs_to("templating_engine"),
                    "section-active": context.belongs_to("create_content"),
                    "section-create-content": true,
                  })}
                >
                  Templating Engine
                </a>
                <a
                  href={context.link_to("#consistency-guard-initial-page")}
                  class={clsx(#{
                    "page-navigation__link": true,
                    "active": context.belongs_to("consistency_guard"),
                    "section-active": context.belongs_to("create_content"),
                    "section-create-content": true,
                  })}
                >
                  Consistency Guard
                </a>
              </LayoutMinimalDropdown>
            </div>
            <LayoutMinimalDropdown label="Create content">
              <a
                href={context.link_to("#mcp-initial-page")}
                class={clsx(#{
                  "page-navigation__link": true,
                  "active": context.belongs_to("mcp"),
                  "section-active": context.belongs_to("leverage_content"),
                  "section-leverage-content": true,
                })}
              >
                Model Context Protocol
              </a>
              <a
                href={context.link_to("#rag-initial-page")}
                class={clsx(#{
                  "page-navigation__link": true,
                  "active": context.belongs_to("rag"),
                  "section-active": context.belongs_to("leverage_content"),
                  "section-leverage-content": true,
                })}
              >
                RAG
              </a>
              <a
                href={context.link_to("#llm-seo-initial-page")}
                class={clsx(#{
                  "page-navigation__link": true,
                  "active": context.belongs_to("llm_seo"),
                  "section-active": context.belongs_to("leverage_content"),
                  "section-leverage-content": true,
                })}
              >
                LLM SEO
              </a>
            </LayoutMinimalDropdown>
            <LayoutMinimalDropdown label="Create content">
              <a
                href={context.link_to("#nlp-initial-page")}
                class={clsx(#{
                  "page-navigation__link": true,
                  "active": context.belongs_to("nlp"),
                  "section-active": context.belongs_to("analyze_content"),
                  "section-analyze-content": true,
                })}
              >
                Natural Language Processing
              </a>
            </LayoutMinimalDropdown>
          </div>
        </nav>
        {content}
        <footer>
          <p>
            Made with ❤️ by <a href="https://intentee.com" target="_blank">Intentee</a> and contributors.
          </p>
            <a
              href="https://bsky.app/profile/intentee.bsky.social"
              target="_blank"
            >
              <img alt="Bluesky" src={context.assets.file("resources/media/logo-bluesky.svg")}>
            </a>
            <a
              href="https://discord.gg/92x3Z8a4gj"
              target="_blank"
            >
              <img alt="Discord" src={context.assets.file("resources/media/logo-discord.svg")}>
            </a>
            <a
              href="https://github.com/intentee/poet"
              target="_blank"
            >
              <img alt="GitHub" src={context.assets.file("resources/media/logo-github.svg")}>
            </a>
            <a
              href="https://www.linkedin.com/company/intentee"
              target="_blank"
            >
              <img alt="LinkedIn" src={context.assets.file("resources/media/logo-linkedin.svg")}>
            </a>
            <a
              href="https://x.com/intentee"
              target="_blank"
            >
              <img alt="X" src={context.assets.file("resources/media/logo-x.svg")}>
            </a>
        </footer>
      </body>
    </html>
  }
}
