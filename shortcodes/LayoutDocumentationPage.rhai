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
          class={clsx(#{
            "docs__nav-link": true,
            "docs__nav-link--active": context.is_current_page(node.reference.basename),
          })}
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

  let menu_hierarchy = context.primary_collection.hierarchy;

  component {
    <LayoutMinimal>
      {render_hierarchy(
        menu_hierarchy,
        render_menu.curry(context)
      )}
      <h1>
        {context.front_matter.title}
      </h1>
      {content}
      {
        let prev = menu_hierarchy.before(context.reference.basename);

        if prev != () {
          component {
            <a
              class="docs__related-pages__link docs__related-pages__link--prev"
              href={prev.canonical_link}
            >
              <span class="docs__related-pages__link__text">{prev.front_matter.title}</span>
            </a>
          }
        }
      }
      {
        let next = menu_hierarchy.after(context.reference.basename);

        if next != () {
          component {
            <a
              class="docs__related-pages__link docs__related-pages__link--next"
              href={next.canonical_link}
            >
              <span class="docs__related-pages__link__text">{next.front_matter.title}</span>
            </a>
          }
        }
      }
      {
        let rendered = [];

        for heading in context.table_of_contents.headings {
          if heading.depth == 2 {
            rendered.push(component {
              <div class="page__item">
                <a class="page__link" href={`#${heading.id}`}>
                  {heading.content}
                </a>
              </div>
            });
          }
        }

        rendered
      }
    </LayoutMinimal>
  }
}
