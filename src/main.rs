mod banner;
mod scanner;
mod udp;
mod util;
mod progress;

use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::AtomicUsize;
use std::thread;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "port-scanner", about = "A fast, concurrent TCP/UDP port scanner")]
struct Opts {
    #[arg(short = 'd', long, default_value = "127.0.0.1")]
    target: String,

    #[arg(short = 's', long, default_value_t = 1)]
    start_port: u16,

    #[arg(short = 'e', long, default_value_t = 1024)]
    end_port: u16,

    #[arg(short = 't', long, default_value_t = 10)]
    threads: usize,

    #[arg(short = 'c', long, default_value_t = 50)]
    timeout_ms: u64,

    #[arg(short = 'u', long, default_value_t = 100)]
    udp_timeout_ms: u64,
}

fn main() {
    let opts = Opts::parse();
    println!("Starting scan on target: {}", opts.target);

    let target = Arc::new(opts.target);
    let start_port = opts.start_port;
    let end_port = opts.end_port;
    let total_ports = (end_port - start_port + 1) as usize;

    let completed = Arc::new(AtomicUsize::new(0));
    let reporter_handle = progress::spawn_reporter(total_ports, Arc::clone(&completed));

    let (task_tx, task_rx_raw) = mpsc::channel::<u16>();
    let (res_tx,  res_rx) = mpsc::channel::<u16>();

    // Wrap the receiver so it can be shared by multiple workers
    let task_rx = Arc::new(Mutex::new(task_rx_raw));

    // Create worker threads
    let mut handles = Vec::with_capacity(10);
    for _ in 0..opts.threads {
        let task_rx   = Arc::clone(&task_rx);
        let res_tx = res_tx.clone();
        let target = Arc::clone(&target);
        let completed = Arc::clone(&completed);

        let handle = thread::spawn(move || {
            // Delegate to scanner module
            scanner::worker_loop(
                task_rx,
                res_tx,
                target,
                completed,
                opts.timeout_ms,
                opts.udp_timeout_ms
            );
        });
        handles.push(handle);
    }
    drop(res_tx);

    for port in start_port..=end_port {
        let _ = task_tx.send(port);
    }
    drop(task_tx);

    for open_port in res_rx {
        println!("[RESULT] Port {} is OPEN", open_port);
    }

    // Wait for all worker threads to finish
    for handle in handles {
        let _ = handle.join();
    }
    let _ = reporter_handle.join();

    println!("Scan complete.");
}
