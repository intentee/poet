fn template(context, props, content) {
  context.assets.add("resources/css/page-home.css");

  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
          <title>{
            let base_title = "Poet static site generator";

            if context.front_matter.title.is_empty() {
              base_title
            } else {
              `${context.front_matter.title} - ${base_title}`
            }
          }</title>
          {context.assets.render()}
        </head>
        <body>
          {content}
        </body>
    </html>
  }
}
