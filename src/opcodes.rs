use crate::stack::{Stack, StackItem};

fn concat_args(operation: &'static str, args: Vec<StackItem>) -> String {
  let mut concat_str = String::from(operation);
  let args_str: Vec<String> = args.iter().map(|x| match &x {
    StackItem::Known(val) =>  val.to_string(),
    StackItem::Unknown(val) => val.to_owned()
  }).collect();

  concat_str.push('(');
  let joined = args_str.join(", ");
  
  concat_str.push_str(joined.as_str());
  concat_str.push(')');
  concat_str
}

impl Stack {
  pub fn add(&mut self) -> Result<(), &'static str> {
    let a = match self.pop() {
      Some(item) => item,
      None => return Result::Err("Stack empty"),
    };

    let b = match self.pop() {
      Some(item) => item,
      None => return Result::Err("Stack empty"),
    };

    let to_push = match &a {
      StackItem::Known(val_a) => match &b {
        StackItem::Known(val_b) => StackItem::Known(val_a + val_b),
        StackItem::Unknown(_) => {
          let concatted = concat_args("add", vec![a, b]);
          StackItem::Unknown(concatted)
        }
      },
      StackItem::Unknown(_) => StackItem::Unknown(
        concat_args("add", vec![a, b])
      ) 
    };

    self.push(to_push);
    Result::Ok(())
  }
}
