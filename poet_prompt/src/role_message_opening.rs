use markdown::mdast::Node;
use markdown::mdast::Text;

const ROLE_MESSAGE_SEPARATOR: char = ':';

pub struct RoleMessageOpening<'mdast> {
    pub first_content: &'mdast str,
    pub rest: &'mdast [Node],
}

impl<'mdast> RoleMessageOpening<'mdast> {
    #[must_use]
    pub fn parse(message: &'mdast [Node]) -> Option<Self> {
        match message {
            [Node::Text(Text { value, .. }), rest @ ..] => value
                .trim_start()
                .strip_prefix(ROLE_MESSAGE_SEPARATOR)
                .map(|first_content| Self {
                    first_content,
                    rest,
                }),
            _ => None,
        }
    }
}
