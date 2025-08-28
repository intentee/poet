fn template(context, props, content) {
  context.assets.add("resources/css/page-docs.css");

  component {
    <LayoutHomepage>
      <div class="page-docs">
        {content}
      </div>
    </LayoutHomepage>
  }
}
