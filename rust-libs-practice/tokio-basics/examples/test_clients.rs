use mini_redis::{client, Result};
use tokio::time::{sleep, Duration};

async fn sequential_client_requests() -> Result<()> {
    for i in 0..5 {
        let mut client = client::connect("127.0.0.1:6380").await?;

        println!("Sending request #{}", i + 1);

        client.set("hello", format!("client{}", i + 1).into()).await?;
        let result = client.get("hello").await?;

        println!("client{} got: {:?}", i + 1, result);

        sleep(Duration::from_millis(500)).await;
    }
    Ok(())
}

async fn concurrent_client_requests() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            let mut client = client::connect("127.0.0.1:6380").await.unwrap();
            let key = "hello";
            let value = format!("client{}", i + 1);

            client.set(key, value.clone().into()).await.unwrap();
            let result = client.get(key).await.unwrap();

            println!("client{} got: {:?}", i + 1, result);
        });

        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // let _ = sequential_client_requests().await;
    concurrent_client_requests().await;
}