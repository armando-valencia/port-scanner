# Port Scanner

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)

A fast TCP/UDP port scanning tool written in Rust.

---

## Features

-   **Dual Interface**: Run as CLI tool or launch a web UI for easier interaction
-   **Concurrent Scanning**: Uses a configurable pool of worker threads to scan ports in parallel
-   **TCP & UDP Support**: Checks both TCP and UDP ports
-   **Service Fingerprinting**: Service identification using multiple detection methods:
    -   Banner grabbing for SSH, FTP, SMTP, POP3, IMAP
    -   HTTP Server header analysis with version extraction
    -   TLS/HTTPS detection
    -   Protocol-specific probing
-   **Extensive Signature "Database"**: Recognizes 30+ web servers and development tools including:
    -   Production servers: nginx, Apache, IIS, Tomcat, Jetty
    -   Dev servers: Vite, Webpack, Metro (React Native), Expo, Next.js, Angular CLI
    -   Application servers: Flask, Django, Rails, Express.js, ASP.NET, Gunicorn, Uvicorn
-   **Service Mapping**: Translates common port numbers to service names (e.g. 22 → SSH, 8081 → Metro)
-   **Real-time Progress**: Live updates via CLI or web UI with progress bars and status indicators
-   **Detailed Results**: Shows service name, version, confidence score, and banner information

---

## Prerequisites

-   Rust (1.60+): installed via [rustup](https://rustup.rs/)

---

## Building & Running

1. Clone the repo
2. Navigate to the project directory

    ```bash
    cd port-scanner
    ```

3. Build project

    ```bash
    cargo build
    ```

---

## Usage

### Web UI Mode

Launch the web interface for an intuitive scanning experience:

```bash
cargo run -- --web
```

Then open your browser to **http://127.0.0.1:9876**

The web UI provides:

-   ✨ Clean, modern interface with gradient styling
-   📊 Real-time progress bar with percentage updates
-   📋 Live results table that populates as ports are discovered
-   🎨 Color-coded confidence levels (high/medium/low)
-   🔍 Service fingerprinting with version detection

### CLI Mode

Run directly from the command line with arguments:

| Short | Long               | Type     | Default     | Description                              |
| ----- | ------------------ | -------- | ----------- | ---------------------------------------- |
| `-d`  | `--target`         | `String` | `127.0.0.1` | Hostname or IP address to scan           |
| `-s`  | `--start-port`     | `u16`    | `1`         | First port in the scan range (inclusive) |
| `-e`  | `--end-port`       | `u16`    | `1024`      | Last port in the scan range (inclusive)  |
| `-t`  | `--threads`        | `usize`  | `10`        | Number of worker threads to use          |
| `-c`  | `--timeout-ms`     | `u64`    | `50`        | TCP connect timeout in milliseconds      |
| `-u`  | `--udp-timeout-ms` | `u64`    | `100`       | UDP receive timeout in milliseconds      |
| `-w`  | `--web`            | `bool`   | `false`     | Launch web UI instead of CLI mode        |

#### CLI Examples

-   Scan localhost ports 1–1000 using 20 threads:

    ```bash
    cargo run -- -d 127.0.0.1 -s 1 -e 1000 -t 20
    ```

-   Scan a remote host with custom timeouts:

    ```bash
    cargo run -- --target scanme.nmap.org --start-port 1 --end-port 65535 --threads 50 --timeout-ms 100 --udp-timeout-ms 200
    ```

-   Fingerprint a specific service:

    ```bash
    cargo run -- -d 192.168.1.100 -s 8081 -e 8081 -c 2000
    ```

---

## Output Examples

### CLI Output

```
Scanning target 192.168.86.250 from port 1 to 1000...

TCP Port 22 (OPEN) - OpenSSH_8.2p1 | Banner: SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5 [confidence: 95%]
TCP Port 80 (OPEN) - nginx v1.18.0 | Banner: HTTP/1.1 200 OK [confidence: 95%]
TCP Port 8081 (OPEN) - Metro Bundler (React Native) | Banner: HTTP/1.1 200 OK [confidence: 90%]

========== SCAN SUMMARY ==========
Total open ports found: 3
==================================

[RESULT] TCP Port 22 (OPEN) - OpenSSH_8.2p1 | Banner: SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5 [confidence: 95%]
[RESULT] TCP Port 80 (OPEN) - nginx v1.18.0 | Banner: HTTP/1.1 200 OK [confidence: 95%]
[RESULT] TCP Port 8081 (OPEN) - Metro Bundler (React Native) | Banner: HTTP/1.1 200 OK [confidence: 90%]
```

### Web UI

The web interface displays results in a beautiful table format with:

-   Real-time progress tracking
-   Color-coded confidence indicators
-   Sortable columns for port, service, version, and confidence
-   Responsive design that works on all screen sizes
