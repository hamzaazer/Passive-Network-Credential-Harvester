# Passive-Network-Credential-Harvester
This tool is for educational network auditing only. It prints a demo banner and simulates packet capture unless you uncomment the real packet processing. It's cross‑platform (Windows, Linux, macOS) with conditional compilation.
# Siren

A minimal Rust application demonstrating live network packet capture using the `pcap` crate.

> **Disclaimer**
>
> This project is intended for educational purposes, debugging, and authorized network analysis only. Capture network traffic only on systems and networks you own or have explicit permission to monitor.

---

## Features

- Written in Rust
- Live packet capture
- Automatically selects the default network interface
- Uses Berkeley Packet Filter (BPF)
- Captures HTTP (80) and HTTPS (443) traffic
- Cross-platform (Windows, Linux, macOS)

---

## Requirements

- Rust (stable)
- Cargo
- libpcap (Linux/macOS)
- Npcap (Windows)

---

## Installation

### Clone the repository

```bash
git clone https://github.com/hamzaazer/Passive-Network-Credential-Harvester.git
cd Passive-Network-Credential-Harvester
```

### Build

```bash
cargo build --release
```

---

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
pcap = "2"
```

---

## Platform Setup

### Windows

Install **Npcap** and enable:

- WinPcap API-compatible Mode

Download:
https://npcap.com/

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install libpcap-dev
```

### Fedora

```bash
sudo dnf install libpcap-devel
```

### Arch Linux

```bash
sudo pacman -S libpcap
```

### macOS

```bash
brew install libpcap
```

---

## Running

```bash
cargo run --release
```

or

```bash
./target/release/main.sr
```

---

## Project Structure

```
siren/

├── README.md
└──  main.rs
```

---

## Current Packet Filter

```text
tcp port 80 or tcp port 443
```

---

## How It Works

1. Detects the default network interface.
2. Opens a live capture session.
3. Applies a BPF filter.
4. Receives matching packets.
5. Waits until the program exits.

---

## Example Output

```text
[+] Siren execution complete. Proof hash: 00000000686fd4f1
```

---

## Notes

- Administrator/root privileges may be required.
- HTTPS traffic is encrypted.
- This example demonstrates packet capture setup only.

---

## Build Release

```bash
cargo build --release
```

Binary location:

Linux/macOS

```text
target/release/siren
```

Windows

```text
target\release\siren.exe
```

---

## License

MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software.

---

## Disclaimer

This software is provided for educational purposes, debugging, and authorized network analysis only.

The authors are not responsible for any misuse or unauthorized use of this software.

Always obtain permission before capturing network traffic.
