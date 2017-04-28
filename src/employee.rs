use std::vec::Vec;
use std::fmt;

use toolbelt::{Toolbelt, Item};

pub struct Employee<'a> {
    pub name: &'static str,
    pub age: i32,
    pub belt: Toolbelt<'a>,
}

impl<'a> Employee<'a> {
    pub fn add_tool(&mut self, tool: &'a Item) {
        self.belt.tools.push(tool);
    }

    pub fn empty_toolbelt(&mut self) {
        self.belt.empty();
    }
}

impl<'a> fmt::Display for Employee<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.age, self.belt)

    }
}
