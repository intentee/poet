fn template(context, content, props) {
  component {
    <div
      alt={props.alt}
      class="formatted-text__admonition formatted-text__admonition--{props.type}"
    >
      {content}
    </div>
  }
}
