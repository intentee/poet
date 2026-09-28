use markdown::Constructs;
use markdown::ParseOptions;
use markdown::mdast::Node;
use markdown::to_mdast;

use crate::mdx_error::MdxError;

pub fn string_to_mdast(contents: &str) -> Result<Node, MdxError> {
    to_mdast(
        contents,
        &ParseOptions {
            constructs: Constructs {
                autolink: false,
                code_indented: false,
                frontmatter: true,
                html_flow: false,
                math_flow: true,
                math_text: true,
                mdx_expression_flow: true,
                mdx_expression_text: true,
                mdx_jsx_flow: true,
                mdx_jsx_text: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::default()
        },
    )
    .map_err(|message| MdxError::ParseMarkdown { message })
}
