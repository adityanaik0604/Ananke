# 🖥️ Ananke — Rust System Monitor

A lightweight **system monitoring tool written in Rust** that displays real-time information about CPU usage, memory usage, running processes, and network activity directly in the terminal.

This project demonstrates how Rust can be used for **efficient systems programming and performance monitoring**.

---

## 🚀 Features

* 📊 **CPU Monitoring**
  Displays usage for each CPU core in real time.

* 💾 **Memory Monitoring**
  Shows total and used system memory.

* ⚙️ **Process Monitoring**
  Lists the top running processes sorted by CPU usage.

* 🌐 **Network Monitoring**
  Tracks received and transmitted bytes for network interfaces.

* 🔄 **Live Refresh**
  Updates system information every second.

---

## 🧱 Project Structure

```
ANANKE/
├── src/
│   ├── main.rs        # Main program loop and system orchestration
│   ├── cpu.rs         # CPU usage collection
│   ├── memory.rs      # Memory statistics
│   ├── process.rs     # Process monitoring logic
│   ├── network.rs     # Network statistics
│   └── ui.rs          # Terminal output rendering
│
├── Cargo.toml         # Project configuration and dependencies
├── Cargo.lock
└── README.md
```

The project follows a **modular architecture**, where each system component (CPU, memory, network, processes) is handled by its own module.

---

## ⚙️ Installation

### 1️⃣ Clone the repository

```
git clone https://github.com/adityanaik0604/ananke.git
cd ananke
```

### 2️⃣ Build the project

```
cargo build
```

### 3️⃣ Run the monitor

```
cargo run
```

For optimized performance:

```
cargo run --release
```

---

## 📷 Example Output

```
=== SYSTEM MONITOR ===

CPU Usage
Core 0: 14.2%
Core 1: 11.5%
Core 2: 9.7%

Memory
Used: 5423 MB / 16000 MB

Top Processes
chrome.exe - 18.3%
discord.exe - 7.2%

Network
WiFi: RX 102034 bytes | TX 23012 bytes
```

---

## 📦 Dependencies

This project uses the following Rust crates:

* **sysinfo** – Collect system information such as CPU, memory, and processes
* **tokio** – Asynchronous runtime (optional future improvements)

All dependencies are managed through **Cargo**.

---

## 🎯 Learning Goals

This project was built to explore:

* Rust systems programming
* Interacting with operating system metrics
* Modular Rust project architecture
* Building terminal-based monitoring tools

---

## 🤝 Contributing

Contributions are welcome!
If you have ideas for improvements or new features, feel free to open an issue or submit a pull request.

---

## 📜 License

This project is open-source and available under the **MIT License**.

---

## 👨‍💻 Author

Developed as a learning project in **Rust systems programming**.
