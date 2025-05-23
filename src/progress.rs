use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Prints the progress in terminal
pub fn spawn_reporter(
    total: usize,
    completed: Arc<AtomicUsize>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while completed.load(Ordering::Relaxed) < total {
            let done = completed.load(Ordering::Relaxed);
            let pct = done as f64 * 100.0 / total as f64;
            println!("Progress: {}/{} ({:.1}%)", done, total, pct);
            thread::sleep(Duration::from_millis(500));
        }
    })
}
