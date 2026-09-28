fn template(context, props, content) {
  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
          <title>Poet</title>
        </head>
        <body>
          {content}
        </body>
    </html>
  }
}
