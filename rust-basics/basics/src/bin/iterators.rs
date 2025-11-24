#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn iterators_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("-----v1_iter-----");
    for val in v1_iter {
        println!("Got: {val}");
    }
    println!("v1: {v1:?}");
 
    let mut v2 = vec![4, 5, 6];
    let v2_iter_mut = v2.iter_mut();
    println!("-----v2_iter_mut-----");
    for val in v2_iter_mut {
        *val *= 10;
        println!("Got: {val}");
    }
    println!("v2: {v2:?}");

    let v3 = vec![7, 8, 9];
    let v3_into_iter = v3.into_iter();
    println!("-----v3_into_iter-----");
    for val in v3_into_iter {
        println!("Got: {val}");
    }
    // Compile Error: `v3` was moved into `v3_into_iter`.
    // println!("v3: {v3:?}");
    let v3 = vec![7, 8, 9];
    let v3_into_iter = v3.into_iter();
    for mut val in v3_into_iter {
        val *= 10;
        println!("Got: {val}");
    }

    let v4 = vec![10, 20, 30];
    let v4_iter = v4.iter();
    let total: i32 = v4_iter.sum();
    println!("v4 total: {total:?}");
    println!("V4: {v4:?}");
}

fn main() {
    iterators_demo();
}