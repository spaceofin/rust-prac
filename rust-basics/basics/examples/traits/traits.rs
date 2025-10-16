use std::iter;
use std::vec::IntoIter;

fn print_msg<T: std::fmt::Debug>(msg: T) {
    println!("{:?}", msg);
}

fn print_msg_impl(msg: impl std::fmt::Debug) {
    println!("{:?}", msg);
}

fn as_argument_type () {
    print_msg::<i32>(1000);
    print_msg("hi");
    print_msg_impl("hello");
    // Compile Error: 
    // `impl Trait` cannot be explicitly specified as a generic argument
    // print_msg_impl::<i32>("2000");
}

fn zero_to_n_explicit_return_type(n: i32) -> std::ops::Range<i32> {
    0..n+1
}

fn zero_to_n(n: i32) -> impl Iterator<Item = i32> {
    0..n+1
}

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function_boxed(y: i32) -> Box<dyn Fn(i32) -> i32> {
    let closure = move |x: i32| x + y;
    Box::new(closure)
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn double_positives_boxed<'a>(
    numbers: &'a Vec<i32>
) -> Box<dyn Iterator<Item = i32> + 'a> {
    Box::new(
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    )
}

fn as_return_type() {
    let mut iter = zero_to_n(5);

    while let Some(x) = iter.next() {
        print!("{} ",x);
    }
    println!();

    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6,7];
    let v3 = combine_vecs(v1, v2);
    for x in v3.take(20) {
        print!("{} ", x);
    }
    println!();

    let plus_one = make_adder_function(1);
    let result = plus_one(2);
    println!("result: {}", result);

    let plus_one_boxed = make_adder_function_boxed(1);
    let result_boxed = plus_one_boxed(2);
    println!("result_boxed: {}", result_boxed);

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    println!("doubles: {:?}", doubles.collect::<Vec<i32>>());
    
    let doubles_boxed = double_positives_boxed(&singles);
    println!("doubles_boxed: {:?}", doubles_boxed.collect::<Vec<i32>>());
}

pub fn traits_demo() {
    // as_argument_type();
    as_return_type();
}