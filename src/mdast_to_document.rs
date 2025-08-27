#![allow(clippy::all)]
#![allow(warnings)]

use log::info;
use markdown::mdast::Node;
use markdown::mdast::Root;

pub fn mdast_to_document(mdast: Node) {
    match mdast {
        Node::Root(Root { ref children, .. }) => {
            for child in children {
                info!("MDAST Child Node: {:?}", child);
            }
        }
        _ => {}
    }
}
