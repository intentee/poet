use markdown::mdast::Node;
use slug::slugify;

use poet_mdx::find_text_content_in_mdast::find_text_content_in_mdast;

pub fn mdast_children_to_heading_id(children: &[Node]) -> String {
    slugify(
        children
            .iter()
            .map(find_text_content_in_mdast)
            .collect::<String>(),
    )
}
