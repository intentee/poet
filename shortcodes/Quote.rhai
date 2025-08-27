fn template(context, props, content) {
  component{
    <h1 class="quote-title">{props.title}></h1>
      <p class="quote-content"></p>
    <br>
        {content}
  }
}
