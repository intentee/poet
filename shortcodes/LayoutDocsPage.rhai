fn template(context, props, content) {
    component {
      <LayoutHomepage>
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
