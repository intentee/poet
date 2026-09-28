fn template(context, props, content) {
  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Test</title>
      </head>
      <body>
        <PrimaryNavigation>nav</PrimaryNavigation>
        {content}
        <ul>
          {
            let ret = [];

            for author in context.authors {
              ret.push(component {
                <li>{author.data.name}</li>
              });
            }

            ret
          }
        </ul>
      </body>
    </html>
  }
}
