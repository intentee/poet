fn template(assets, content, props) {
    assets.styles.add("resources/css/page-docs.css");

    component {
      <LayoutHomepage>
        <div>
          Hello! :D
          {content}
        </div>
      </LayoutHomepage>
    }
}
