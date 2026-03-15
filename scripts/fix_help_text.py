#!/usr/bin/env python3
"""
Fix --help output for all Rust command modules.
Reads command_requirements.yaml and replaces the --help text in each .rs file
with a properly formatted flag listing.
"""

import yaml
import os
import re
import sys

REQUIREMENTS_PATH = "command_requirements.yaml"
RUST_DIR = "src-tauri/src/commands"


def load_requirements():
    with open(REQUIREMENTS_PATH, "r") as f:
        return yaml.safe_load(f)


def generate_help_text(cmd_name, flags):
    """Generate a properly formatted help text string for a command."""
    lines = []
    lines.append(f"{cmd_name} - Linuxcommand.org Standard Parity")
    lines.append("")

    for flag_entry in flags:
        names = flag_entry.get("names", [])
        desc = flag_entry.get("description", "")

        # Format: "  -x, --long-name     description"
        flag_str = ", ".join(names)

        # Truncate description if too long for single line
        if len(desc) > 80:
            desc = desc[:77] + "..."

        # Pad the flag part for alignment
        padded = f"  {flag_str}"
        if len(padded) < 24:
            padded = padded.ljust(24)
        else:
            padded = padded + "  "

        lines.append(f"{padded}{desc}")

    return "\n".join(lines)


def fix_help_in_file(rust_file, cmd_name, help_text):
    """Replace the --help return text in a Rust source file."""
    with open(rust_file, "r") as f:
        content = f.read()

    # Pattern: find the --help block and replace the return Ok(r#"..."#.to_string());
    # We look for the pattern: if args.contains(&"--help") ... return Ok(r#"..."#.to_string());
    # The help text is between r#" and "#.to_string()
    
    # Match the entire help block
    pattern = r'(if args\.contains\(&"--help"\)[^\n]*\{?\s*\n\s*return Ok\(r#")([^"]*?|(?:[^#]|#(?!"))*?)("#\.to_string\(\)\);)'
    
    match = re.search(pattern, content, re.DOTALL)
    if not match:
        # Try alternative pattern without the if block wrapper (for single-line returns)
        pattern2 = r'(return Ok\(r#")([^"]*?|(?:[^#]|#(?!"))*?)("#\.to_string\(\)\);\s*\n\s*\})'
        match = re.search(pattern2, content, re.DOTALL)
        if not match:
            return "no_match"

    # Replace just the help text content
    old_text = match.group(2)
    new_content = content[:match.start(2)] + help_text + content[match.end(2):]

    with open(rust_file, "w") as f:
        f.write(new_content)

    return "fixed"


def main():
    print("=" * 60)
    print("  Fixing --help Output for All Commands")
    print("=" * 60)

    reqs = load_requirements()
    commands = reqs.get("commands", {})

    fixed = 0
    skipped = 0
    errors = 0

    for cmd_name, cmd_data in commands.items():
        rust_file = os.path.join(RUST_DIR, f"{cmd_name}.rs")
        if not os.path.exists(rust_file):
            skipped += 1
            continue

        flags = cmd_data.get("flags", [])
        help_text = generate_help_text(cmd_name, flags)

        result = fix_help_in_file(rust_file, cmd_name, help_text)

        if result == "fixed":
            print(f"  ✅ {cmd_name}: help text updated ({len(flags)} flags)")
            fixed += 1
        elif result == "no_match":
            print(f"  ❌ {cmd_name}: could not find help pattern")
            errors += 1
        else:
            print(f"  ⏭️  {cmd_name}: {result}")
            skipped += 1

    print(f"\n  Results: {fixed} fixed, {skipped} skipped, {errors} errors")
    print("=" * 60)


if __name__ == "__main__":
    main()
