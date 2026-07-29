# blockscan Development Setup

Welcome to the `blockscan` project! This guide will help you set up your local development environment.

## Prerequisites

1. **Rust and Cargo**: You must have Rust installed. The easiest way is via [rustup](https://rustup.rs/).
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Git**: Required to clone the repository and manage version control.

## Building Locally

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/blockscan.git
   cd blockscan
   ```

2. **Run Cargo Build**:
   To compile the project in debug mode:
   ```bash
   cargo build
   ```
   To compile an optimized release binary:
   ```bash
   cargo build --release
   ```

3. **Running the App**:
   You can run the app directly through cargo:
   ```bash
   cargo run -- https://example.com
   ```

## Development Guidelines

- **Format Code**: Ensure your code is properly formatted before committing:
  ```bash
  cargo fmt
  ```
- **Linting**: Run `clippy` to catch common mistakes and improve code quality:
  ```bash
  cargo clippy -- -D warnings
  ```
- **Testing**: Run the test suite:
  ```bash
  cargo test
  ```

Thank you for contributing to `blockscan`!
