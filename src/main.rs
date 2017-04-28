use std::vec::Vec;
use std::fmt;

pub mod employee;
pub mod toolbelt;

use employee::Employee;
use toolbelt::{Toolbelt, Item};

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
