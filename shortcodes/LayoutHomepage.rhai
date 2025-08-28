fn template(context, props, content) {

  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
          <title>{
            let base_title = "Paddler, LLMOps platform";

            if context.front_matter.title.is_empty() {
              base_title
            } else {
              `${context.front_matter.title} - ${base_title}`
            }
          }</title>
        </head>
        <body>
          {content}
        </body>
    </html>
  }
}
