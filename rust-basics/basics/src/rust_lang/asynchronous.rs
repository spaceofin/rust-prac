use std::thread;
use std::time::Duration;
use std::future::Future;
use trpl::{Html, block_on, spawn_task, sleep, join, select, yield_now, Either};

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

fn message_passing_without_async_blocks() {
  block_on(async{
    println!("-- first block_on --");
    let (tx, mut rx) = trpl::channel();

    let val = String::from("hi");
    tx.send(val).unwrap();

    let received = rx.recv().await.unwrap();
    println!("received '{received}'");
  });

  block_on(async{
    println!("-- second block_on --");
    let (tx, mut rx) = trpl::channel();

    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    let vals_len = vals.len();

    for val in vals {
        tx.send(val).unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    for _ in 0..vals_len {
      let received = rx.recv().await.unwrap();
      println!("received '{received}'");
    }
  });

  block_on(async{
    println!("-- third block_on --");
    let (tx, mut rx) = trpl::channel();

    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    drop(tx);

    // while let Some(value) = rx.recv().await {
    //     println!("received '{value}'");
    // }
    loop {
      match rx.recv().await {
          Some(value) => {
              println!("received '{value}'");
          }
          None => {
              println!("channel closed (received None)");
              break;
          }
      }
    }
  });
}

fn message_passing() {
  block_on(async {
    println!("-- joining sender and receiver futures --");
    let (tx, mut rx) = trpl::channel();
    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
          tx.send(val).unwrap();
          sleep(Duration::from_millis(500)).await;
        }
      };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
          println!("received '{value}'");
        }
      };

    trpl::join(tx_fut, rx_fut).await;
  });

  block_on(async {
    println!("-- Joining multiple sender futures and a receiver future --");
    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let tx1_fut = async move {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
      ];

      for val in vals {
        tx1.send(val).unwrap();
        sleep(Duration::from_millis(500)).await;
      }
    };

    let rx_fut = async {
      while let Some(value) = rx.recv().await {
        println!("received '{value}'");
      }
    };

    let tx_fut = async move {
      let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        sleep(Duration::from_millis(1500)).await;
      }
    };

    trpl::join!(tx1_fut, tx_fut, rx_fut);
  })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn select_asnyc() {
  block_on(async{
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        slow("a", 10);
        slow("a", 20);
        sleep(Duration::from_millis(50)).await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        slow("b", 10);
        slow("b", 15);
        slow("b", 350);
        sleep(Duration::from_millis(50)).await;
        println!("'b' finished.");
    };
    select(a, b).await;
  })
}

fn interleaved_async_with_sleep() {
  block_on(async {
      let one_ms = Duration::from_millis(1);

      let a = async {
          println!("'a' started.");
          slow("a", 30);
          sleep(one_ms).await;
          slow("a", 10);
          sleep(one_ms).await;
          slow("a", 20);
          sleep(one_ms).await;
          println!("'a' finished.");
      };

      let b = async {
          println!("'b' started.");
          slow("b", 75);
          sleep(one_ms).await;
          slow("b", 10);
          sleep(one_ms).await;
          slow("b", 15);
          sleep(one_ms).await;
          slow("b", 350);
          sleep(one_ms).await;
          println!("'b' finished.");
      };
      select(a, b).await;
  });
}

fn interleaved_async_with_yield() {
  block_on(async {
      let one_ms = Duration::from_millis(1);

      let a = async {
          println!("'a' started.");
          slow("a", 30);
          yield_now().await;
          slow("a", 10);
          yield_now().await;
          slow("a", 20);
          yield_now().await;
          println!("'a' finished.");
      };

      let b = async {
          println!("'b' started.");
          slow("b", 75);
          yield_now().await;
          slow("b", 10);
          yield_now().await;
          slow("b", 15);
          yield_now().await;
          slow("b", 350);
          yield_now().await;
          println!("'b' finished.");
      };
      select(a, b).await;
  });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration>
{
  match select(future_to_try, sleep(max_time)).await {
    Either::Left(output) => Ok(output),
    Either::Right(_) => Err(max_time),
  }
}

fn await_with_timeout() {
  block_on(async {
    let slow = async {
      sleep(Duration::from_secs(5)).await;
      "Finally finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
  })
}

async fn retry<F, Fut>(mut f: F, max_attempts: usize) -> Result<usize, ()>
where
  F: Fn() -> Fut,
  Fut: Future<Output = bool>,  {
    for attempt_num in 1..=max_attempts {
      if f().await {
        return Ok(attempt_num);
      }
    }
    Err(())
}

fn run_with_retry() {
  use rand::Rng;
  block_on(async {
    let attempt = || async {
      sleep(Duration::from_millis(200)).await;
      rand::thread_rng().gen_bool(0.3)
    };

    let max_attempts = 5;
    match retry(attempt, max_attempts).await {
      Ok(attempt_num) => {
        println!("Succeeded on attempt #{attempt_num}");
      }
      Err(_) => {
        println!("Failed after {max_attempts} attempts");
      }
    }
  })
}

use trpl::{StreamExt, stream_from_iter};

fn stream_basic() {
  block_on(async {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
  })
}

fn stream_of_futures() {
    block_on(async {
        let future_iter = (1..=10).map(|n| async move {
            sleep(Duration::from_millis(500)).await;
            n * 2
        });

        let mut stream = stream_from_iter(future_iter);

        while let Some(fut) = stream.next().await {
          let value = fut.await;
          println!("got: {value}");
        }
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
  // count_numbers_partial_async();
  // message_passing_without_async_blocks();
  // message_passing();
  // select_asnyc();
  // interleaved_async_with_sleep();
  // interleaved_async_with_yield();
  // await_with_timeout();
  // run_with_retry();
  // stream_basic();
  stream_of_futures();
}