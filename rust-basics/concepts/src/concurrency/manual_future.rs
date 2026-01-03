use futures::executor::block_on;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

struct FutureState {
    ready: bool,
    started: bool,
}
struct ManualFuture {
    name: &'static str,
    delay_ms: u64,
    state: Arc<Mutex<FutureState>>,
}

impl ManualFuture {
    fn new(name: &'static str, delay_ms: u64) -> Self {
        Self {
            name,
            delay_ms,
            state: Arc::new(Mutex::new(FutureState {
                ready: false,
                started: false,
            })),
        }
    }
}

impl Future for ManualFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.ready {
            println!("[{} poll called] Ready", self.name);
            Poll::Ready("done")
        } else {
            if !state.started {
                println!("[{} poll called] Pending (Starting {}ms sleep)", self.name, self.delay_ms);
                state.started = true;

                let state_clone = Arc::clone(&self.state);
                let waker = cx.waker().clone();
                let delay = self.delay_ms;

                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(delay));
                    let mut locked = state_clone.lock().unwrap();
                    locked.ready = true;
                    waker.wake();
                });
            } else {
                println!("[{} poll called] Still Pending...", self.name);
            }
            Poll::Pending
        }
    }
}

async fn foo() {
    ManualFuture::new("a", 3000).await;
    ManualFuture::new("b", 1000).await;
}

async fn bar() {
    ManualFuture::new("d", 1000).await;
    ManualFuture::new("e", 2000).await;
}

fn manual_future_concurrency() {
  block_on(async {
    let f1 = foo();
    let f2 = bar();
    futures::join!(f1, f2);
  })
}

pub fn run() {
  manual_future_concurrency();
}