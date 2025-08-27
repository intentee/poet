fn template(context, props, content) {
  component {
    <div class={`note note--${props.type}`}>
      {content}
    </div>
  }
}
