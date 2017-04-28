use std::vec::Vec;
use std::fmt;

pub struct Toolbelt<'a> {
    pub tools: Vec<&'a Item>,
}

impl<'a> Toolbelt<'a> {
    pub fn empty(&mut self) {
        self.tools.clear();
    }
}

impl<'a> fmt::Display for Toolbelt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tools = String::new();
        for tool in &self.tools {
            tools.push_str(tool.name);
            tools.push_str(" ,");
        }
        write!(f, "({})", tools)
    }
}

pub struct Item {
    pub name: &'static str,
}
