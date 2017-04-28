use std::vec::Vec;
use std::fmt;

struct Employee<'a> {
    name: &'static str,
    age: i32,
    belt: Toolbelt<'a>,
}

impl<'a> Employee<'a> {
    fn add_tool(&mut self, tool: &'a Item) {
        self.belt.tools.push(tool);
    }
}

impl<'a> fmt::Display for Employee<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.age, self.belt)

    }
}

struct Toolbelt<'a> {
    tools: Vec<&'a Item>,
}

impl<'a> Toolbelt<'a> {
    fn empty(&mut self) {
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

struct Item {
    name: &'static str,
}

fn main() {
    let screwdriver = Item { name: "Screwdriver" };
    let wrench = Item { name: "Wrench" };
    let hammer = Item { name: "Hammer" };

    let toolbelt = Toolbelt { tools: vec![&wrench, &hammer] };

    let mut e = Employee {
        name: "John Smith",
        age: 52,
        belt: toolbelt,
    };

    println!("{}", e);
    e.add_tool(&screwdriver);
    println!("{}", e);
    e.belt.empty();
    println!("{}", e);
}
