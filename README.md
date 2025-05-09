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
   
4. Build and run project

    ```bash
    cargo run
    ```
