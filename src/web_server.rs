use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

use crate::scanner::worker_loop;
use crate::signatures::SignatureMatcher;
use crate::web_state::ScanState;

// Global state for the current scan
static CURRENT_SCAN: Lazy<RwLock<Option<ScanState>>> = Lazy::new(|| RwLock::new(None));

#[derive(Clone)]
pub struct AppState {
    pub matcher: Arc<SignatureMatcher>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    target: String,
    start_port: u16,
    end_port: u16,
    threads: usize,
    timeout_ms: u64,
    udp_timeout_ms: u64,
}

pub async fn run_web_server(matcher: Arc<SignatureMatcher>) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState { matcher };

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/styles.css", get(serve_styles))
        .route("/api/scan", post(start_scan))
        .route("/api/status", get(get_status))
        .route("/api/results", get(get_results))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9876").await?;
    println!("\n🌐 Web UI running at http://127.0.0.1:9876\n");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn serve_styles() -> ([(&'static str, &'static str); 1], &'static str) {
    (
        [("Content-Type", "text/css")],
        include_str!("../static/styles.css")
    )
}

async fn start_scan(
    State(app_state): State<AppState>,
    Form(req): Form<ScanRequest>,
) -> Html<String> {
    // Create new scan state
    let scan_state = ScanState::new(
        req.target.clone(),
        req.start_port,
        req.end_port,
        req.threads,
        req.timeout_ms,
        req.udp_timeout_ms,
    );

    scan_state.start();

    // Store in global state
    *CURRENT_SCAN.write().unwrap() = Some(scan_state.clone());

    // Spawn scan in background thread
    let matcher = app_state.matcher.clone();
    thread::spawn(move || {
        run_scan(scan_state, matcher);
    });

    Html(r#"
        <div id="progress-container">
            <div hx-get="/api/status" hx-trigger="every 500ms" hx-swap="outerHTML">
                <div class="progress-bar">
                    <div class="progress-fill" style="width: 0%"></div>
                </div>
                <p>Scan starting...</p>
            </div>
        </div>
        <div id="results-container" hx-get="/api/results" hx-trigger="every 1s" hx-swap="outerHTML">
        </div>
    "#.to_string())
}

fn run_scan(scan_state: ScanState, matcher: Arc<SignatureMatcher>) {
    let (task_tx, task_rx) = mpsc::channel();
    let (res_tx, res_rx) = mpsc::channel();

    let task_rx = Arc::new(Mutex::new(task_rx));
    let completed = scan_state.scanned_count.clone();
    let target = Arc::new(scan_state.target.clone());

    // Spawn worker threads
    for _ in 0..scan_state.threads {
        let task_rx_clone = Arc::clone(&task_rx);
        let res_tx_clone = res_tx.clone();
        let target_clone = Arc::clone(&target);
        let completed_clone = Arc::clone(&completed);
        let matcher_clone = Arc::clone(&matcher);
        let timeout = scan_state.timeout_ms;
        let udp_timeout = scan_state.udp_timeout_ms;

        thread::spawn(move || {
            worker_loop(
                task_rx_clone,
                res_tx_clone,
                target_clone,
                completed_clone,
                timeout,
                udp_timeout,
                matcher_clone,
            );
        });
    }

    // Send all ports to workers
    for port in scan_state.start_port..=scan_state.end_port {
        let _ = task_tx.send(port);
    }
    drop(task_tx); // Close channel so workers know to stop

    // Collect results
    drop(res_tx); // Close sender so receiver knows when to stop
    for service_info in res_rx {
        scan_state.add_result(service_info);
    }

    // Mark scan as complete
    scan_state.complete();
}

async fn get_status() -> Html<String> {
    let scan = CURRENT_SCAN.read().unwrap();

    if let Some(ref state) = *scan {
        let (scanned, total) = state.get_progress();
        let percentage = if total > 0 {
            (scanned as f64 / total as f64 * 100.0) as usize
        } else {
            0
        };

        let status_text = if state.is_complete() {
            format!("Scan complete! {}/{} ports scanned", scanned, total)
        } else if state.is_running() {
            format!("Scanning... {}/{} ports ({:.0}%)", scanned, total, percentage)
        } else {
            "Idle".to_string()
        };

        // Only poll if scan is still running
        let (poll_trigger, results_update) = if state.is_complete() {
            (String::new(), r#"<script>htmx.trigger('#results-container', 'scan-complete')</script>"#)
        } else {
            (r#" hx-get="/api/status" hx-trigger="every 500ms" hx-swap="outerHTML""#.to_string(), "")
        };

        Html(format!(r#"
            <div{}>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: {}%"></div>
                </div>
                <p>{}</p>
            </div>
            {}
        "#, poll_trigger, percentage, status_text, results_update))
    } else {
        Html(r#"
            <div>
                <p>No active scan</p>
            </div>
        "#.to_string())
    }
}

async fn get_results() -> Html<String> {
    let scan = CURRENT_SCAN.read().unwrap();

    if let Some(ref state) = *scan {
        let results = state.get_results();

        if results.is_empty() {
            let poll_attr = if state.is_complete() {
                String::new()
            } else {
                r#" hx-get="/api/results" hx-trigger="every 1s" hx-swap="outerHTML""#.to_string()
            };
            return Html(format!(r#"<div id="results-container"{}><p class="no-results">No open ports found yet...</p></div>"#, poll_attr));
        }

        // Only poll if scan is still running
        let poll_attr = if state.is_complete() {
            String::new()
        } else {
            r#" hx-get="/api/results" hx-trigger="every 1s" hx-swap="outerHTML""#.to_string()
        };

        let mut html = format!(r#"<div id="results-container"{}>"#, poll_attr);
        html.push_str(r#"
            <table class="results-table">
                <thead>
                    <tr>
                        <th>Port</th>
                        <th>Protocol</th>
                        <th>State</th>
                        <th>Service</th>
                        <th>Version</th>
                        <th>Banner</th>
                        <th>Confidence</th>
                    </tr>
                </thead>
                <tbody>
        "#);

        for result in results {
            let confidence_class = if result.confidence > 0.8 {
                "high"
            } else if result.confidence > 0.5 {
                "medium"
            } else {
                "low"
            };

            html.push_str(&format!(r#"
                <tr>
                    <td><strong>{}</strong></td>
                    <td>{}</td>
                    <td><span class="state-open">{}</span></td>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="banner">{}</td>
                    <td><span class="confidence {}">{:.0}%</span></td>
                </tr>
            "#,
                result.port,
                result.protocol,
                result.state,
                result.service.as_deref().unwrap_or("unknown"),
                result.version.as_deref().unwrap_or("-"),
                result.banner.as_deref().unwrap_or("-"),
                confidence_class,
                result.confidence * 100.0
            ));
        }

        html.push_str("</tbody></table></div>");
        Html(html)
    } else {
        Html(r#"<div id="results-container"><p>No scan data available</p></div>"#.to_string())
    }
}
