use std::mem;
use std::any::{TypeId,type_name};

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn assert_same_type<T>(_: &T, _: &T) {}

pub fn arrays_and_slices_demo() {
    let arr_1: [i32; 5] = [3, 2, 1, 4, 5];
    let arr_2: [i32; 100] = [0; 100];

    println!("arr_1 occupies {} bytes", mem::size_of_val(&arr_1));

    println!("\nBorrow the whole arr_1 as a slice.");
    analyze_slice(&arr_1);
    println!("\nBorrow a section of the arr_2 as a slice.");
    analyze_slice(&arr_2[1 .. 5]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    println!("\nPrint arr_1");
    // Arrays can be safely accessed using `.get`, which returns an `Option`.
    for i in 0..arr_1.len() + 3 { 
        match arr_1.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // compile error: index out of bounds
    // println!("{}", arr_1[5]);
    
    println!("");
    
    let string_arr_1: [&str; 2] = ["One", "Two"];
    let string_arr_2: [&str; 3] = ["One", "Two", "Three"];

    let string_arr_1_type_name = type_name::<[&str; 2]>();
    let string_arr_2_type_name = type_name::<[&str; 3]>();

    if TypeId::of::<[&str; 2]>() == TypeId::of::<[&str; 3]>() {
        println!("{} and {} are Same type!",string_arr_1_type_name,string_arr_2_type_name);
    } else {
        println!("{} and {} are Different type!",string_arr_1_type_name,string_arr_2_type_name);
    }
}