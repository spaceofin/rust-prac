fn any_examples() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x ==2 ));
}

fn find_examples() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    // iter() for vecs yields `&i32`. 
    // `find()` want to reference one of its items, destructure &&i32 to i32
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    // iter() for vecs yields `i32`.
    // Destructure &i32 to i32 
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));


    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}

fn position_example() {
    let vec = vec![1, 9, 3, 3, -1, 13, 2];

    // `iter()` for vecs yields `&i32`
    // `position()` does not take a reference, destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    println!("index of first even nubmer of vec: {:?}", index_of_first_even_number);

    // `into_iter()` for vecs yields `i32`
    // No destructuring required.
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    println!("index of first negative nubmer of vec: {:?}", index_of_first_negative_number);
}

pub fn closure_examples_std_demo() {
    // any_examples();
    // find_examples();
    position_example();
    
}  