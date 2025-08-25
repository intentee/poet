fn template(context, content, props) {
    context.assets.add("resources/css/page-docs.css");

    component {
      <LayoutHomepage extraBodyClass="bar">
        <div>
          Hello! :D
          {content}
          <Admonition alt="something" type="warn">
            xDD
          </Admonition>
        </div>
      </LayoutHomepage>
    }
}
