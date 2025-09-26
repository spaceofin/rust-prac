use rand::Fill;

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn get_static_num<'a>(_: &'a i32) -> &'static i32 {
    &NUM
}

fn reference_lifetime() {
    let outer_static_string;
    {
        println!("__________inner block 1__________");
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        outer_static_string = static_string;
        println!("addr static_string: {:p}", static_string.as_ptr());
        println!("addr outer_static_string: {:p}", outer_static_string.as_ptr());
    }

    let coerced_static;
    let static_num;
    {
        println!("__________inner block 2__________");
        println!("outer_static_string: {}", outer_static_string);

        let lifetime_num = 9;
        coerced_static = coerce_static(&lifetime_num);
        println!("coered_static: {}", coerced_static);
        static_num = get_static_num(&lifetime_num);
        println!("static_NUM: {}", static_num);
    }

    println!("__________outer block__________");
    // Compile Error: borrow lifetime already ends
    // println!("coered_static: {}", coerced_static);
    println!("static_num: {}",static_num);
    println!("NUM: {} stays accessible!", NUM);
}

fn random_vec() -> &'static [usize; 10] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 10]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

fn compare_random_vectors() {
    let first: &'static [usize; 10] = random_vec();
    println!("first vector: {:?}",first);
    let second: &'static [usize; 10] = random_vec();
    println!("second vector: {:?}",second);
    if first == second {
        println!("first == second");
    } else {
        println!("first != second");
    }
}

pub fn lifetimes_demo() {
    // reference_lifetime();
    compare_random_vectors();
}