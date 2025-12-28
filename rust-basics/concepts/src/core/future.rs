use futures::executor::block_on;
use futures::future;

fn generate_futures() {
  // async {}
  let fut = async { 42 };
  let result = block_on(fut);
  println!("[async] result: {}", result);

  // async fn {}
  async fn return_42() -> i32 { 42 }
  let fut = return_42();
  let result = block_on(fut);
  println!("[async fn] result: {}", result);

  // helper function - futures::future::ready
  let fut = future::ready(42);
  let result = block_on(fut); 
  println!("[helper fn] result: {}", result);

  use std::future::Future;
  use std::pin::Pin;
  use std::task::{Context, Poll};

  // manually implemented Future
  // struct MyFut;
  // impl Future for MyFut {
  //   type Output = i32;
  //     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
  //       Poll::Ready(42)
  //   }
  // }

  // struct MyFut (i32);
  // impl Future for MyFut {
  //   type Output = i32;
  //   fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
  //       Poll::Ready(self.0)
  //   }
  // }

  struct MyFut<T: Copy>(T);
  impl<T: Copy> Future for MyFut<T> {
      type Output = T;
      fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0)
      }
  }

  let fut = MyFut(42);
  let result = block_on(fut);
  println!("[MyFut] result: {}", result);

}

pub fn run() {
  generate_futures();
}