# Port Scanner

A simple, concurrent TCP port scanner written in Rust.  
It scans ports 1–1024 on a specified target host using a 10‑worker thread pool and reports any open ports.

---

## 🔍 Features

-   Scans TCP ports 1–1024 by default
-   Use 10 parallel worker threads
-   50ms timeout per port
-   Prints open port as it’s discovered

---

## 🚀 Prerequisites

-   Rust (1.60+): installed via [rustup](https://rustup.rs/)
-   A terminal (macOS/Linux) or PowerShell/CMD (Windows)

---

## 🛠️ Building & Running

1. Clone the repo
2. Navigate to the project directory

   ```bash
   cd port-scanner
   ```
   
3. Build and run project

    ```bash
    cargo build
    ```

4. Run with arguments

    | Short | Long               | Type     | Default     | Description                              |
    | ----- | ------------------ | -------- | ----------- | ---------------------------------------- |
    | `-d`  | `--target`         | `String` | `127.0.0.1` | Hostname or IP address to scan           |
    | `-s`  | `--start-port`     | `u16`    | `1`         | First port in the scan range (inclusive) |
    | `-e`  | `--end-port`       | `u16`    | `1024`      | Last port in the scan range (inclusive)  |
    | `-t`  | `--threads`        | `usize`  | `10`        | Number of worker threads to use          |
    | `-c`  | `--timeout-ms`     | `u64`    | `50`        | TCP connect timeout in milliseconds      |
    | `-u`  | `--udp-timeout-ms` | `u64`    | `100`       | UDP receive timeout in milliseconds      |

    ### Examples

    - Scan localhost ports 1–1000 using 20 threads:

        ```bash
        cargo run -- -d 127.0.0.1 -s 1 -e 1000 -t 20
        ```

    - Scan a remote host with custom timeouts:

        ```bash
        cargo run -- --target scanme.nmap.org --start-port 1 --end-port 65535 --threads 50 --timeout-ms 100 --udp-timeout-ms 200
        ```
