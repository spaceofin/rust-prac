// use crate::smart_pointers::List::{Cons, Nil};
use List::{Cons, Nil};
use BinaryTree::{Leaf, Node};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum BinaryTree {
    Leaf(i32),
    Node {
        value: i32,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    }
}

fn smart_pointers_basics() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list:\n{list:?}");

    let binary_tree = Node {
        value: 1,
        left: Box::new(Leaf(2)),
        right: Box::new(Node {
            value: 3,
            left: Box::new(Leaf(4)),
            right: Box::new(Leaf(5)),
        }),
    };
    println!("binary tree:\n{binary_tree:#?}");
}

pub fn run() {
    smart_pointers_basics();
}