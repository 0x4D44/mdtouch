# mdtouch

`mdtouch` is a command-line utility written in Rust that mimics the behavior of the Unix `touch` command. It is designed to be used primarily on Windows but is cross-platform.

## Overview

The tool performs two main functions:
1.  **Create:** If a specified file does not exist, it creates an empty file.
2.  **Update:** If the file already exists, it updates the file's access and modification timestamps to the current time.

## Project Structure

*   `Cargo.toml`: Project configuration and dependencies (uses `filetime`).
*   `src/main.rs`: Contains the application entry point, logic, and unit tests.

## Usage

### Build

```bash
cargo build --release
```

### Run

```bash
# General usage
cargo run -- <file_path> [file_path...]

# Get help
cargo run -- -h
```

**Examples:**

*   `mdtouch newfile.txt` (Creates `newfile.txt`)
*   `mdtouch existing.log` (Updates timestamps of `existing.log`)

### Testing

The project includes built-in unit tests within `src/main.rs`.

```bash
cargo test
```

## Development

*   **Language:** Rust (Edition 2021)
*   **Dependencies:** `filetime` for handling file timestamps.
*   **Architecture:** A simple CLI with argument parsing in `main` and core logic in `touch_file`.
*   **Tests:** Unit tests verify file creation, timestamp updates, and help message content.
