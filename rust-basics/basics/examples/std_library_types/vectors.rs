fn vectors_capacity() {
    
    let mut v = vec![1,2,3];
    let v_len = v.len();
    let mut v_capacity = v.capacity();
    println!("[initial vector]len: {}, capacity: {}", v_len, v_capacity);

    println!("\n-----Vec Capacity Growth Pattern-----");
    for i in (v_len + 1)..=1000 {
        v.push(i);
        if v_capacity != v.capacity() {
            v_capacity = v.capacity();
            println!("[after push {}] len: {}, capacity: {}", i, v.len(), v_capacity);
        }
    }
    println!("\n-----Vec Capacity Shrink Pattern-----");
    
    for _ in 0..900 { v.pop(); }
    println!("[after pops] len: {}, capacity: {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("[after shrink_to_fit] len: {}, capacity: {}", v.len(), v.capacity());

    for i in ((v_len + 1)..=100).rev() {
        v.pop();
        v.shrink_to_fit();
        if v_capacity != v.capacity() {
            v_capacity = v.capacity();
            println!("[after pop {}] len: {}, capacity: {}", i, v.len(), v_capacity);
        }
    }
}

fn vectors_basic() {
    // Iterators can be collected into vectors
    // Immutable vectors can't grow
    let immutable_v: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", immutable_v);

    // The `vec!` macro can be used to initialize a vector
    let mut mutable_v = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", mutable_v);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    mutable_v.push(4);

    println!("Vector: {:?}", mutable_v);
    println!("Second element: {}", mutable_v[1]);

    // Out of bounds indexing yields a panic
    // println!("Fourth element: {}", mutable_v[3]);

    // `Vector`s can be easily iterated
    println!("Contents of mutable_v:");
    for x in mutable_v.iter() {
        println!("> {}", x);
    }

    // A `Vector` can also be iterated with index using enumerate.
    for (i, x) in mutable_v.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // `iter_mut` allows modifying each value
    for x in mutable_v.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", mutable_v);
}

pub fn vectors_demo() {
    // vectors_capacity();
    vectors_basic();
}