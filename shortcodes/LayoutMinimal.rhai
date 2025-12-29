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

        <p>Authors:
          {
            let ret = [];

            for author in context.authors {
              ret.push(component {
                <span>{author.basename}</span>
                <span>{author.data.name}</span>
              });
            }

            ret
          }
        </p>
      </body>
    </html>
  }
}
