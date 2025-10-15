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

}

pub fn traits_demo() {
    // as_argument_type();
    as_return_type();
}