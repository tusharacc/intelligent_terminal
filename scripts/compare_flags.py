#!/usr/bin/env python3
"""
Missing Flags Gap Analysis
Compares command_requirements.yaml (LLM-extracted from linuxcommand.org)
against actual Rust source implementations to identify unimplemented flags.
Outputs missing_flags.yaml
"""

import yaml
import os
import re
import sys

REQUIREMENTS_PATH = "command_requirements.yaml"
RUST_COMMANDS_DIR = "src-tauri/src/commands"
OUTPUT_PATH = "missing_flags.yaml"

# Commands that delegate to another command's module at runtime
# Key = command name, Value = delegate target command name
DELEGATES_TO = {
    "dir": "ls",
    "vdir": "ls",
}


def load_requirements():
    """Load the LLM-extracted flag requirements."""
    with open(REQUIREMENTS_PATH, "r") as f:
        return yaml.safe_load(f)


def get_rust_files():
    """Map command names to their Rust source files."""
    cmd_files = {}
    if not os.path.isdir(RUST_COMMANDS_DIR):
        print(f"ERROR: {RUST_COMMANDS_DIR} not found")
        sys.exit(1)

    for fname in os.listdir(RUST_COMMANDS_DIR):
        if fname.endswith(".rs") and fname != "mod.rs":
            cmd_name = fname[:-3]  # strip .rs
            cmd_files[cmd_name] = os.path.join(RUST_COMMANDS_DIR, fname)

    return cmd_files


def check_flag_in_source(flag_name, source_code):
    """
    Check if a flag is handled in the Rust source code.
    Looks for:
    - Direct string matching: parsed_flags.contains("flag")
    - Help text mentions (inside r#"..."# raw strings)
    - Unsupported flag error messages
    - Any string literal containing the flag
    """
    # Normalize flag for searching
    clean_flag = flag_name.strip()

    # Skip flags with = or [] in them (parameterized flags)
    # Check just the base flag name
    base_flag = clean_flag.split("=")[0].split("[")[0]

    # Check various patterns the code uses
    patterns = [
        f'"{base_flag}"',           # Direct string literal
        f'"{clean_flag}"',          # Full flag string
        f"contains(&\"{base_flag}\"",  # parsed_flags.contains
        f'"{base_flag}',            # Partial match in help text
        f'  {base_flag},',          # Help text: "  -q, --long-name"
        f'  {base_flag} ',          # Help text: "  --version   description"
        f', {base_flag} ',          # Help text: "-x, --long   description"
    ]

    for pattern in patterns:
        if pattern in source_code:
            return True

    return False


def analyze_gaps():
    """Compare requirements against implementations."""
    reqs = load_requirements()
    rust_files = get_rust_files()

    commands = reqs.get("commands", {})
    results = {}
    total_required = 0
    total_implemented = 0
    total_missing = 0

    for cmd_name, cmd_data in commands.items():
        rust_file = rust_files.get(cmd_name)

        if not rust_file or not os.path.exists(rust_file):
            # Command not implemented at all
            all_flags = []
            for flag_entry in cmd_data.get("flags", []):
                all_flags.extend(flag_entry.get("names", []))

            if all_flags:
                results[cmd_name] = {
                    "status": "NOT_IMPLEMENTED",
                    "rust_file": None,
                    "total_required": len(cmd_data.get("flags", [])),
                    "implemented": 0,
                    "missing": len(cmd_data.get("flags", [])),
                    "missing_flags": []
                }
                for flag_entry in cmd_data.get("flags", []):
                    results[cmd_name]["missing_flags"].append({
                        "names": flag_entry.get("names", []),
                        "description": flag_entry.get("description", "")
                    })
                total_required += len(cmd_data.get("flags", []))
                total_missing += len(cmd_data.get("flags", []))
            continue

        # Read source code
        with open(rust_file, "r") as f:
            source_code = f.read()

        # If this command delegates to another, also check the delegate's source
        if cmd_name in DELEGATES_TO:
            delegate_file = rust_files.get(DELEGATES_TO[cmd_name])
            if delegate_file and os.path.exists(delegate_file):
                with open(delegate_file, "r") as f:
                    source_code += "\n" + f.read()

        flag_entries = cmd_data.get("flags", [])
        missing_flags = []
        implemented_count = 0

        for flag_entry in flag_entries:
            flag_names = flag_entry.get("names", [])
            description = flag_entry.get("description", "")

            # A flag group is "implemented" if ANY of its names are found in source
            found = False
            for name in flag_names:
                if check_flag_in_source(name, source_code):
                    found = True
                    break

            if found:
                implemented_count += 1
            else:
                missing_flags.append({
                    "names": flag_names,
                    "description": description
                })

        total_required += len(flag_entries)
        total_implemented += implemented_count
        total_missing += len(missing_flags)

        if missing_flags:
            results[cmd_name] = {
                "status": "PARTIAL",
                "rust_file": rust_file,
                "total_required": len(flag_entries),
                "implemented": implemented_count,
                "missing": len(missing_flags),
                "missing_flags": missing_flags
            }

    # Build output
    output = {
        "metadata": {
            "title": "Missing Flags Gap Analysis",
            "requirements_source": REQUIREMENTS_PATH,
            "rust_source_dir": RUST_COMMANDS_DIR,
            "analysis_method": "Static source code comparison"
        },
        "summary": {
            "total_flags_required": total_required,
            "total_implemented": total_implemented,
            "total_missing": total_missing,
            "compliance_percentage": round(
                (total_implemented / total_required * 100) if total_required > 0 else 0, 1
            ),
            "commands_with_gaps": len(results),
            "commands_fully_compliant": len(commands) - len(results)
        },
        "gaps": results
    }

    return output


def main():
    print("=" * 60)
    print("  Missing Flags Gap Analysis")
    print("=" * 60)

    print(f"\n  Requirements: {REQUIREMENTS_PATH}")
    print(f"  Rust source:  {RUST_COMMANDS_DIR}/")

    output = analyze_gaps()

    with open(OUTPUT_PATH, "w") as f:
        yaml.dump(output, f, default_flow_style=False, sort_keys=False, width=120, allow_unicode=True)

    summary = output["summary"]
    print(f"\n  Results:")
    print(f"    Total flags required:   {summary['total_flags_required']}")
    print(f"    Implemented:            {summary['total_implemented']}")
    print(f"    Missing:                {summary['total_missing']}")
    print(f"    Compliance:             {summary['compliance_percentage']}%")
    print(f"    Commands with gaps:     {summary['commands_with_gaps']}")
    print(f"    Fully compliant:        {summary['commands_fully_compliant']}")

    print(f"\n  Saved to: {OUTPUT_PATH}")
    print("=" * 60)

    # Print per-command summary for gaps
    if output["gaps"]:
        print("\n  Commands with missing flags:")
        for cmd, data in output["gaps"].items():
            flag_names = []
            for mf in data["missing_flags"]:
                flag_names.extend(mf["names"])
            print(f"    {cmd}: {data['missing']}/{data['total_required']} missing → {', '.join(flag_names[:8])}")


if __name__ == "__main__":
    main()
