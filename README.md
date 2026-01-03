# mdtouch

`mdtouch` is a lightweight, cross-platform command-line utility written in Rust. It mimics the behavior of the standard Unix `touch` command, making it particularly useful for Windows developers who need a native way to create files or update timestamps without relying on WSL or external shell tools.

## Features

- **Create Files:** Instantly create new, empty files if they do not exist.
- **Update Timestamps:** Update the access and modification timestamps of existing files to the current time.
- **Cross-Platform:** Works natively on Windows, Linux, and macOS.
- **Zero Dependencies:** (Runtime) compiled into a single static binary.

## Installation

### From Source

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
git clone https://github.com/your-username/mdtouch.git
cd mdtouch
cargo install --path .
```

This will install the `mdtouch` binary to your Cargo bin directory (usually `~/.cargo/bin`), allowing you to run it globally.

## Usage

The syntax is compatible with the standard `touch` command for basic operations.

```bash
mdtouch [OPTIONS] <file> [file...]
```

### Examples

**1. Create a new file:**
If `newfile.txt` does not exist, it will be created.
```bash
mdtouch newfile.txt
```

**2. Update timestamps of an existing file:**
If `log.txt` exists, its "Last Modified" and "Last Accessed" times will be updated to now.
```bash
mdtouch log.txt
```

**3. Touch multiple files:**
```bash
mdtouch file1.rs file2.rs file3.rs
```

### Options

| Option | Description |
| :--- | :--- |
| `-h`, `-?` | Display help message and exit. |

## Development

### Prerequisites
*   Rust (latest stable)

### Build
```bash
cargo build --release
```

### Testing
The project maintains high code coverage (>99%). You can run the test suite using standard cargo commands.

**Unit & Integration Tests:**
```bash
cargo test
```

**Linting & Formatting:**
```bash
cargo fmt --check
cargo clippy --all-targets
```

## License

This project is open-source.
