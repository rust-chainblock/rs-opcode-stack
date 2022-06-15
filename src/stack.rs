// Some stack items will depend on the call context, and in its current
// version I'll be representing them with StackItem::Unknown
#[derive(Debug)]
pub enum StackItem {
    Num(i32),
    Unknown(String)
}

#[derive(Debug)]
pub struct StackI32 {
    items: Vec<StackItem>
}

impl StackI32 {
    pub fn new() -> StackI32 {
        StackI32 { 
            items: Vec::new()
        }
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.items.pop()
    }

    pub fn push(&mut self, value: StackItem) {
        self.items.push(value);
    }
}