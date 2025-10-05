# Testing Service Fingerprinting

## Quick Test Commands

### 1. Test SSH Fingerprinting
```bash
# Scan scanme.nmap.org SSH (known to work)
cargo run -- -d 45.33.32.156 -s 22 -e 22 -t 1 -c 2000

# Expected output:
# TCP Port 22 (OPEN) - OpenSSH_6.6.1p1 | Banner: OpenSSH_6.6.1p1 Ubuntu-2ubuntu2.13 [confidence: 80%]
```

### 2. Test HTTP Fingerprinting
```bash
# Test HTTP on various sites
cargo run -- -d example.com -s 80 -e 80 -t 1 -c 2000
cargo run -- -d scanme.nmap.org -s 80 -e 80 -t 1 -c 2000
cargo run -- -d httpbin.org -s 80 -e 80 -t 1 -c 2000
```

### 3. Test HTTPS/TLS Fingerprinting
```bash
# Test HTTPS on port 443
cargo run -- -d example.com -s 443 -e 443 -t 1 -c 2000
cargo run -- -d www.google.com -s 443 -e 443 -t 1 -c 2000
```

### 4. Test Multiple Services
```bash
# Scan common ports (SSH, HTTP, HTTPS)
cargo run -- -d scanme.nmap.org -s 20 -e 450 -t 10 -c 500

# Scan all common service ports
cargo run -- -d scanme.nmap.org -s 1 -e 1000 -t 20 -c 200
```

### 5. Test Local Services (if you have them running)
```bash
# Scan localhost
cargo run -- -d 127.0.0.1 -s 1 -e 10000 -t 20 -c 50

# Scan specific local services
cargo run -- -d localhost -s 3306 -e 3306 -t 1 -c 1000  # MySQL
cargo run -- -d localhost -s 5432 -e 5432 -t 1 -c 1000  # PostgreSQL
cargo run -- -d localhost -s 6379 -e 6379 -t 1 -c 1000  # Redis
```

### 6. Fast Wide Scan
```bash
# Quick scan of many ports with high concurrency
cargo run -- -d scanme.nmap.org -s 1 -e 10000 -t 50 -c 100
```

### 7. Slow Thorough Scan
```bash
# Slower scan with longer timeouts for accuracy
cargo run -- -d scanme.nmap.org -s 1 -e 1000 -t 10 -c 1000 -u 500
```

## Expected Results

### SSH Detection
Should show:
- Service name (e.g., "OpenSSH_8.2p1")
- Full banner with OS info
- High confidence (80-95%)

### HTTP Detection
Should show:
- Server software (nginx, Apache, etc.)
- Version if available
- HTTP status line as banner
- Medium-high confidence (70-95%)

### HTTPS/TLS Detection
Should show:
- "HTTPS" or "TLS Service Detected"
- TLS info if probe succeeds
- Medium-high confidence (80-90%)

### Unknown Services
Should show:
- Port hint from signatures.json if available
- Low confidence (30%)
- Or "unknown" if no hint

## Testing Tips

1. **Start with known services**: Use scanme.nmap.org (port 22) which reliably has SSH
2. **Check timeouts**: Increase `-c` and `-u` values if services aren't being detected
3. **Use fewer threads initially**: Start with `-t 1` to see results clearly
4. **Compare with nmap**: Run `nmap -sV <target>` to compare results

## Common Test Targets

```bash
# Known open SSH
cargo run -- -d 45.33.32.156 -s 22 -e 22

# Known web servers (may vary)
cargo run -- -d httpbin.org -s 80 -e 80
cargo run -- -d www.apache.org -s 80 -e 80

# Mixed services
cargo run -- -d scanme.nmap.org -s 1 -e 100
```

## Troubleshooting

If fingerprinting isn't working:
1. Increase timeout: `-c 2000` (2 seconds for TCP)
2. Test single port: `-s 22 -e 22` instead of ranges
3. Use known good target: `45.33.32.156` port 22
4. Check if service responds to probes (some are configured to ignore scans)
