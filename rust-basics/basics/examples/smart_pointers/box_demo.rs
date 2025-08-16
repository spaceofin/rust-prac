use std::fmt;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons(head, tail) => {
                write!(f, "{}", head)?;
                match **tail {
                    Nil => write!(f, " → Nil"),
                    _ => write!(f, " → {}", tail),
                }
            }
            Nil => write!(f, "Nil"),
        }
    }
}

use List::*;

fn cons(value: i32, next: List) -> List {
    Cons(value, Box::new(next))
}

fn list_demo() {
    let list = cons(1, cons(2, cons(3, Nil)));
    println!("{}",list);
}

pub fn box_demo() {
    list_demo();
}