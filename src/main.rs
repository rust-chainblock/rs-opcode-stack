use crate::stack::{ StackI32, StackItem };

mod stack;
fn main() {

    let mut stack : StackI32 = StackI32::new();
    stack.push(StackItem::Unknown("<calldata_message>".to_owned()));
    stack.push(StackItem::Num(100));
    stack.pop();

    println!("{:?}", stack);
}
