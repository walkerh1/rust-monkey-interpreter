use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

mod tests;

#[derive(Debug, PartialEq)]
pub enum SymbolScope {
    Global,
    Local,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: String,
    pub scope: SymbolScope,
    pub index: u32,
}

impl Symbol {
    pub fn new(name: &str, scope: SymbolScope, index: u32) -> Self {
        Symbol {
            name: name.to_string(),
            scope,
            index,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SymbolTable {
    pub outer: Option<Box<SymbolTable>>,
    store: HashMap<String, Rc<Symbol>>,
    pub num_definitions: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            outer: None,
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn new_enclosed(table: SymbolTable) -> Self {
        let mut new = Self::new();
        new.outer = Some(Box::new(table));
        new
    }

    pub fn define(&mut self, name: String) -> Rc<Symbol> {
        let scope = match &self.outer {
            Some(_) => SymbolScope::Local,
            None => SymbolScope::Global,
        };
        let symbol = Rc::new(Symbol::new(name.as_str(), scope, self.num_definitions));
        self.store.insert(name, Rc::clone(&symbol));
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: String) -> Option<Rc<Symbol>> {
        let symbol = self.store.get(&name).cloned();
        if let Some(sym) = symbol {
            return Some(Rc::clone(&sym));
        } else if let Some(outer) = &mut self.outer {
            if let Some(object) = outer.resolve(name) {
                return Some(object);
            }
        }
        None
    }
}
