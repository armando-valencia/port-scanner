use std::net::{SocketAddr, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn scan_port(addr: &str, port: u16) -> bool {
    let socket = format!("{}:{}", addr, port);
    let socket_addr: SocketAddr = match socket.parse() {
        Ok(sa) => sa,
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&socket_addr, Duration::from_millis(50)).is_ok()
}

fn main() {
    let target = Arc::new("127.0.0.1".to_string());

    let (task_tx, task_rx_raw) = mpsc::channel::<u16>();
    let (res_tx, res_rx)       = mpsc::channel::<u16>();

    // Wrap the receiver in an Arc<Mutex> to share it between threads
    let task_rx = Arc::new(Mutex::new(task_rx_raw));

    // Spawn 10 worker threads
    let mut handles = Vec::with_capacity(10);
    for _ in 0..10 {
        let task_rx = Arc::clone(&task_rx);
        let res_tx  = res_tx.clone();
        let target  = Arc::clone(&target);

        let handle = thread::spawn(move || {
            loop {
                let port = {
                    let rx_guard = task_rx.lock().unwrap();
                    match rx_guard.recv() {
                        Ok(p) => p,
                        Err(_) => break,
                    }
                };

                if scan_port(&target, port) {
                    let _ = res_tx.send(port);
                }
            }
        });
        handles.push(handle);
    }

    // Drop extra sender so res_rx sees EOF when workers finish
    drop(res_tx);

    for port in 1..=1024 {
        let _ = task_tx.send(port);
    }
    drop(task_tx);

    for open_port in res_rx {
        println!("Port {} is OPEN", open_port);
    }

    // Wait for all workers to finish
    for handle in handles {
        let _ = handle.join();
    }

    println!("Scan complete.");
}
