macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

macro_rules! greet {
    (say $name:literal $( $rest:tt )* ) => {
        println!("Hello, {}!", $name);
        greet! { $($rest)* }
    };
    (repeat $name:literal $n:literal $( $rest:tt )* ) => {
        for _ in 0..$n {
            println!("Hello again, {}!", $name);
        }
        greet! { $($rest)* }
    };
    () => {};
}

macro_rules! log {
    ($msg:literal) => {
        println!("Log: {}", $msg);
    };

    ($msg:literal, $($rest:tt)*) => {
        println!("Log: {}", $msg);
        log! { $($rest)* }
    };
}

pub fn dsls_demo() {
    calculate! {
        eval 1 + 2 // `eval` is _not_ a Rust keyword!
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }
    calculate! { // Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
    
    greet! { 
        say "Rustacean"
        repeat "Alice" 5
    }

    log!(
        "Starting program",
        "Loading resources",
        "Initialization complete"
    );
}