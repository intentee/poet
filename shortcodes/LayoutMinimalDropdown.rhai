fn template(context, props, content) {
  component {
    <div
      class={clsx(#{
        "page-navigation__dropdown": true,
        "page-navigation__dropdown--collection-active": context.belongs_to(props.collection),
      })}
    >
      <a
        class="page-navigation__dropdown__label"
        href={props.href}
      >
        {props.label}
      </a>
      <div class="page-navigation__dropdown__content">
        <div class="page-navigation__dropdown__links">
          {content}
        </div>
      </div>
    </div>
  }
}
