fn template(assets, content, props) {`
  <div
    alt="${props.alt}"
    class="formatted-text__admonition formatted-text__admonition--${props.type}"
  >
    ${content}
  </div>
`}
