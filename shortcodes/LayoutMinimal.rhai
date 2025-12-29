fn template(context, props, content) {
  let author_names = context.authors.map(|a| a.data.name).join(", ");

  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Poet</title>
      </head>
      <body>
        <PrimaryNavigation>test</PrimaryNavigation>

        Hello!

        {content}

        <p>Authors: {author_names}</p>
      </body>
    </html>
  }
}
