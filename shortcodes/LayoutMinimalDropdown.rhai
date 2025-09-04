fn template(context, props, content) {
  component {
    <div class="page-navigation__dropdown">
      <div class="page-navigation__dropdown__label">
        {props.label}
      </div>
      <div class="page-navigation__dropdown__content">
        <div class="page-navigation__dropdown__links">
          {content}
        </div>
      </div>
    </div>
  }
}
