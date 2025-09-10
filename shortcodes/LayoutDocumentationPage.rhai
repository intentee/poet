fn render_heading(context, heading) {
  component {
    <a
      class="documentation-page__toc__link-list__link"
      href={`#${heading.id}`}
    >
      {heading.content}
    </a>
  }
}

fn render_menu(context, node, level, nested) {
  component {
    <div class={`documentation-page__menu__section documentation-page__menu__section--level-${level}`}>
      {switch level {
        0 => component {
          <div class="documentation-page__menu__section__title">
            {node.reference.front_matter.title}
          </div>
          <div class="documentation-page__menu__section__content">
            {nested}
          </div>
        },
        _ => if node.reference.front_matter.render {
          component {
            <a
              class={clsx(#{
                "documentation-page__menu__link": true,
                "documentation-page__menu__link--active": context.is_current_page(node.reference.basename),
              })}
              href={context.link_to(node.reference.basename)}
            >
              {node.reference.front_matter.title}
            </a>
            <div class="documentation-page__menu__section__content">
              {nested}
            </div>
          }
        } else {
          component {
            <div class="documentation-page__menu__section__title">
              {node.reference.front_matter.title}
            </div>
            <div class="documentation-page__menu__section__content">
              {nested}
            </div>
          }
        }
      }}
    </div>
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
          <input id="documentation-page-menu-toggle" type="checkbox">
          <label
            class="documentation-page__menu__title"
            for="documentation-page-menu-toggle"
          >{switch context.primary_collection.name {
            // _ => error(`Collection not handled: "${context.primary_collection.name}"`),
            _ => "Menu",
          }}</label>
          <div class="documentation-page__menu__content">
            {render_hierarchy(
              menu_hierarchy,
              render_menu.curry(context)
            )}
          </div>
        </div>
        <nav class="documentation-page__toc">
          <div class={clsx(#{
            "documentation-page__toc__content": true,
            "documentation-page__toc__content--empty": context.table_of_contents.headings.is_empty(),
          })}>{if !context.table_of_contents.headings.is_empty() {
            component {
              <div class="documentation-page__toc__title">
                On this page
              </div>
              <div class="documentation-page__toc__link-list">{
                context
                  .table_of_contents
                  .headings
                  .filter(|heading| heading.depth == 2)
                  .map(render_heading.curry(context))
              }</div>
            }
          }}</div>
        </nav>
        <div class="documentation-page__content">
          <div class="formatted-text">
            <h1>
              {context.front_matter.title}
            </h1>
            {content}
          </div>
          <div class="documentation-page__related-pages">
            {
              let prev = menu_hierarchy.before(context.reference.basename);

              if has(prev) {
                component {
                  <a
                    class="documentation-page__related-pages__link documentation-page__related-pages__link--prev"
                    href={prev.canonical_link}
                  >
                    <span class="documentation-page__related-pages__link__text">{prev.front_matter.title}</span>
                  </a>
                }
              }
            }
            {
              let next = menu_hierarchy.after(context.reference.basename);

              if has(next) {
                component {
                  <a
                    class="documentation-page__related-pages__link documentation-page__related-pages__link--next"
                    href={next.canonical_link}
                  >
                    <span class="documentation-page__related-pages__link__text">{next.front_matter.title}</span>
                  </a>
                }
              }
            }
          </div>
        </div>
      </div>
    </LayoutMinimal>
  }
}
