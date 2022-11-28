use std::{collections::HashMap, ptr};

use super::Symbol;

#[derive(Debug)]
pub(crate) struct Env {
    table: HashMap<String, Symbol>,
    parent: *const Env,
    // children 用于保存所有权
    children: Vec<Env>,
}

impl Env {
    pub(crate) fn new(parent: &Env) -> Self {
        let child = Self {
            table: HashMap::new(),
            parent: parent as *const Env,
            children: Vec::new(),
        };
        child
    }

    pub(crate) fn new_global() -> Self {
        let table = HashMap::new();
        Self {
            table,
            parent: ptr::null_mut(),
            children: Vec::new(),
        }
    }

    pub(crate) fn store_env(&mut self, child: Env) {
        self.children.push(child);
    }

    pub(crate) fn put_symbol(&mut self, symbol_name: &str, symbol: Symbol) {
        self.table.insert(String::from(symbol_name), symbol);
    }

    pub(crate) fn get_symbol(&self, symbol_name: &str) -> Option<&Symbol> {
        // 先在本地查找
        let sym = self.table.get(symbol_name);

        // 没有则去上一个域查找
        if sym.is_none() {
            unsafe {
                if let Some(parent) = self.parent.as_ref() {
                    parent.get_symbol(symbol_name)
                } else {
                    None
                }
            }
        } else {
            return sym;
        }
    }
}
