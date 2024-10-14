use simple_db::{self, Command};
use std::{
    collections::VecDeque,
    io::{prelude::*, Read},
    net::{TcpListener, TcpStream},
    sync::{Arc, LazyLock, Mutex},
    thread,
    time::Duration,
};

struct Application {
    mailbox: VecDeque<String>,
}

impl Application {
    fn new() -> Self {
        Application {
            mailbox: VecDeque::new(),
        }
    }
    fn eval(&mut self, msg: String) -> anyhow::Result<Option<String>> {
        match simple_db::parse(msg.as_str()) {
            Ok(cmd) => self.execute(cmd),
            Err(e) => {
                println!("{:?}", e);
                Err(anyhow::anyhow!("Invalid command: {}", msg))
            }
        }
    }
    fn execute(&mut self, cmd: Command) -> anyhow::Result<Option<String>> {
        match cmd {
            Command::Publish(msg) => {
                self.mailbox.push_back(msg);
                Ok(None)
            }
            Command::Retrieve => {
                if let Some(msg) = self.mailbox.pop_front() {
                    Ok(Some(msg))
                } else {
                    println!("No messages in the mailbox");
                    Ok(None)
                }
            }
        }
    }
}

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

static APPLICATION: LazyLock<Arc<Mutex<Application>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Application::new())));

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    thread::scope(|s| {
        for stream in listener.incoming() {
            s.spawn(move || {
                match stream {
                    Ok(stream) => {
                        println!(
                            "{:?} - Client: {:?}",
                            thread::current().id(),
                            stream.peer_addr().unwrap()
                        );
                        if let Err(e) = handle_client(stream) {
                            println!("Error handling client: {:?}", e);
                        }
                    }
                    Err(e) => {
                        println!("Error connecting: {:?}", e);
                    }
                }
                println!();
            });
        }
    });
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buffer = [0; 512]; // 512-byte buffer
    let mut received_data = String::new(); // Store all the received data

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client closed the connection
                println!("Client closed connection");
                break;
            }
            Ok(size) => {
                // Append the received chunk to the complete data
                received_data.push_str(&String::from_utf8_lossy(&buffer[..size]));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout or no data available, stop reading
                // println!("Timed out or no more data available to read");
                break;
            }
            Err(e) => {
                // Handle other errors
                return Err(e.into());
            }
        }
    }

    if !received_data.is_empty() {
        println!("Received: {:?}", received_data);
        writeln!(stream, "Thank you for {received_data:?}")?;
        let app_clone = APPLICATION.clone();
        let mut app = app_clone.lock().unwrap();
        if let Some(ret) = app.eval(received_data)? {
            writeln!(stream, "{ret}")?;
        }
    }

    Ok(())
}
