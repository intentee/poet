fn template(context, content, props) {
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
