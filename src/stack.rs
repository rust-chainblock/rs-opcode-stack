// Some stack items will depend on the call context, and in its current
// version I'll be representing them with StackItem::Unknown
#[derive(Debug)]
pub enum StackItem {
  Known(i32),
  Unknown(String),
}

#[derive(Debug)]
pub struct Stack {
  items: Vec<StackItem>,
}

impl Stack {
  pub fn new() -> Stack {
    Stack { items: Vec::new() }
  }

  pub fn pop(&mut self) -> Option<StackItem> {
    self.items.pop()
  }

  pub fn push(&mut self, value: StackItem) {
    self.items.push(value);
  }
}
