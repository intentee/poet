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

        {content}
      </body>
    </html>
  }
}
