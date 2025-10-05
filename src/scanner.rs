use std::{
    net::TcpStream,
    sync::{Arc, Mutex, mpsc::{Receiver, Sender}},
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use std::net::ToSocketAddrs;

use crate::udp::scan_udp;
use crate::fingerprint::fingerprint_service;
use crate::service_info::{ServiceInfo, Protocol};
use crate::signatures::SignatureMatcher;


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
// performs service fingerprinting, sends ServiceInfo results to `res_tx`,
// and increments the shared `completed` counter.
pub fn worker_loop(
    task_rx: Arc<Mutex<Receiver<u16>>>,
    res_tx: Sender<ServiceInfo>,
    target: Arc<String>,
    completed: Arc<AtomicUsize>,
    timeout_ms: u64,
    udp_timeout_ms: u64,
    matcher: Arc<SignatureMatcher>,
) {
    loop {
        let port = {
            let rx_guard = task_rx.lock().unwrap();
            match rx_guard.recv() {
                Ok(p) => p,
                Err(_) => break, // channel closed => exit loop
            }
        };

        // Scan TCP
        if scan_tcp(&target, port, timeout_ms) {
            // Perform fingerprinting
            let service_info = fingerprint_service(&target, port, Protocol::TCP, &matcher);
            println!("{}", service_info.display_full());
            let _ = res_tx.send(service_info);
        }

        // Scan UDP
        if scan_udp(&target, port, udp_timeout_ms) {
            let service_info = fingerprint_service(&target, port, Protocol::UDP, &matcher);
            println!("{}", service_info.display_full());
            let _ = res_tx.send(service_info);
        }

        // Update progress
        completed.fetch_add(1, Ordering::Relaxed);
    }
}
