# LogGate Agent

LogGate Agent is a lightweight, high-performance logging and metrics collection agent built in Rust.

## Prerequisites

To build and run this project, you need the Rust toolchain (compiler, Cargo package manager, and standard library).

### Install Rust (via `rustup`)
If Rust is not already installed on your system:

- **macOS / Linux:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Follow the on-screen instructions, then restart your shell or run:
  ```bash
  source "$HOME/.cargo/env"
  ```

- **Windows:**
  Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

Verify installation:
```bash
rustc --version
cargo --version
```

---

## Getting Started

### 1. Build and Run
From the root of the `loggate-agent` directory:

```bash
# Run in development mode
cargo run

# Check the code for compile errors without producing binaries
cargo check

# Build debug binary (output to target/debug/loggate-agent)
cargo build
```

### 2. Run Tests
```bash
cargo test
```

### 3. Production Release Build
To generate an optimized release binary:
```bash
cargo build --release
```
The optimized binary will be located at:
```text
target/release/loggate-agent
```

---

## Project Structure

```text
loggate-agent/
├── Cargo.toml    # Manifest and dependency configurations
├── Cargo.lock    # Exact dependency versions
└── src/
    └── main.rs   # Agent entry point
```
