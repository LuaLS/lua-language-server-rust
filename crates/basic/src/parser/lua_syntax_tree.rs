use emmylua_parser::LuaSyntaxTree as EmmyLuaSyntaxTree;
use mlua::prelude::*;

use super::lua_node::LuaNodeWrapper;

pub struct LuaSyntaxTree {
    tree: EmmyLuaSyntaxTree,
}

impl LuaSyntaxTree {
    pub fn new(tree: EmmyLuaSyntaxTree) -> Self {
        Self { tree }
    }

    pub fn get_root(&self) -> LuaNodeWrapper {
        LuaNodeWrapper::new(self.tree.get_red_root().clone())
    }
}

impl LuaUserData for LuaSyntaxTree {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getRoot", |_, this, ()| Ok(this.get_root()));
        // methods.add_method("get_chunk_node", |_, this, ()| Ok(this.get_chunk_node()));
    }
}
