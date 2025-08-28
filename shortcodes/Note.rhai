fn template(context, props, content) {
  component {
    <div
      class={if "type" in props {
        `note note--${props.type}`
      } else {
        "note"
      }}
    >
      {content}
    </div>
  }
}
