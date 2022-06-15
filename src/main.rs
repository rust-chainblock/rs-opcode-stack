use crate::stack::{Stack, StackItem};

mod opcodes;
mod stack;
fn main() {
    let mut stack: Stack = Stack::new();
    stack.push(StackItem::Unknown(String::from("CDM")));
    stack.push(StackItem::Known(100));
    stack.add().unwrap();

    println!("{:?}", stack);
}
