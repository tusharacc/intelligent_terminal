# Test Runner Creation Guide

This document explains how to set up and use the Python-based test runner for the Intelligent Terminal.

## Architecture

The testing system consists of two parts:
1.  **Rust CLI Binary (`iterminal-cli`)**: A lightweight wrapper around the core terminal logic that takes commands via `stdin` and outputs JSON to `stdout`.
2.  **Python Test Harness (`test_runner.py`)**: A script that manages the Rust process, sends test cases, and verifies the JSON output.

## Initial Setup

### 1. Configure the Rust Binary
In `src-tauri/Cargo.toml`, add a binary target:

```toml
[[bin]]
name = "iterminal-cli"
path = "src/cli.rs"
```

Create `src-tauri/src/cli.rs` to read lines from `stdin` and call the command executor.

### 2. Build the CLI
Run this from the project root:
```bash
cd src-tauri && cargo build --bin iterminal-cli
```

### 3. Create the Python Harness
Create `tests/test_runner.py` using the `subprocess` module to interact with the binary:

```python
import subprocess
import json

# Start the binary
proc = subprocess.Popen(['./path/to/binary'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)

# Send command
proc.stdin.write("ls -la\n")
proc.stdin.flush()

# Read response
response = json.loads(proc.stdout.readline())
print(response['output'])
```

## Running Tests

From the project root:
```bash
python3 tests/test_runner.py
```

## Adding New Tests
Test cases are defined in `test_runner.py`'s `get_all_tests()` function. Simply add a new `TestCase` instance with the command and expected substrings or regex.
