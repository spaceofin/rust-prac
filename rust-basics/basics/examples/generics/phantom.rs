use std::marker::PhantomData;
use std::ops::Add;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] 
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

/// Create void enumerations to define unit types.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// f64 and PhantomData already implement the Debug, Clone and Copy traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// pub trait Add<RHS = Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

/// The `Add` trait defines the behavior of the `+` operator.
impl<Unit> Add<Length<Unit>> for Length<Unit> {
    type Output = Length<Unit>;

    // add() returns a new `Length` struct containing the sum.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`.
        Length::<Unit>(self.0 + rhs.0, PhantomData)
    }
}

fn phantom_example() {
    // Here, `f32` and `f64` are the hidden parameters.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _tuple3: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple4: PhantomTuple<char, f32> = PhantomTuple('R', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct3: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct4: PhantomStruct<char, f32> = PhantomStruct {
        first: 'R',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);

    println!("_tuple1 == _tuple3 yields: {}", _tuple1 == _tuple3);
    println!("_tuple1 == _tuple4 yields: {}", _tuple1 == _tuple4);
    println!("_struct1 == _struct3 yields: {}", _struct1 == _struct3);
    println!("_struct1 == _struct4 yields: {}", _struct1 == _struct4);
}

fn unit_clarification() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let two_foot: Length<Inch> = Length::<Inch>(24.0, PhantomData::<Inch>);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    let two_meter: Length<Mm> = Length::<Mm>(2000.0, PhantomData::<Mm>);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    let three_feet = one_foot + two_foot;
    let three_meters = one_meter + two_meter;

    println!("one foot + two_foot = {:?} in", three_feet.0);
    println!("one meter + two_meter = {:?} mm", three_meters.0);
}

pub fn phantom_demo() {
    // phantom_example();
    unit_clarification();
}