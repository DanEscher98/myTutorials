use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let mut handlers: Vec<JoinHandle<()>> = Vec::new();
    for thread in 0..3 {
        let handle = thread::spawn(move || {
            for c in b'a'..=b'e' {
                println!("Letter: {} from thread {}", c as char, thread);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handlers.push(handle);
    }

    handlers
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
