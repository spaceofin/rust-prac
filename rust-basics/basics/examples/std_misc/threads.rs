use std::thread;
use itertools::Itertools;

const NTHREADS: usize = 10;

fn threads_basic() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let orders = [
    "first", "second", "third", "fourth", "fifth",
    "sixth", "seventh", "eighth", "ninth", "tenth"
    ];

    for i in 0..NTHREADS {
        let order = orders[i];
        // Spin up another thread
        children.push(thread::spawn(move || {
            print!("[thread {}] ", i);
            println!("This is the {} thread", order); 
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn sum_digits_parallel() {
    // calculate the sum of all digits via a threaded map-reduce approach

    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    // vector to hold the child-threads which we will spawn.
    let mut children = vec![];
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                        .chars()
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}

fn sum_digits_parallel_chunks() {
    const NUM_CHUNKS: usize = 3;

    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    let mut children: Vec<thread::JoinHandle<u32>> = vec![];
    let data_vec: Vec<&str> = data.split_whitespace().collect();
    let chunks = data_vec.chunks((data_vec.len() + NUM_CHUNKS - 1) / NUM_CHUNKS);

    for (i, chunk) in chunks.enumerate() {
        let chunk = chunk.to_owned();
        println!("data chunk {} is \"{:?}\"", i, &chunk);
        children.push(thread::spawn(move || -> u32 {
            let result = chunk.iter()
                        .flat_map(|s| s.chars())
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        .sum();
            println!("processed chunk {}, result={}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}


pub fn threads_demo() {
    // threads_basic();
    // sum_digits_parallel();
    sum_digits_parallel_chunks();
}