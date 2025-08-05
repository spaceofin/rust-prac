use std::panic;

fn trigger_panic() {
    panic!("crash and burn");
}

fn trigger_index_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

pub fn trigger_unrecoverable_errors() {
    // trigger_panic();
    // trigger_index_panic();
    let result = panic::catch_unwind(||{
        trigger_index_panic();
    });

    assert!(result.is_err());
    println!("___Panic occurred, execution continues.");

    assert!(result.is_ok());
    println!("___No panic occurred, execution continues.");
}
