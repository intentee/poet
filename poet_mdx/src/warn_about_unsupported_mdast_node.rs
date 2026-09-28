use log::warn;
use markdown::mdast::Node;

pub fn warn_about_unsupported_mdast_node(mdast: &Node) {
    warn!("Markdown node is not supported and was skipped: {mdast:?}");
}
