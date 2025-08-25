fn template(assets, content, props) {
  component {
    <!DOCTYPE html>
    <html lang="en">
        <head>
            {assets.render()}
        </head>
        <body>
            {content}
        </body>
    </html>
  }
}
