fn template(context, props, content) {
  let author_name = if !context.authors.is_empty() { context.authors[0].name } else { "" };

  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Poet</title>
      </head>
      <body>
        <p>Author: {author_name}</p>
        {content}
      </body>
    </html>
  }
}
