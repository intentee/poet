fn template(context, props, content) {
    context.assets.add("resources/css/page-docs.css");

    component {
      <LayoutHomepage extraBodyClass="bar">
        <div>
          Hello! :D
          {content}
          <Note type="warn">
            xDD
          </Note>
        </div>
      </LayoutHomepage>
    }
}
