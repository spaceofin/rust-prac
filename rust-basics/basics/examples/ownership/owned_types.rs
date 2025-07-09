pub fn owned_types() {
    let s1 = String::from("Hello");
    println!("s1: {}",s1);
    println!("&s1: {}",&s1);
    
    //move: s1's ownership transferred to s2
    let s2 = s1;

    //compile error: value borrowed after move
    //println!("s1: {}",s1);
    //println!("&s1: {}",&s1);

    //compile error: s1 has been invalidated, cannot assign value
    //sl = String::from("Hi");
    //println!("s1: {}",s1);

    println!("s2: {}",s2);
    println!("&s2: {}",&s2);

    {   
        let s3 = String::from("World");
        println!("s3: {}",s3);
    }

    //compile error: s3 has been dropped at the end of its scope
    //println!("s3: {}",s3);

    //deep copy 
    let s4 = String::from("hi");
    let s5 = s4.clone();
    println!("s4: {}, s5: {}", s4, s5);
}