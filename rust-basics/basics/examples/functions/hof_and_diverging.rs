fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn imperative_sum_of_odd_squares(upper: u32) {
    println!("Find the sum of all the numbers with odd squares under {}", upper);

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper { break; }
        else if is_odd(n_squared) { acc += n_squared; } 
    }
     println!("imperative style: {}", acc);
}

fn functional_sum_of_odd_squares(upper: u32) {
    println!("Find the sum of all the numbers with odd squares under {}", upper);

    // `map`, `take_while`, and `filter` are higher-order functions (HOFs)
    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn sum_odd_numbers(up_to: u32) {
    let mut acc = 0;
    for i in 0..up_to {
        let addition: u32 = match i%2 == 1 {
            true => i,
            // `continue` is a diverging expression (never returns a value)
            // It does not violate the type requirements of the match expression.
            false => continue,
        };
        acc += addition;
    }
    println!("Sum of odd numbers up to {} (excluding): {}", up_to, acc);
}

pub fn hof_and_diverging_demo() {
    // imperative_sum_of_odd_square(1000);
    // functional_sum_of_odd_squares(1000);
    sum_odd_numbers(10);
}