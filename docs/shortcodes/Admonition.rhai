component Admonition(content, type) {
  <div
    alt="{type}"
    class="formatted-text__admonition formatted-text__admonition--{type}"
  >
    {content}
    <Figure foo="bar" />
  </div>
}
