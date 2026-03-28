# 🛡️ ISU-SEC-OPS ENGINE

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-7000ff?style=for-the-badge)
![SQLite](https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white)

**ISU-SecOps-Engine** is a high-performance, professional-grade network service fingerprinting and host discovery tool built with Rust. It combines a robust backend engine with a premium, glassmorphism-based web dashboard for real-time security analysis.

## 🚀 Key Features

- **Extreme Performance:** Built on Tokio's asynchronous runtime for high-concurrency port scanning.
- **Smart Host Discovery:** Automatically detects and skips dead hosts to optimize large network (/24) scans.
- **Service Fingerprinting:** Deep banner grabbing and version detection for protocols like SSH, HTTP, Redis, Postgres, SMTP, and more.
- **Premium Web Dashboard:** Modern dark-mode UI with glassmorphism, floating animations, and real-time WebSocket updates.
- **Vulnerability Intelligence:** Integrated one-click CVE/exploit lookup for detected service versions.
- **Visual Analytics:** Real-time metrics (Hosts Scanned, Open Ports) and service distribution charts.
- **Persistent Scan History:** SQLite-backed storage allows you to review and re-analyze past scans.
- **Multi-Format Export:** Export your results in JSON, CSV, or generate a professional PDF Report.

## 🛠️ Technology Stack

- **Backend:** Rust, Axum, Tokio, Rusqlite, Serde
- **Frontend:** Vanilla HTML5/CSS3 (Glassmorphism), JavaScript (ES6+), WebSockets
- **Protocols:** TCP, SSL/TLS Banner Grabbing

## 📦 Installation & Setup

### Prerequisites
- [Rust](https://rustup.rs/) (Stable 1.70+)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (MSVC Linker)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ISU-SecOps-Engine.git
cd ISU-SecOps-Engine

# Build & Run the Web Dashboard
cargo run -- web
```

## 🖥️ Usage

1. **Launch the Engine:** Run `cargo run -- web`.
2. **Access the Dashboard:** Open `http://127.0.0.1:8080` in your browser.
3. **Configure Scan:**
   - Enter a target IP (e.g., `192.168.1.1`), Hostname, or CIDR (e.g., `192.168.1.0/24`).
   - Use **Scan Presets** (Web, DB, Popular) or enter custom ports.
4. **Analyze:** Watch results stream in real-time. Use the search bar to filter specific services.
5. **Report:** Click **PDF Report** to generate a clean summary for documentation.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---
*Developed by Antigravity AI for ISU SecOps.*
