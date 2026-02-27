use std::{thread, time::Duration};
use tokio::time::{sleep};
use futures::join;

fn sync_wait_and_print(id: u8) {
    println!("Sync: Start task {}", id);
    thread::sleep(Duration::from_secs(3));
    println!("Sync: End task {}", id);
}

async fn async_wait_and_print(id: u8) {
    println!("Async: Start task {}", id);
    sleep(Duration::from_secs(5)).await;
    println!("Async: End task {}", id);
}

fn run_sync_tasks() {
    sync_wait_and_print(1);
    sync_wait_and_print(2);
    sync_wait_and_print(3);
}

async fn run_async_tasks() {
    let f1 = async_wait_and_print(1);
    let f2 = async_wait_and_print(2);
    let f3 = async_wait_and_print(3);

    join!(f1, f2, f3);
}

#[tokio::main]
async fn main() {
    run_async_tasks().await;
    run_sync_tasks();
}