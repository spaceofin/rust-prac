use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let server1 = async {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            process_redis_commands(socket).await;
        }
    };

    let server2 = async {
        let listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                process_redis_connection(socket).await;
            });
        }
    };
    
    tokio::join!(server1, server2);
}

async fn process_redis_connection(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    println!("New Task Started.");
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        sleep(Duration::from_secs(5)).await;

        let response = Frame::Simple("OK".to_string());
        connection.write_frame(&response).await.unwrap();
    }

    println!("Connection closed");
}

async fn process_redis_commands(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    db.insert("test key".to_string(), "test value".as_bytes().to_vec());

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("Inserted key: {}", cmd.key());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    println!("Found key: {}", cmd.key());
                    Frame::Bulk(value.clone().into())
                } else {
                    println!("Key not found: {}", cmd.key());
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}