macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("You called {}()", stringify!($func_name));
        }
    }
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{} = {}",
                 stringify!($expression),
                 $expression);
    };
}

macro_rules! logic_compare {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right)
    };
    ($left:expr=> or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right)
    };
}

macro_rules! find_min_or_max {
    // min
    (min, $x:expr) => ($x);
    (min, $x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min_or_max!(min, $($y),+))
    );
    // max
    (max, $x:expr) => ($x);
    (max, $x:expr, $($y:expr),+) => (
        std::cmp::max($x, find_min_or_max!(max, $($y),+))
    );
}


pub fn macro_examples() {
    create_function!(foo);
    create_function!(bar);

    foo();
    bar();

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    logic_compare!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    logic_compare!(true=> or false);

    println!("min:{}, max:{}", find_min_or_max!(min, 1), find_min_or_max!(max, 1));
    println!("min:{}, max:{}", find_min_or_max!(min, 1 + 2, 2), find_min_or_max!(max, 1 + 2, 2));
    println!("min:{}, max:{}", find_min_or_max!(min, 5, 2 * 3, 4), find_min_or_max!(max, 5, 2 * 3, 4));
}