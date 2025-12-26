use std::thread;
use std::time::Duration;
use trpl::{Html, block_on, spawn_task, sleep, join};

async fn wait_and_add_one(x: i32) -> i32 {
  for i in (1..=3).rev() {
    thread::sleep(Duration::from_secs(1));
    println!("{}", i);
  }
  x + 1
}

fn countdown_and_add_one(input: i32) {
  let result = block_on(async {
    wait_and_add_one(input).await
  });
  println!("input: {input}, result: {result}");
}

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn print_page_title() {
    let args: Vec<String> = std::env::args().collect();

    block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

fn await_sleep() {
  block_on(async {
      println!("A");
      sleep(Duration::from_secs(2)).await;
      println!("B");
  });
}

fn await_sleep_with_concurrent_task(){
  block_on(async {
      spawn_task(async {
        println!("C");
      });
      spawn_task(async {
        println!("D");
      });
      println!("A");
      sleep(Duration::from_secs(2)).await;
      println!("B");
      spawn_task(async {
        println!("E");
      });
  });
}

fn count_numbers() {
  block_on(async {
    spawn_task(async {
      for i in 1..10 {
        println!("[first task] number {i}");
        sleep(Duration::from_millis(500)).await;
      }
    });

    for i in 1..5 {
      println!("[second task] number {i}");
      sleep(Duration::from_millis(500)).await;
    }
  });
}

fn count_numbers_and_wait() {
  block_on(async {
    let handle = spawn_task(async {
      for i in 1..10 {
        println!("[first task] number {i}");
        sleep(Duration::from_millis(500)).await;
      }
    });

    // handle.await.unwrap();

    for i in 1..5 {
      println!("[second task] number {i}");
      sleep(Duration::from_millis(500)).await;
    }

    handle.await.unwrap();
  });
}

fn count_numbers_with_join() {
  block_on(async {
    let fut1 = async {
        for i in 1..10 {
            println!("[first task] number {i}");
            sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("[second task] number {i}");
            sleep(Duration::from_millis(500)).await;
        }
    };

    join(fut1, fut2).await;
  });
}

fn count_numbers_no_async_blocks() {
  block_on(async {
    for i in 1..10 {
        println!("[first loop] number {i}");
        sleep(Duration::from_millis(500)).await;
    }

    for i in 1..5 {
        println!("[second loop] number {i}");
        sleep(Duration::from_millis(500)).await;
    }
  });
}

fn count_numbers_await_immediately() {
  block_on(async {
    let fut1 = async {
        for i in 1..10 {
            println!("[first task] number {i}");
            sleep(Duration::from_millis(500)).await;
        }
    };
    fut1.await;

    let fut2 = async {
        for i in 1..5 {
            println!("[second task] number {i}");
            sleep(Duration::from_millis(500)).await;
        }
    };
    fut2.await;
  });
}

fn count_numbers_partial_async() {
  block_on(async {
    let fut1 = async {
        for i in 1..10 {
            println!("[first task] number {i}");
            sleep(Duration::from_millis(500)).await;
        }
    };

    for i in 1..5 {
        println!("[second loop] number {i}");
        sleep(Duration::from_millis(500)).await;
    }

    fut1.await;
  });
}

pub fn run() {
  // countdown_and_add_one(5);
  // print_page_title();
  // await_sleep();
  // await_sleep_with_concurrent_task();
  // count_numbers();
  // count_numbers_and_wait();
  // count_numbers_with_join();
  // count_numbers_no_async_blocks();
  // count_numbers_await_immediately();
  count_numbers_partial_async();
}