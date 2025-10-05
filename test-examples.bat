@echo off
REM Test script for port scanner fingerprinting feature

echo ========================================
echo Port Scanner Fingerprinting Test Suite
echo ========================================
echo.

echo [TEST 1] SSH Fingerprinting (High confidence expected)
echo Command: cargo run -- -d 45.33.32.156 -s 22 -e 22 -t 1 -c 2000
cargo run -- -d 45.33.32.156 -s 22 -e 22 -t 1 -c 2000 2>nul
echo.
echo Press any key to continue...
pause >nul

echo.
echo [TEST 2] Multiple ports on scanme.nmap.org
echo Command: cargo run -- -d 45.33.32.156 -s 20 -e 100 -t 10 -c 500
cargo run -- -d 45.33.32.156 -s 20 -e 100 -t 10 -c 500 2>nul
echo.
echo Press any key to continue...
pause >nul

echo.
echo [TEST 3] HTTP Detection
echo Command: cargo run -- -d example.com -s 80 -e 80 -t 1 -c 2000
cargo run -- -d example.com -s 80 -e 80 -t 1 -c 2000 2>nul
echo.
echo Press any key to continue...
pause >nul

echo.
echo [TEST 4] HTTPS/TLS Detection
echo Command: cargo run -- -d example.com -s 443 -e 443 -t 1 -c 2000
cargo run -- -d example.com -s 443 -e 443 -t 1 -c 2000 2>nul
echo.
echo Press any key to continue...
pause >nul

echo.
echo [TEST 5] Wide range fast scan
echo Command: cargo run -- -d 45.33.32.156 -s 1 -e 1000 -t 50 -c 200
cargo run -- -d 45.33.32.156 -s 1 -e 1000 -t 50 -c 200 2>nul
echo.
echo Press any key to continue...
pause >nul

echo.
echo [TEST 6] Localhost scan (if services running)
echo Command: cargo run -- -d 127.0.0.1 -s 1 -e 10000 -t 20 -c 50
cargo run -- -d 127.0.0.1 -s 1 -e 10000 -t 20 -c 50 2>nul
echo.

echo.
echo ========================================
echo All tests complete!
echo ========================================
