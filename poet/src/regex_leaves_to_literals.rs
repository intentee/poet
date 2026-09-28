use tantivy::query_grammar::Delimiter;
use tantivy::query_grammar::UserInputAst;
use tantivy::query_grammar::UserInputLeaf;
use tantivy::query_grammar::UserInputLiteral;

fn regex_leaf_to_literal(user_input_leaf: UserInputLeaf) -> UserInputLeaf {
    match user_input_leaf {
        UserInputLeaf::Regex { field, pattern } => UserInputLeaf::Literal(UserInputLiteral {
            field_name: field,
            phrase: format!("/{pattern}/"),
            delimiter: Delimiter::None,
            slop: 0,
            prefix: false,
        }),
        other_leaf => other_leaf,
    }
}

pub fn regex_leaves_to_literals(user_input_ast: UserInputAst) -> UserInputAst {
    match user_input_ast {
        UserInputAst::Clause(clauses) => UserInputAst::Clause(
            clauses
                .into_iter()
                .map(|(occur, clause_ast)| (occur, regex_leaves_to_literals(clause_ast)))
                .collect(),
        ),
        UserInputAst::Boost(boosted_ast, boost) => {
            UserInputAst::Boost(Box::new(regex_leaves_to_literals(*boosted_ast)), boost)
        }
        UserInputAst::Leaf(user_input_leaf) => {
            UserInputAst::Leaf(Box::new(regex_leaf_to_literal(*user_input_leaf)))
        }
    }
}
