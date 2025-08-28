fn template(context, props, content) {
  context.assets.add("resources/css/page-docs.css");

  component {
    <LayoutMinimal>
      <div class="docs">
          <input checked type="checkbox" id="docs-nav-toggle">
          <nav class="docs__nav">
              <label class="docs__nav__toggle" for="docs-nav-toggle">
                Mobile docs navigation placeholder
              </label>
              <div class="docs__nav__content">
                <div class="docs__nav-section-title">
                  Docs pages titles navigation placeholder
                </div>
                    <div class="docs__nav-section">
                        <a
                            class="docs__nav-link"
                            href={context.link_to("#")}
                        >
                          Page title
                        </a>
                </div>
              </div>
          </nav>
          <div class="docs__content">
              <div class="formatted-text">
                  <h1 class="docs__title">
                      {context.front_matter.title}
                  </h1>
                  {content}
              </div>
              <div class="docs__related-pages">
                  Page navigation placeholder
              </div>
          </div>
          <nav class="page__nav">
              <div class="page__item">
                  <a
                      href="#"
                      class="page__link"
                  >
                  </a>
              </div>
          </nav>
      </div>
    </LayoutMinimal>
  }
}
