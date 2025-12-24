use std::thread;
use std::time::Duration;
use trpl::{Html, block_on};

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

pub fn run() {
  // countdown_and_add_one(5);
  print_page_title();
}