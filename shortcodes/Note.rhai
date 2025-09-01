fn template(context, props, content) {
  component {
    <div
      class={if "type" in props {
        `formatted-text__note formatted-text__note--${props.type}`
      } else {
        "formatted-text__note"
      }}
    >
      {content}
    </div>
  }
}
