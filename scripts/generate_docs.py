#!/usr/bin/env python3
"""
Generate per-command documentation files in docs/ folder.
Uses command_requirements.yaml (full requirements) and missing_flags.yaml (gaps)
to create one markdown file per command showing implemented vs missing flags.
"""

import yaml
import os
import sys

REQUIREMENTS_PATH = "command_requirements.yaml"
MISSING_PATH = "missing_flags.yaml"
DOCS_DIR = "docs"


def load_yaml(path):
    with open(path, "r") as f:
        return yaml.safe_load(f)


def generate_command_doc(cmd_name, req_data, missing_data):
    """Generate markdown documentation for a single command."""
    flags = req_data.get("flags", [])
    total = req_data.get("total_flags", len(flags))

    # Build set of missing flag names for lookup
    missing_flag_names = set()
    missing_entries = {}
    if missing_data:
        for mf in missing_data.get("missing_flags", []):
            for name in mf.get("names", []):
                missing_flag_names.add(name)
                missing_entries[name] = mf.get("description", "")

    implemented_count = total - (missing_data["missing"] if missing_data else 0)
    missing_count = missing_data["missing"] if missing_data else 0

    lines = []
    lines.append(f"# `{cmd_name}` — Command Reference")
    lines.append("")
    lines.append(f"**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/{cmd_name}1.php)")
    lines.append(f"**Rust module**: `src-tauri/src/commands/{cmd_name}.rs`")
    lines.append("")

    # Status badge
    if missing_count == 0:
        lines.append(f"**Status**: ✅ Fully Implemented ({implemented_count}/{total} flags)")
    elif implemented_count == 0:
        lines.append(f"**Status**: ❌ Not Implemented ({missing_count}/{total} flags missing)")
    else:
        pct = round(implemented_count / total * 100)
        lines.append(f"**Status**: ⚠️ Partial ({implemented_count}/{total} flags, {pct}% complete)")
    lines.append("")

    # --- Implemented Flags ---
    lines.append("---")
    lines.append("")
    lines.append("## ✅ Implemented Flags")
    lines.append("")

    has_implemented = False
    for flag_entry in flags:
        names = flag_entry.get("names", [])
        desc = flag_entry.get("description", "")

        # Check if ALL names in this group are missing
        all_missing = all(n in missing_flag_names for n in names)
        if not all_missing:
            flag_str = ", ".join(f"`{n}`" for n in names)
            lines.append(f"- {flag_str}")
            lines.append(f"  - {desc}")
            lines.append("")
            has_implemented = True

    if not has_implemented:
        lines.append("_No flags currently implemented._")
        lines.append("")

    # --- Missing Flags ---
    lines.append("---")
    lines.append("")
    lines.append("## ❌ Missing Flags (To Implement)")
    lines.append("")

    has_missing = False
    for flag_entry in flags:
        names = flag_entry.get("names", [])
        desc = flag_entry.get("description", "")

        all_missing = all(n in missing_flag_names for n in names)
        if all_missing and names:
            flag_str = ", ".join(f"`{n}`" for n in names)
            lines.append(f"- {flag_str}")
            lines.append(f"  - {desc}")
            lines.append("")
            has_missing = True

    if not has_missing:
        lines.append("_All flags are implemented!_ 🎉")
        lines.append("")

    return "\n".join(lines)


def main():
    print("=" * 60)
    print("  Generating Per-Command Documentation")
    print("=" * 60)

    reqs = load_yaml(REQUIREMENTS_PATH)
    missing = load_yaml(MISSING_PATH)

    commands = reqs.get("commands", {})
    gaps = missing.get("gaps", {})

    os.makedirs(DOCS_DIR, exist_ok=True)

    created = 0
    for cmd_name, cmd_data in commands.items():
        missing_data = gaps.get(cmd_name, None)
        doc_content = generate_command_doc(cmd_name, cmd_data, missing_data)

        filepath = os.path.join(DOCS_DIR, f"{cmd_name}.md")
        with open(filepath, "w") as f:
            f.write(doc_content)

        status = "⚠️" if missing_data else "✅"
        print(f"  {status} {filepath}")
        created += 1

    print(f"\n  Generated {created} command docs in {DOCS_DIR}/")
    print("=" * 60)


if __name__ == "__main__":
    main()
