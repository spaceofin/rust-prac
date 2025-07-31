use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let server1 = async {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            process(socket).await;
        }
    };

    let server2 = async {
        let listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                process(socket).await;
            });
        }
    };
    
    tokio::join!(server1, server2);
}

async fn process(socket: TcpStream) {
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