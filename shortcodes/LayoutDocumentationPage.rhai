fn figure_out_collection(context) {
  for collection in context.front_matter.collections {
    if "documentation_pages" == collection.name {
      return "documentation_pages";
    }
  }

  for collection in context.front_matter.collections {
    if "api_pages" == collection.name {
      return "api_pages";
    }
  }

  "*not_found*"
}

fn render_menu(context, node, level, nested) {
  switch level {
    0 => component {
      <div class="docs__nav-section-title">
        {node.reference.front_matter.title}
      </div>
      {nested}
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
  // context.speculation.next(context.collection("documentation_pages").hierarchy.next_after(context.basename));

  component {
    <LayoutMinimal>
      <div class="docs">
        <input checked type="checkbox" id="docs-nav-toggle">
        <nav class="docs__nav">
          <label class="docs__nav__toggle" for="docs-nav-toggle">
            Mobile docs navigation placeholder
          </label>
          <div class="docs__nav__content">
            {render_hierarchy(
              context.collection(figure_out_collection(context)).hierarchy,
              render_menu.curry(context)
            )}
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
