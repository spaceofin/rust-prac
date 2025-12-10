// use crate::smart_pointers::List::{Cons, Nil};
use List::{Cons, Nil};
use BinaryTree::{Leaf, Node};
use std::ops::{Deref, DerefMut};
use std::{fmt, rc::Rc};
use RcList::{Cons as RcCons, Nil as RcNil};

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

struct NoDerefBox<T>(T);

impl<T> NoDerefBox<T> {
    fn new(x: T) -> NoDerefBox<T> {
        NoDerefBox(x)
    }
}

impl<T> fmt::Pointer for NoDerefBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr: *const T = &self.0;
        fmt::Pointer::fmt(&ptr, f)
    }
}

// A practice smart pointer that stores its value on the stack,
// not on the heap like `Box<T>`. Used for understanding how `Deref` works.
struct DerefBox<T>(T);

impl<T> DerefBox<T> {
    fn new(x: T) -> DerefBox<T> {
        DerefBox(x)
    }
}

impl<T> Deref for DerefBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Pointer for DerefBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

fn deref_examples() {
    println!("-----Box-----");
    let x = 5;
    let y1 = &x;
    let y2 = &x;
    let z1 = Box::new(x);
    let z2 = Box::new(x);

    println!("x: {x}");
    println!("&y1: {:p}, y1: {:p}, *y1: {}", &y1, y1, *y1);
    println!("&y2: {:p}, y2: {:p}, *y2: {}", &y2, y2, *y2);
    println!("&z1: {:p}, z1: {:p}, *z1: {}", &z1, z1, *z1);
    println!("&z2: {:p}, z2: {:p}, *z2: {}", &z2, z2, *z2);

    println!("-----Deref & NoDeref Box-----");
    let a = 10;
    let b = DerefBox::new(a);
    let c = DerefBox::new(a);
    let no_deref_d = NoDerefBox::new(a);

    println!("a: {}", a); 
    println!("&b: {:p}, b: {:p}, *b: {}", &b ,b ,*b); 
    println!("&c: {:p}, c: {:p}, *c: {}", &c, c, *c);
    println!("&no_deref_d: {:p}, no_deref_d: {:p}, no_deref_d: {}", &no_deref_d, no_deref_d, no_deref_d.0);
    // Compile Error: `no_deref_d` can't be dereferenced.
    // println!("*no_deref_d: {}", *no_deref_d);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

struct Pointer1<T>(T);
impl<T> Deref for Pointer1<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

struct Pointer2<T>(T);
impl<T> Deref for Pointer2<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn value(value: &i32) {
    println!("Value, {value}!");
}
fn value_plus_one(value: &mut i32){
    println!("Value Plus One, {}!", *value + 1);
}
fn value_ref(value: &i32){
    println!("Value Ref, {value}!");
}

fn deref_coercion_examples() {
    let m = MyBox::new(String::from("Rust"));
    let v = DerefBox(Pointer1(Pointer2(123)));
    let mut vv = 123;
    println!("-----no deref coercion-----");
    hello(&*m);
    value(&***v);
    println!("-----deref coercion-----");
    hello(&m);
    value(&**v);
    value(&*v);
    value(&v);
    value_plus_one(&mut vv);
    value_ref(&mut vv);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_examples() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Compile Error: explicit destructor calls not allowed
    // d.drop();

    // std::mem::drop
    drop(d);
    println!("CustomSmartPointer dropped before the end of the function.");
}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_examples() {
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("reference count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("reference count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("reference count after creating c = {}", Rc::strong_count(&a));
        println!("a: {a:?}\nb: {b:?}\nc: {c:?}");
        // When `c` goes out of scope at the end of this block,
        // its Drop is automatically called and the reference count of `a` decreases.
    }
    println!("reference count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn run() {
    // smart_pointers_basics();
    // deref_examples();
    // deref_coercion_examples();
    // drop_examples();
    rc_examples();
}