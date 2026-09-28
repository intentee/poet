use poet_assets::register_asset_rhai_types::register_asset_rhai_types;
use rhai::Engine;

use crate::author::Author;
use crate::author_collection::AuthorCollection;
use crate::author_data::AuthorData;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_hierarchy::ContentDocumentHierarchy;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_tree_node::ContentDocumentTreeNode;
use crate::rhai_helpers::render_hierarchy;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

pub fn register_content_rhai_types(engine: &mut Engine) {
    register_asset_rhai_types(engine);
    engine.build_type::<Author>();
    engine.build_type::<AuthorCollection>();
    engine.build_type::<AuthorData>();
    engine.build_type::<ContentDocumentCollectionRanked>();
    engine.build_type::<ContentDocumentComponentContext>();
    engine.build_type::<ContentDocumentFrontMatter>();
    engine.build_type::<ContentDocumentHierarchy>();
    engine.build_type::<ContentDocumentReference>();
    engine.build_type::<ContentDocumentTreeNode>();
    engine.build_type::<Heading>();
    engine.build_type::<TableOfContents>();
    engine.register_fn("render_hierarchy", render_hierarchy);
}
