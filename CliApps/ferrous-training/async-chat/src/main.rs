#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{self, OwnedWriteHalf},
        TcpListener, TcpStream, ToSocketAddrs,
    },
    sync::{mpsc, oneshot},
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: OwnedWriteHalf,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    loop {
        let event = match events.recv().await {
            Some(event) => event,
            None => break,
        };

        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {from}: {msg}\n");
                        peer.send(msg).unwrap();
                    }
                }
            }
            Event::NewPeer { name, mut stream } => match peers.entry(name.clone()) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, mut client_receiver) = mpsc::unbounded_channel();
                    entry.insert(client_sender);
                    spawn_and_log_error(async move {
                        connection_writer_loop(&mut client_receiver, &mut stream).await
                    });
                }
            },
        }
    }
}

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: &mut OwnedWriteHalf,
) -> Result<()> {
    loop {
        let msg = messages.recv().await;
        match msg {
            Some(msg) => stream.write_all(msg.as_bytes()).await?,
            None => return Ok(()),
        }
    }
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded_channel();
    let _broker = task::spawn(broker_loop(broker_receiver));

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    Ok(())
}

async fn connection_loop(broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let Some(name) = lines.next_line().await? else {
        return Err("peer disconnected immediately")?;
    };
    let name = match lines.next_line().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err("peer disconnected immediately".into()),
        Err(e) => return Err(Box::new(e)),
    };
    println!("user '{}' connected!", name);

    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
        })
        .unwrap();

    while let Some(line) = lines.next_line().await? {
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1..].trim()),
        };
        let dest = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect::<Vec<_>>();
        let msg = msg.to_string();
        // TODO: this is temporary
        println!("Received message: {}", msg);
    }
    Ok(())
}

/// to handle client errors, log them and continue serving other clients
fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
