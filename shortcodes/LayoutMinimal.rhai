fn template(context, props, content) {
  context.assets.add("resources/css/layout-minimal.css");

  if context.is_watching {
    context.assets.add("resources/ts/global-live-reload.ts");
  }

  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <link rel="canonical" href={context.reference.canonical_link}>
        <link rel="icon" href={context.assets.file("resources/media/favicon.svg")}>

        <meta charset="UTF-8">
        <meta name="turbo-refresh-method" content="morph">
        <meta name="turbo-refresh-scroll" content="preserve">
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
        <div class="page">
          <nav class="page-navigation">
            <div class="page-navigation__track">
              <a class="page-navigation__link page-navigation__link--site-name" href={context.link_to("index")}>
                Poet
              </a>
              <div class="page-navigation__dropdowns">
                <LayoutMinimalDropdown
                  collection="create_content"
                  href={context.link_to("#static-site-generator-initial-page")}
                  label="Create content"
                >
                  <a
                    href={context.link_to("#static-site-generator-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    Static Site Generator
                  </a>
                  <a
                    href={context.link_to("#templating-engine-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    Templating Engine
                  </a>
                  <a
                    href={context.link_to("#consistency-guard-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                      "section-create-content": true,
                    })}
                  >
                    Consistency Guard
                  </a>
                </LayoutMinimalDropdown>
                <LayoutMinimalDropdown
                  collection="leverage_content"
                  href={context.link_to("#mcp-initial-page")}
                  label="Leverage content"
                >
                  <a
                    href={context.link_to("#mcp-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    Model Context Protocol
                  </a>
                  <a
                    href={context.link_to("#rag-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    RAG
                  </a>
                  <a
                    href={context.link_to("#llm-seo-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    LLM SEO
                  </a>
                </LayoutMinimalDropdown>
                <LayoutMinimalDropdown
                  collection="analyze_content"
                  href={context.link_to("#nlp-initial-page")}
                  label="Analyze content"
                >
                  <a
                    href={context.link_to("#nlp-initial-page")}
                    class={clsx(#{
                      "page-navigation__link": true,
                      "page-navigation__link--dropdown": true,
                    })}
                  >
                    Natural Language Processing
                  </a>
                </LayoutMinimalDropdown>
              </div>
            </div>
            <a
              class="page-navigation__link"
              href="https://github.com/intentee/poet"
              target="_blank"
            >
              <img alt="GitHub" src={context.assets.file("resources/media/logo-github.svg")}>
            </a>
          </nav>
          <main class="page-main">
            {content}
          </main>
          <footer class="page-footer">
            <p>
              Made with ❤️ by <a href="https://intentee.com" target="_blank">Intentee</a> and contributors.
            </p>
            <div class="page-footer__links">
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
            </div>
          </footer>
        </div>
      </body>
    </html>
  }
}
