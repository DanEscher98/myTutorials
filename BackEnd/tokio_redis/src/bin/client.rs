use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    // establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // create a new channel with a capacity of at most 32
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

    // start receiving messages
    while let Some(cmd) = rx.recv().await {
        use Command::*;
        match cmd {
            Get { key } => {
                client.get(&key).await;
            }
            Set { key, val } => {
                client.set(&key, val).await;
            }
        }
    }
}
