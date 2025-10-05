use crate::service_info::ServiceInfo;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ScanState {
    pub target: String,
    pub start_port: u16,
    pub end_port: u16,
    pub threads: usize,
    pub timeout_ms: u64,
    pub udp_timeout_ms: u64,
    pub scanned_count: Arc<AtomicUsize>,
    pub total_ports: usize,
    pub results: Arc<Mutex<Vec<ServiceInfo>>>,
    pub is_running: Arc<AtomicBool>,
    pub is_complete: Arc<AtomicBool>,
}

impl ScanState {
    pub fn new(
        target: String,
        start_port: u16,
        end_port: u16,
        threads: usize,
        timeout_ms: u64,
        udp_timeout_ms: u64,
    ) -> Self {
        let total_ports = (end_port - start_port + 1) as usize;
        Self {
            target,
            start_port,
            end_port,
            threads,
            timeout_ms,
            udp_timeout_ms,
            scanned_count: Arc::new(AtomicUsize::new(0)),
            total_ports,
            results: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            is_complete: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
        self.is_complete.store(false, Ordering::SeqCst);
        self.scanned_count.store(0, Ordering::SeqCst);
        self.results.lock().unwrap().clear();
    }

    pub fn complete(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        self.is_complete.store(true, Ordering::SeqCst);
    }

    pub fn add_result(&self, result: ServiceInfo) {
        self.results.lock().unwrap().push(result);
    }

    pub fn increment_scanned(&self) {
        self.scanned_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> (usize, usize) {
        (self.scanned_count.load(Ordering::SeqCst), self.total_ports)
    }

    pub fn get_results(&self) -> Vec<ServiceInfo> {
        self.results.lock().unwrap().clone()
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete.load(Ordering::SeqCst)
    }
}
