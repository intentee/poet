fn render_section(context, node, nesting_level) {
  switch nesting_level {
    0 => component {
      <div class="docs__nav-section-title">
        {node.reference.front_matter.title}
      </div>
      {
        let rendered = [];

        for child in node.children {
          rendered.push(render_section(context, child, nesting_level + 1));
        }

        rendered
      }
    },
    _ => component {
      <div class="docs__nav-section">
        <a
          class="docs__nav-link"
          href={context.link_to(node.reference.basename)}
        >
          {node.reference.front_matter.title}
        </a>
      </div>
    }
  }
}

fn template(context, props, content) {
  context.assets.add("resources/css/layout-documentation-page.css");

  component {
    <LayoutMinimal>
      <div class="docs">
        <input checked type="checkbox" id="docs-nav-toggle">
        <nav class="docs__nav">
          <label class="docs__nav__toggle" for="docs-nav-toggle">
            Mobile docs navigation placeholder
          </label>
          <div class="docs__nav__content">{
            let root_documents = context.collection("documentation_pages").hierarchy;
            let rendered = [];

            for root_document in root_documents {
              rendered.push(render_section(context, root_document, 0));
            }

            rendered
          }</div>
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
