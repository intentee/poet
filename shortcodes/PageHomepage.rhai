fn template(context, props, content) {
  context.assets.add("resources/css/page-homepage.css");

  component {
    <LayoutMinimal>
      <div class="homepage">
        <div class="homepage__product-name">
        </div>
        <h1 class="homepage__title">
          Static site generator
        </h1>
        <div class="homepage__details">
          <h2>What is it?</h2>
          <p>
            <strong>Poet</strong> is a static site generator with LLM SEO features, RAG, and a built-in
            MCP server.
          </p>
          <h2>For whom?</h2>
          <p>Technical writing and documentation teams</p>
          <p>Data scientists who need to perform NLP analysis of the documentation pages</p>
          <div class="homepage__cta">
            <a href={context.link_to("#static-site-generator-initial-page")} class="homepage__cta__button homepage__cta__button--primary">
              Get Started
            </a>
            <a href={context.link_to("#install")} class="homepage__cta__button">
              Install
            </a>
          </div>
        </div>
      </div>
    </LayoutMinimal>
  }
}
