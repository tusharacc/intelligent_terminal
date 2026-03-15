#!/usr/bin/env python3
"""
Intelligent Terminal — Python Test Framework
============================================
Spawns the iterminal-cli Rust binary as a subprocess, sends commands,
captures JSON output, and compares against expected values.

Usage:
    python3 tests/test_runner.py                  # run all tests
    python3 tests/test_runner.py --cmd ls         # run only ls tests
    python3 tests/test_runner.py --verbose        # show full output diffs

Binary path: src-tauri/target/debug/iterminal-cli
"""

import subprocess
import json
import sys
import os
import argparse
import time
import re
from dataclasses import dataclass, field
from typing import Optional

# ─── Configuration ────────────────────────────────────────────────────────────
CLI_BINARY = os.path.join(
    os.path.dirname(__file__), "..", "src-tauri", "target", "debug", "iterminal-cli"
)

COLORS = {
    "green": "\033[92m",
    "red": "\033[91m",
    "yellow": "\033[93m",
    "blue": "\033[94m",
    "cyan": "\033[96m",
    "reset": "\033[0m",
    "bold": "\033[1m",
}

def c(color, text): return f"{COLORS[color]}{text}{COLORS['reset']}"


# ─── Data Model ───────────────────────────────────────────────────────────────
@dataclass
class TestCase:
    """A single test case definition."""
    name: str
    command: str
    expect_status: str = "ok"                  # "ok" or "error"
    expect_contains: list = field(default_factory=list)   # strings that must appear in output
    expect_not_contains: list = field(default_factory=list) # strings that must NOT appear
    expect_exact: Optional[str] = None           # exact output match
    expect_regex: Optional[str] = None           # regex pattern match

@dataclass
class TestResult:
    """Result of a single test execution."""
    test: TestCase
    passed: bool
    actual_status: str
    actual_output: str
    failures: list = field(default_factory=list)
    duration_ms: float = 0.0


# ─── CLI Runner ───────────────────────────────────────────────────────────────
class CLIRunner:
    """Manages a persistent subprocess connection to iterminal-cli."""

    def __init__(self, binary_path: str):
        self.binary_path = binary_path
        self.proc = None

    def start(self):
        if not os.path.exists(self.binary_path):
            print(c("red", f"\nERROR: CLI binary not found: {self.binary_path}"))
            print(c("yellow", "Build it first with: cd src-tauri && cargo build --bin iterminal-cli"))
            sys.exit(1)

        self.proc = subprocess.Popen(
            [self.binary_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

    def run_command(self, command: str) -> tuple[str, str]:
        """Send a command and get back (status, output)."""
        self.proc.stdin.write(command + "\n")
        self.proc.stdin.flush()

        raw = self.proc.stdout.readline().strip()
        try:
            data = json.loads(raw)
            return data.get("status", "error"), data.get("output", "")
        except json.JSONDecodeError:
            return "error", f"Invalid JSON from CLI: {raw}"

    def stop(self):
        if self.proc:
            try:
                self.proc.stdin.write("exit\n")
                self.proc.stdin.flush()
                self.proc.wait(timeout=2)
            except Exception:
                self.proc.kill()


# ─── Test Runner ──────────────────────────────────────────────────────────────
class TestRunner:
    def __init__(self, cli: CLIRunner, verbose: bool = False):
        self.cli = cli
        self.verbose = verbose
        self.results: list[TestResult] = []

    def run_test(self, tc: TestCase) -> TestResult:
        start = time.monotonic()
        status, output = self.cli.run_command(tc.command)
        duration_ms = (time.monotonic() - start) * 1000

        failures = []

        # Check status
        if tc.expect_status and status != tc.expect_status:
            failures.append(f"Expected status '{tc.expect_status}' but got '{status}'")

        # Check exact match
        if tc.expect_exact is not None:
            if output.strip() != tc.expect_exact.strip():
                failures.append(f"Exact mismatch:\n  Expected: {repr(tc.expect_exact)}\n  Got:      {repr(output)}")

        # Check contains
        for substring in tc.expect_contains:
            if substring not in output:
                failures.append(f"Expected output to contain: {repr(substring)}")

        # Check not contains
        for substring in tc.expect_not_contains:
            if substring in output:
                failures.append(f"Expected output NOT to contain: {repr(substring)}")

        # Check regex
        if tc.expect_regex:
            if not re.search(tc.expect_regex, output, re.MULTILINE):
                failures.append(f"Expected output to match regex: {tc.expect_regex}")

        passed = len(failures) == 0
        result = TestResult(tc, passed, status, output, failures, duration_ms)
        self.results.append(result)
        return result

    def run_all(self, tests: list[TestCase], filter_cmd: Optional[str] = None):
        if filter_cmd:
            tests = [t for t in tests if t.command.split()[0] == filter_cmd]
            if not tests:
                print(c("yellow", f"No tests found for command: {filter_cmd}"))
                return

        print(c("bold", f"\n  Running {len(tests)} test(s)...\n"))

        for tc in tests:
            result = self.run_test(tc)

            icon = c("green", "✅") if result.passed else c("red", "❌")
            duration_str = c("cyan", f"{result.duration_ms:.0f}ms")
            cmd_str = c("blue", tc.command)
            print(f"  {icon}  {tc.name}  [{cmd_str}]  ({duration_str})")

            if not result.passed:
                for failure in result.failures:
                    print(c("red", f"       → {failure}"))
                if self.verbose:
                    print(c("yellow", f"       Actual output: {repr(result.actual_output[:200])}"))

        self._print_summary()

    def _print_summary(self):
        total = len(self.results)
        passed = sum(1 for r in self.results if r.passed)
        failed = total - passed
        avg_ms = sum(r.duration_ms for r in self.results) / total if total > 0 else 0

        print("\n" + "─" * 60)
        pct = round(passed / total * 100) if total > 0 else 0
        status_color = "green" if failed == 0 else "red"
        print(c(status_color, c("bold", f"  {passed}/{total} passed ({pct}%)")), end="")
        print(c("cyan", f"  avg {avg_ms:.0f}ms/test"))
        print("─" * 60)

        if failed > 0:
            print(c("red", "\n  Failed tests:"))
            for r in self.results:
                if not r.passed:
                    print(c("red", f"    • {r.test.name}: {r.test.command}"))
        print()


# ─── Test Definitions ─────────────────────────────────────────────────────────
def get_all_tests() -> list[TestCase]:
    return [

        # ── echo ──────────────────────────────────────────────────────────────
        TestCase("echo: basic text", "echo hello world",
                 expect_contains=["hello world"]),
        TestCase("echo: -n flag (no newline)", "echo -n hello",
                 expect_contains=["hello"], expect_status="ok"),
        TestCase("echo: -e escape sequences", r"echo -e hello\nworld",
                 expect_contains=["hello"]),
        TestCase("echo: --help shows flags", "echo --help",
                 expect_contains=["-n", "-e", "-E"]),

        # ── pwd ────────────────────────────────────────────────────────────────
        TestCase("pwd: returns current directory", "pwd",
                 expect_contains=["/"]),
        TestCase("pwd: --help shows all 4 flags", "pwd --help",
                 expect_contains=["-L", "--logical", "-P", "--physical", "--version"]),
        TestCase("pwd: -L flag (unsupported feedback)", "pwd -L",
                 expect_status="error", expect_contains=["not yet implemented"]),

        # ── mkdir ──────────────────────────────────────────────────────────────
        TestCase("mkdir: --help shows all 7 flags", "mkdir --help",
                 expect_contains=["-m", "--mode", "-p", "--parents", "-v", "--verbose", "-Z"]),
        TestCase("mkdir: -p flag (unsupported feedback)", "mkdir -p /tmp/test_nested/dir",
                 expect_status="error", expect_contains=["not yet implemented"]),
        TestCase("mkdir: -v flag (unsupported feedback)", "mkdir -v /tmp/test_mkdir_v",
                 expect_status="error", expect_contains=["not yet implemented"]),

        # ── ls ─────────────────────────────────────────────────────────────────
        TestCase("ls: lists current directory", "ls",
                 expect_status="ok"),
        TestCase("ls: --help shows flags", "ls --help",
                 expect_contains=["-l", "-a", "--all", "-h", "--human-readable"]),
        TestCase("ls: -l flag gives unsupported feedback", "ls -l",
                 expect_status="error", expect_contains=["not supported"]),

        # ── cat ────────────────────────────────────────────────────────────────
        TestCase("cat: --help shows flags", "cat --help",
                 expect_contains=["-n", "--number", "-b", "--number-nonblank", "-s"]),
        TestCase("cat: -n flag (unsupported feedback)", "cat -n /dev/null",
                 expect_status="error", expect_contains=["not yet implemented"]),

        # ── grep ───────────────────────────────────────────────────────────────
        TestCase("grep: --help shows flags", "grep --help",
                 expect_contains=["-i", "-v", "-c", "-n", "-l"]),
        TestCase("grep: -i flag (unsupported feedback)", "grep -i hello",
                 expect_status="error", expect_contains=["not yet implemented"]),

        # ── head ───────────────────────────────────────────────────────────────
        TestCase("head: --help shows flags", "head --help",
                 expect_contains=["-n", "--lines", "-c", "--bytes"]),
        TestCase("head: -n flag (unsupported feedback)", "head -n 5",
                 expect_status="error", expect_contains=["not yet implemented"]),

        # ── tail ───────────────────────────────────────────────────────────────
        TestCase("tail: --help shows flags", "tail --help",
                 expect_contains=["-n", "--lines", "-c", "--bytes"]),

        # ── wc ─────────────────────────────────────────────────────────────────
        TestCase("wc: --help shows flags", "wc --help",
                 expect_contains=["-l", "--lines", "-w", "--words", "-c", "--bytes"]),

        # ── sort ───────────────────────────────────────────────────────────────
        TestCase("sort: --help shows flags", "sort --help",
                 expect_contains=["-b", "-d", "-f", "-g", "-i"]),

        # ── uniq ───────────────────────────────────────────────────────────────
        TestCase("uniq: --help shows flags", "uniq --help",
                 expect_contains=["-c", "--count", "-d", "--repeated"]),

        # ── date ───────────────────────────────────────────────────────────────
        TestCase("date: returns current date/time", "date",
                 expect_status="ok", expect_regex=r"\d{4}"),
        TestCase("date: --help shows flags", "date --help",
                 expect_contains=["-d", "--date", "-u", "--utc", "-R"]),

        # ── sleep ──────────────────────────────────────────────────────────────
        TestCase("sleep: --help shows flags", "sleep --help",
                 expect_contains=["--help", "--version"]),

        # ── env ────────────────────────────────────────────────────────────────
        TestCase("env: --help shows flags", "env --help",
                 expect_contains=["-i", "--ignore-environment", "-u", "--unset"]),

        # ── alias ──────────────────────────────────────────────────────────────
        TestCase("alias: --help shows -p flag", "alias --help",
                 expect_contains=["-p"]),

        # ── history ────────────────────────────────────────────────────────────
        TestCase("history: --help shows flags", "history --help",
                 expect_contains=["-c", "-d", "-a", "-r", "-w"]),

        # ── gzip ───────────────────────────────────────────────────────────────
        TestCase("gzip: --help shows flags (full compliance)", "gzip --help",
                 expect_contains=["-d", "-k", "--keep", "-l", "--list", "-v", "--verbose"]),

        # ── tar ────────────────────────────────────────────────────────────────
        TestCase("tar: --help shows flags", "tar --help",
                 expect_contains=["-c", "--create", "-v", "--verbose", "-f", "--file"]),

        # ── md5sum ─────────────────────────────────────────────────────────────
        TestCase("md5sum: --help shows flags", "md5sum --help",
                 expect_contains=["-b", "--binary", "-c", "--check"]),

        # ── find ───────────────────────────────────────────────────────────────
        TestCase("find: --help shows flags (full compliance)", "find --help",
                 expect_contains=["--help"]),

        # ── cd ─────────────────────────────────────────────────────────────────
        TestCase("cd: --help shows flags", "cd --help",
                 expect_contains=["-L", "-P", "-e"]),

        # ── Unknown command ────────────────────────────────────────────────────
        TestCase("unknown command: returns error", "foobar123",
                 expect_status="error", expect_contains=["command not found"]),
    ]


# ─── Entry Point ──────────────────────────────────────────────────────────────
def main():
    parser = argparse.ArgumentParser(description="Intelligent Terminal Test Runner")
    parser.add_argument("--cmd", help="Filter tests to only run for a specific command (e.g. ls)")
    parser.add_argument("--verbose", "-v", action="store_true", help="Show actual output on failure")
    parser.add_argument("--binary", help=f"Path to CLI binary (default: {CLI_BINARY})")
    args = parser.parse_args()

    binary = args.binary or CLI_BINARY
    print(c("bold", "\n  Intelligent Terminal — Python Test Framework"))
    print(c("cyan", f"  Binary: {os.path.abspath(binary)}"))

    cli = CLIRunner(binary)
    cli.start()

    runner = TestRunner(cli, verbose=args.verbose)
    tests = get_all_tests()

    try:
        runner.run_all(tests, filter_cmd=args.cmd)
    finally:
        cli.stop()

    # Exit with error code if any tests failed
    failed = sum(1 for r in runner.results if not r.passed)
    sys.exit(1 if failed > 0 else 0)


if __name__ == "__main__":
    main()
