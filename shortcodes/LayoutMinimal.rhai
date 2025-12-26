fn template(context, props, content) {
  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Poet</title>
      </head>
      <body>
        <PrimaryNavigation>test</PrimaryNavigation>

        Hello!

        <p>Authors: {context.authors}</p>

        {content}
      </body>
    </html>
  }
}
