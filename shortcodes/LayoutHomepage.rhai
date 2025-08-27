fn template(context, props, content) {
  context.assets.add("resources/css/page-home.css");

  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
          <title>{
            let base_title = "Paddler, LLMOps platform";

            if context.front_matter.title.is_empty() {
              base_title
            } else {
              `${content.front_matter.title} - ${base_title}`
            }
          }</title>
          {context.assets.render()}
        </head>
        <body class={props.extraBodyclass}>
          {content}
        </body>
    </html>
  }
}
