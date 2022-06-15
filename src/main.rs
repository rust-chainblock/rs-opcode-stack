use crate::stack::StackI32;

mod stack;
fn main() {

    let mut stack : StackI32 = StackI32::new();
    stack.push(100);
    stack.push(101);
    stack.pop();

    println!("{:?}", stack);
}
