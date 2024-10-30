use emmylua_parser::{LuaSyntaxKind, LuaSyntaxNode, LuaSyntaxToken, LuaTokenKind};
use mlua::prelude::*;

pub enum LuaNodeWrapper {
    Node(LuaSyntaxNode),
    Token(LuaSyntaxToken),
}

impl LuaNodeWrapper {
    pub fn new(node: LuaSyntaxNode) -> Self {
        Self::Node(node)
    }

    pub fn from_node_or_token(
        node_or_token: rowan::NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>,
    ) -> Self {
        match node_or_token {
            rowan::NodeOrToken::Node(node) => Self::Node(node),
            rowan::NodeOrToken::Token(token) => Self::Token(token),
        }
    }
}

impl LuaUserData for LuaNodeWrapper {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("isNode", |_, this| match this {
            LuaNodeWrapper::Node(_) => Ok(true),
            LuaNodeWrapper::Token(_) => Ok(false),
        });
        fields.add_field_method_get("isToken", |_, this| match this {
            LuaNodeWrapper::Node(_) => Ok(false),
            LuaNodeWrapper::Token(_) => Ok(true),
        });
        fields.add_field_method_get("kind", |_, this| match this {
            LuaNodeWrapper::Node(node) => {
                let kind: LuaSyntaxKind = node.kind().into();
                Ok(kind as u16)
            }
            LuaNodeWrapper::Token(token) => {
                let kind: LuaTokenKind = token.kind().into();
                Ok(kind as u16)
            }
        });
        fields.add_field_method_get("kindText", |_, this| {
            let text = match this {
                LuaNodeWrapper::Node(node) => {
                    let kind: LuaSyntaxKind = node.kind().into();
                    format!("{:?}", kind)
                }
                LuaNodeWrapper::Token(token) => {
                    let kind: LuaTokenKind = token.kind().into();
                    format!("{:?}", kind)
                }
            };

            Ok(text)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getText", |_, this, ()| {
            let text = match this {
                LuaNodeWrapper::Node(node) => node.text().to_string(),
                LuaNodeWrapper::Token(token) => token.text().to_string(),
            };
            Ok(text)
        });
        methods.add_method("getRange", |lua, this, ()| {
            let range = match this {
                LuaNodeWrapper::Node(node) => node.text_range(),
                LuaNodeWrapper::Token(token) => token.text_range(),
            };
            let table = lua.create_table()?;
            let line: u32 = range.start().into();
            let col: u32 = range.start().into();
            table.set("start", line)?;
            table.set("end", col)?;
            Ok(table)
        });
        methods.add_method("getChildren", |lua, this, ()| {
            let children = match this {
                LuaNodeWrapper::Node(node) => node
                    .children_with_tokens()
                    .filter_map(|it| match it {
                        rowan::NodeOrToken::Node(node) => Some(LuaNodeWrapper::Node(node)),
                        rowan::NodeOrToken::Token(token) => Some(LuaNodeWrapper::Token(token)),
                    })
                    .collect(),
                LuaNodeWrapper::Token(_) => vec![],
            };

            Ok(children)
        });

        methods.add_method("dump", |_, this, ()| {
            let dump = match this {
                LuaNodeWrapper::Node(node) => format!("{:#?}", node),
                LuaNodeWrapper::Token(token) => format!("{:#?}", token),
            };
            Ok(dump)
        });

        methods.add_method("getParent", |_, this, ()| {
            let parent = match this {
                LuaNodeWrapper::Node(node) => node.parent().map(LuaNodeWrapper::Node),
                LuaNodeWrapper::Token(token) => token.parent().map(LuaNodeWrapper::Node),
            };
            Ok(parent)
        });

        methods.add_method("getPrevSibling", |_, this, ()| {
            let prev_sibling = match this {
                LuaNodeWrapper::Node(node) => node.prev_sibling().map(LuaNodeWrapper::Node),
                LuaNodeWrapper::Token(token) => match token.prev_sibling_or_token() {
                    Some(rowan::NodeOrToken::Node(node)) => Some(LuaNodeWrapper::Node(node)),
                    Some(rowan::NodeOrToken::Token(token)) => Some(LuaNodeWrapper::Token(token)),
                    None => None,
                },
            };
            Ok(prev_sibling)
        });

        methods.add_method("getNextSibling", |_, this, ()| {
            let next_sibling = match this {
                LuaNodeWrapper::Node(node) => node.next_sibling().map(LuaNodeWrapper::Node),
                LuaNodeWrapper::Token(token) => match token.next_sibling_or_token() {
                    Some(rowan::NodeOrToken::Node(node)) => Some(LuaNodeWrapper::Node(node)),
                    Some(rowan::NodeOrToken::Token(token)) => Some(LuaNodeWrapper::Token(token)),
                    None => None,
                },
            };
            Ok(next_sibling)
        });

        methods.add_method("getDescendants", |_, this, ()| {
            let descendants = match this {
                LuaNodeWrapper::Node(node) => node
                    .descendants_with_tokens()
                    .map(LuaNodeWrapper::from_node_or_token)
                    .collect(),
                LuaNodeWrapper::Token(_) => vec![],
            };
            Ok(descendants)
        });

        methods.add_method("getAncestors", |_, this, ()| {
            let ancestors: Vec<LuaNodeWrapper> = match this {
                LuaNodeWrapper::Node(node) => node.ancestors().map(LuaNodeWrapper::Node).collect(),
                LuaNodeWrapper::Token(token) => {
                    token.parent_ancestors().map(LuaNodeWrapper::Node).collect()
                }
            };

            Ok(ancestors)
        });
    }
}
