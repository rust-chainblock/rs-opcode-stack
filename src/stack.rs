#[derive(Debug)]
pub struct StackI32 {
    items: Vec<i32>
}

impl StackI32 {
    pub fn new() -> StackI32 {
        StackI32 { 
            items: Vec::new()
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    pub fn push(&mut self, value: i32) {
        self.items.push(value);
    }
}