fn template(context, props, content) {
  component {
    <html>
      <head><title>{context.front_matter.title}</title></head>
      <body>
        <p>{context.front_matter.description}</p>
        <span>{context.reference.canonical_link}</span>
        <span>{context.reference.basename}</span>
        <span>reftitle:{context.reference.front_matter.title}</span>
        <span>watching:{context.is_watching}</span>
        <span>props:{context.front_matter.props.len()}</span>
        <span>authors:{context.available_authors.len()}</span>
        {if context.front_matter.render { "<i>is-rendered</i>" } else { "" }}
        {
          let names = "";

          for author in context.authors {
            names += "<b>" + author.basename + "=" + author.data.name + "</b>";
          }

          names
        }
        <ul>
        {
          let toc = "";

          for heading in context.table_of_contents.headings {
            toc += "<li>" + heading.id + ":" + heading.content + ":" + heading.depth + "</li>";
          }

          toc
        }
        </ul>
        {
          let docs = context.collection("docs");
          let heading = "<h2>" + docs.name + "</h2>";

          heading + render_hierarchy(docs.hierarchy, |node, level, children| {
            "<li>" + node.reference.basename + " in " + node.collection_name + " kids " + node.children.len() + children + "</li>"
          })
        }
        {content}
      </body>
    </html>
  }
}
