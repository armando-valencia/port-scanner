use std::{
    net::TcpStream,
    sync::{Arc, Mutex, mpsc::{Receiver, Sender}},
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use std::net::ToSocketAddrs;

use crate::banner::grab_banner;
use crate::udp::scan_udp;
use crate::util::service_name;


// Scans a TCP port on the given address. Returns `true` if the port is open.
pub fn scan_tcp(addr: &str, port: u16, timeout_ms: u64) -> bool {
    // Attempt to resolve the host:port into one or more SocketAddrs
    let mut addrs_iter = match (addr, port).to_socket_addrs() {
        Ok(iter) => iter,
        Err(_)   => return false,  // can't resolve → treat as closed
    };

    // Try the first resolved address
    if let Some(socket_addr) = addrs_iter.next() {
        TcpStream::connect_timeout(&socket_addr, Duration::from_millis(timeout_ms))
            .is_ok()
    } else {
        false
    }
}

// Worker loop: pulls port numbers from `task_rx`, scans TCP and UDP,
// prints results with service names and banners, sends any open ports
// into `res_tx`, and increments the shared `completed` counter.
pub fn worker_loop(
    task_rx: Arc<Mutex<Receiver<u16>>>,
    res_tx: Sender<u16>,
    target: Arc<String>,
    completed: Arc<AtomicUsize>,
    timeout_ms: u64,
    udp_timeout_ms: u64,
) {
    loop {
        let port = {
            let rx_guard = task_rx.lock().unwrap();
            match rx_guard.recv() {
                Ok(p) => p,
                Err(_) => break, // channel closed => exit loop
            }
        };

        if scan_tcp(&target, port, timeout_ms) {
            let svc = service_name(port);
            if let Some(banner) = grab_banner(&target, port) {
                println!("TCP Port {} ({}) OPEN -- {}", port, svc, banner);
            } else {
                println!("TCP Port {} ({}) OPEN", port, svc);
            }
            let _ = res_tx.send(port);
        }

        if scan_udp(&target, port, udp_timeout_ms) {
            let svc = service_name(port);
            println!("UDP Port {} ({}) OPEN or FILTERED", port, svc);
            let _ = res_tx.send(port);
        }

        // Update progress
        completed.fetch_add(1, Ordering::Relaxed);
    }
}
