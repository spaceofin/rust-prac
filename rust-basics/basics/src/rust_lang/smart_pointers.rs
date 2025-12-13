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

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over tour quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         // Compile Error: `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    //         // self.sent_messages.push(String::from(message));
    //     }
    // }

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            // Runtime panic: multiple mutable borrows on RefCell.
            // let mut second_borrow = self.sent_messages.borrow_mut();
            // second_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use std::cell::RefCell;
use RefCellList::{Cons as RefCellCons, Nil as RefCellNil};

#[derive(Debug)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

fn ref_cell_examples() {
    let x = 5;
    // Compile Error: cannot borrow `x` as mutable, as it is not declared as mutable
    // let y = &mut x;

    let refcell1 = RefCell::new(1);
    {
        let ref1 = refcell1.borrow();
        println!("refcell1: {refcell1:?}");
        println!("ref1: {ref1:?}");
        // Runtime panic: RefCell<T> cannot borrow mutably because an immutable borrow already exists.
        // let ref2 = refcell1.borrow_mut();
    }

    {
        let mut ref2 = refcell1.borrow_mut();
        *ref2 += 1;
    }

    *(refcell1.borrow_mut()) += 1;
    println!("refcell1 after: {refcell1:?}\n");


    let refcell2 = RefCell::new(2);
    let rc1 = Rc::new(refcell2);
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);

    *rc1.borrow_mut() *= 10;
    *(rc2.borrow_mut()) *= 10;

    println!("rc1: {rc1:?}");
    println!("rc2: {rc2:?}");
    println!("rc3: {rc3:?}");

    let value = Rc::new(RefCell::new(5));
    let a_cons = RefCellCons(Rc::clone(&value), Rc::new(RefCellNil));
    let a = Rc::new(a_cons);
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

use CyclicList::{Cons as CLCons, Nil as CLNil};

#[derive(Debug)]
enum CyclicList {
    Cons(i32, RefCell<Rc<CyclicList>>),
    Nil,
}

impl CyclicList {
    fn tail(&self) -> Option<&RefCell<Rc<CyclicList>>> {
        match self {
            CLCons(_, item) => Some(item),
            CLNil => None,
        }
    }
}

fn reference_cycle() {
    let a = Rc::new(CLCons(5, RefCell::new(Rc::new(CLNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("Rc<List> stack address: {:p}",&a);
    // println!("Rc<List> points to heap List: {:?}",Rc::as_ptr(&a));
    println!("Rc<List> points to heap List: {:p}",&*a);
    // println!("a: {:?}",*a);
    println!("a: {:?}",a); // Deref Coercion
    // println!("a next item = {:?}", (&*a).tail());
    println!("a next item = {:?}", a.tail()); // Deref Coercion

    let b = Rc::new(CLCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

fn print_list_n(cur: &Rc<CyclicList>, n: usize) {
    let mut cur = Rc::clone(cur);

    for _ in 0..n {
        match &*cur {
            CLNil => {
                println!("Nil");
                return;
            }
            CLCons(val, next) => {
                print!("{val} -> ");
                let next_rc = Rc::clone(&next.borrow());
                cur = next_rc;
            }
        }
    }
    println!("...");
}

fn tri_reference_cycle() {
    let a = Rc::new(CLCons(1, RefCell::new(Rc::new(CLNil))));
    let b = Rc::new(CLCons(2, RefCell::new(Rc::clone(&a))));
    let c = Rc::new(CLCons(3, RefCell::new(Rc::clone(&b))));
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&c);
    }
    print!("a: ");
    print_list_n(&a, 10);
    print!("b: ");
    print_list_n(&b, 10);
    print!("c: ");
    print_list_n(&c, 10);
    println!("a rc count: {}", Rc::strong_count(&a));
    println!("b rc count: {}", Rc::strong_count(&b));
    println!("c rc count: {}", Rc::strong_count(&c));
}

pub fn run() {
    // smart_pointers_basics();
    // deref_examples();
    // deref_coercion_examples();
    // drop_examples();
    // rc_examples();
    // ref_cell_examples();
    // reference_cycle();
    tri_reference_cycle();
}