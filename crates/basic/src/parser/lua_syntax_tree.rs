use emmylua_parser::LuaSyntaxTree as EmmyLuaSyntaxTree;
use mlua::prelude::*;
use rowan::TextSize;

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

    pub fn get_line_col(&self, offset: usize) -> Option<(usize, usize)> {
        let offset = TextSize::from(offset as u32);
        let (line, col) = self.tree.get_line_col(offset)?;
        Some((line, col))
    }

    pub fn get_offset(&self, line: usize, col: usize) -> Option<usize> {
        let offset = self.tree.get_offset(line, col)?;
        Some(offset.into())
    }
}

impl LuaUserData for LuaSyntaxTree {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getRoot", |_, this, ()| Ok(this.get_root()));
        // methods.add_method("get_chunk_node", |_, this, ()| Ok(this.get_chunk_node()));
        methods.add_method("getLineCol", |lua, this, offset: usize| {
            let (line, col) = this.get_line_col(offset).unwrap();
            let table = lua.create_table()?;
            table.set(1, line)?;
            table.set(2, col)?;
            Ok(table)
        });
        methods.add_method("getOffset", |_, this, (line, col): (usize, usize)| {
            Ok(this.get_offset(line, col))
        });
    }
}
