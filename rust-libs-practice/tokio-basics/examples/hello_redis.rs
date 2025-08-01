use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;
    let test_result = client.get("test key").await?;

    println!("got value from the server; result={:?}", result);
    println!("got test value from the server; result={:?}", test_result);

    Ok(())
}