fn template(context, props, content) {
  context.assets.add("resources/css/page-homepage.css");

  component {
    <LayoutMinimal>
      <div class="homepage">
        <h1 class="homepage__title">
          Poet 🪶
        </h1>
        <h2 class="homepage__subtitle">
          Not only build your website, but also expose it as an MCP server,
          optimize for LLM SEO, or analyze the content with NLP.
        </h2>
        <div class="homepage__cta">
          <a href={context.link_to("#static-site-generator-initial-page")} class="homepage__cta__button homepage__cta__button--primary">
            Get Started
          </a>
          <a href={context.link_to("#install")} class="homepage__cta__button">
            Install
          </a>
        </div>
      </div>
    </LayoutMinimal>
  }
}
