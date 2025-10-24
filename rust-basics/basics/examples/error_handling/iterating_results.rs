fn iterating_over_result() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        // Parses each string into a Result<i32, ParseIntError>, returning Ok(n) on success or Err(e) on failure
        .map(|s| s.parse::<i32>())
        // .collect(): gather items from iterator into a collection
        .collect();
    println!("Results: {:?}", numbers);
}

fn iterating_with_filter_map() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        // Converts each string to i32 and keeps only the successful Ok values, discarding any Err
        // .ok(): Converts a Result into an Option
        // .filter_map(): keep Some, skip None
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}

fn iterating_with_filter_map_and_map_err() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        // r: Result<u8, ParseIntError>
        // .map_err(): transform Err value, keep Ok unchanged
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}


fn iterating_with_collect_to_result() {
    let strings = vec!["93", "tofu", "18"];
    // Vec<Result<T, E>> can be turned into a Result<Vec<_>, _>
    // Once an Result::Err is found, the iteration will terminate
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

fn iterating_with_partition() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        // .partition(): splits the iterator into two collections based on a predicate
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Unwrapped numbers: {:?}", numbers);
    println!("Unwrapped errors: {:?}", errors);
}

pub fn iterating_results_demo() {
    println!("-----iterating_over_result-----");
    iterating_over_result();
    println!("\n-----iterating_with_filter_map-----");
    iterating_with_filter_map();
    println!("\n-----iterating_with_filter_map_and_map_err-----");
    iterating_with_filter_map_and_map_err();
    println!("\n-----iterating_with_collect_to_result-----");
    iterating_with_collect_to_result();
    println!("\n-----iterating_with_partition-----");
    iterating_with_partition();
}