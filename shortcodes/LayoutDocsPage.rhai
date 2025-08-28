fn template(context, props, content) {
  context.assets.add("resources/css/page-docs.css");

  component {
    <LayoutHomepage>
      <div class="page-docs">
        {content}
        <Note>
          Notka z klasa note
        </Note>
      </div>
    </LayoutHomepage>
  }
}
