import "LayoutHomepage";

fn template(assets, content, props) {
    assets.styles.add("resources/css/page-docs.css");

    LayoutHomepage::template(assets, `
        <div>
            Hello! :D
            ${content}
        </div>
    `)
}
