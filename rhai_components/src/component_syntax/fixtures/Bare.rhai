fn template(context, props, content) {
  component {
    <span data-disabled={if "disabled" in props { "yes" } else { "no" }}>{content}</span>
  }
}
