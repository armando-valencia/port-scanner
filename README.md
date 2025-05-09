# Rust Port Scanner

A simple, concurrent TCP port scanner written in Rust.  
It scans ports 1–1024 on a specified target host using a 10‑worker thread pool and reports any open ports.

---

## 🔍 Features

-   Scans TCP ports 1–1024 by default
-   Uses 10 parallel worker threads for speed
-   50ms timeout per port
-   Prints open port as it’s discovered

---

## 🚀 Prerequisites

-   Rust (1.60+): installed via [rustup](https://rustup.rs/)
-   A terminal (macOS/Linux) or PowerShell/CMD (Windows)

---

## 🛠️ Building & Running

1. **Clone the repo**

    ```bash
    git clone https://github.com/your-username/port-scanner.git
    cd port-scanner

    ```

2. Build and run project

    ```bash
    cargo run
    ```
