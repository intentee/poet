fn template(context, props, content) {
  context.assets.add("resources/css/page-homepage.css");

  component {
    <LayoutMinimal>
      <div class="homepage">
        <h1 class="homepage__title">
          Poet 🪶
        </h1>
        <h2 class="homepage__subtitle">
          Open-source static site generator that optionally provides you
          with MCP server, RAG, and AI platforms discoverability
          (also works with self-hosted AI)
        </h2>
        <h2 class="homepage__subtitle">
          Comes in with custom templating engine for modern needs
          (somewhat like React, but better for static pages :))
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
