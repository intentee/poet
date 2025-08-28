fn template(context, props, content) {
  if context.is_watching {
    context.assets.add("resources/ts/global-live-reload.ts");
  }

  context.assets.add("resources/css/page-common.css");
  context.assets.add("resources/css/page-home.css");

  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
          <title>{
            let base_title = "Poet";

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
