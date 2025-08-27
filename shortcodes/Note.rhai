fn template(context, props, content) {
  component {
    <div class={`formatted-text__admonition formatted-text__admonition--${props.type}`}>
      {content}
    </div>
  }
}
