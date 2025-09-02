fn template(context, props, content) {
  if context.is_watching {
    context.assets.add("resources/ts/global-live-reload.ts");
  }

  context.assets.add("resources/css/layout-common.css");

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
        <div class="page">
          <nav class="primary-navigation">
            <div class="primary-navigation__group primary-navigation__group--logo">
              <a href={context.link_to("index")} class="primary-navigation__link primary-navigation__link--logo">
              Poet 🪶
              </a>
            </div>
            <div class="primary-navigation__group-wrapper">
              <div class="primary-navigation__group">
                <a
                  href={context.link_to("#getting-started")}
                  class={clsx(#{
                    "primary-navigation__link": true,
                    "primary-navigation__link--active": context.front_matter.collections.has("documentation_pages"),
                  })}
                >
                  Docs
                </a>
              </div>
              <div class="primary-navigation__group">
                <a
                  href="https://github.com/intentee/poet"
                  class="primary-navigation__link"
                  target="_blank"
                >
                  <img alt="GitHub" src={context.assets.file("resources/media/logo-github.svg")}>
                </a>
              </div>
            </div>
          </nav>
          <main class="primary-main">
            {content}
          </main>
          <footer class="footer">
            <p>
              Made with ❤️ by <a href="https://intentee.com" target="_blank">Intentee</a> and contributors.
            </p>
            <div class="footer__links">
              <a
                class="footer__links__link"
                href="https://bsky.app/profile/intentee.bsky.social"
                target="_blank"
              >
                <img alt="Bluesky" src={context.assets.file("resources/media/logo-bluesky.svg")}>
              </a>
              <a
                class="footer__links__link"
                href="https://discord.gg/92x3Z8a4gj"
                target="_blank"
              >
                <img alt="Discord" src={context.assets.file("resources/media/logo-discord.svg")}>
              </a>
              <a
                class="footer__links__link"
                href="https://www.linkedin.com/company/intentee"
                target="_blank"
              >
                <img alt="LinkedIn" src={context.assets.file("resources/media/logo-linkedin.svg")}>
              </a>
              <a
                class="footer__links__link"
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
