import "LayoutHomepage" as homepage_layout;

fn template(assets, content, props) {
    assets.styles.add("resources/css/page-docs.css");

    homepage_layout::template(assets, `
        <div>
            Hello! :D
            ${content}
        </div>
    `)
}
