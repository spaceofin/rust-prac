use std::{str::FromStr, panic};

// Bar and Baz are non-parameterized enum variants
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn if_let() {
  println!("-----Option-----");
  let number_1 = Some(7);
  let number_2: Option<i32> = Some(10);
  let number_3: Option<i32> = None;
  let i_like_numbers = true;

  if let Some(i) = number_1 {
      println!("Matched {:?}!", i);
  } else {
      // Destructure failed. Change to the failure case.
      println!("Didn't match a number. Let's go with a letter!");
  }

  if let Some(i) = number_2 {
      println!("Matched {:?}!", i);
  // Destructure failed. Evaluate an `else if` condition to see if the
  // alternate failure branch should be taken:
  } else if i_like_numbers {
      println!("I like numbers!");
  } else {
      // The condition evaluated false. This branch is the default:
      println!("I don't like numbers");
  }

  if let Some(i) = number_3 {
      println!("Matched {:?}!", i);
  } else {
      println!("Didn't match a number.");
  }

  println!("-----enum-----");
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);
  
  if let Foo::Bar = a {
      println!("a is foobar");
  }
  
  if let Foo::Bar = b {
      println!("b is foobar");
  }
  
  // Variable c matches Foo::Qux which has a value
  // Similar to Some() in the previous example
  if let Foo::Qux(value) = c {
      println!("c is {}", value);
  }

  // Binding also works with `if let`
  if let Foo::Qux(value @ 100) = c {
      println!("c is one hundred. value is {}", value);
  }
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn while_let(){
  println!("-----while_let_increment-----");
  // Option<i32>
  let mut optional = Some(0);
    
  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("`i` is `{:?}`. Try again.", i);
      optional = Some(i + 1);
    }
    // doesn't require explicitly handling the failing case.
  }
  // `while let` does not have additional optional `else`/`else if` clauses.
}

pub fn let_demo(){
  if_let();

  let result = panic::catch_unwind(|| { get_count_item("3 chairs")});
  match result {
    Ok(count_item) => println!("{:?}", count_item),
    Err(_) => println!("get_count_item panicked!"),
  }

  while_let();
}