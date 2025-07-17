use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBarAdd;
#[derive(Debug)]
struct BarFooAdd;

#[derive(Debug)]
struct FooBarSub;
#[derive(Debug)]
struct BarFooSub;

#[derive(Debug)]
struct FooBarMul;
#[derive(Debug)]
struct BarFooMul;

#[derive(Debug)]
struct FooBarDiv;
#[derive(Debug)]
struct BarFooDiv;

impl ops::Add<Bar> for Foo {
    type Output = FooBarAdd;

    fn add(self, _rhs: Bar) -> FooBarAdd {
        println!("> Foo.add(Bar) was called");
        FooBarAdd
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFooAdd;

    fn add(self, _rhs: Foo) -> BarFooAdd {
        println!("> Bar.add(Foo) was called");
        BarFooAdd
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = FooBarSub;

    fn sub(self, _rhs: Bar) -> FooBarSub {
        println!("> Foo.sub(Bar) was called");
        FooBarSub
    }
}

impl ops::Sub<Foo> for Bar {
    type Output = BarFooSub;

    fn sub(self, _rhs: Foo) -> BarFooSub {
        println!("> Bar.sub(Foo) was called");
        BarFooSub
    }
}

impl ops::Mul<Bar> for Foo {
    type Output = FooBarMul;

    fn mul(self, _rhs: Bar) -> FooBarMul {
        println!("> Foo.mul(Bar) was called");
        FooBarMul
    }
}

impl ops::Mul<Foo> for Bar {
    type Output = BarFooMul;

    fn mul(self, _rhs: Foo) -> BarFooMul {
        println!("> Bar.mul(Foo) was called");
        BarFooMul
    }
}

impl ops::Div<Bar> for Foo {
    type Output = FooBarDiv;

    fn div(self, _rhs: Bar) -> FooBarDiv {
        println!("> Foo.div(Bar) was called");
        FooBarDiv
    }
}

impl ops::Div<Foo> for Bar {
    type Output = BarFooDiv;

    fn div(self, _rhs: Foo) -> BarFooDiv {
        println!("> Bar.div(Foo) was called");
        BarFooDiv
    }
}

pub fn test_binary_operators() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    println!("Foo - Bar = {:?}", Foo - Bar);
    println!("Bar - Foo = {:?}", Bar - Foo);

    println!("Foo * Bar = {:?}", Foo * Bar);
    println!("Bar * Foo = {:?}", Bar * Foo);

    println!("Foo / Bar = {:?}", Foo / Bar);
    println!("Bar / Foo = {:?}", Bar / Foo);
}
