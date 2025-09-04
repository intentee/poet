fn render_menu(context, node, level, nested) {
  switch level {
    0 => component {
      <div class="documentation-page__menu-section">
        <div class="documentation-page__menu-section__title">
          {node.reference.front_matter.title}
        </div>
        <div class="documentation-page__menu-section__content">
          {nested}
        </div>
      </div>
    },
    _ => component {
      <a
        class={clsx(#{
          "documentation-page__menu__link": true,
          "documentation-page__menu__link--active": context.is_current_page(node.reference.basename),
        })}
        href={context.link_to(node.reference.basename)}
      >
        {node.reference.front_matter.title}
      </a>
    }
  }
}

fn template(context, props, content) {
  context.assets.add("resources/css/layout-documentation-page.css");
  // context.speculation.next(context.collection("documentation_pages").hierarchy.next_after(context.basename));

  let menu_hierarchy = context.primary_collection.hierarchy;

  component {
    <LayoutMinimal>
      <div class="documentation-page">
        <div class="documentation-page__menu">
          {render_hierarchy(
            menu_hierarchy,
            render_menu.curry(context)
          )}
        </div>
        <nav class="documentation-page__toc">{
          let rendered = [];

          for heading in context.table_of_contents.headings {
            if heading.depth == 2 {
              rendered.push(component {
                <a href={`#${heading.id}`}>
                  {heading.content}
                </a>
              });
            }
          }

          rendered
        }</nav>
        <div class="documentation-page__content">
          <div class="formatted-text">
            <h1>
              {context.front_matter.title}
            </h1>
            {content}
          </div>
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
        </div>
      </div>
    </LayoutMinimal>
  }
}
