#!/usr/bin/env python3
"""
LLM-Powered Flag Extraction Script
Uses Ollama (llama3:latest) to intelligently extract command flags
from linuxcommand.org man pages and outputs a YAML requirements doc.
"""

import urllib.request
import re
import json
import yaml
import sys
import time
import html

OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL = "llama3:latest"

INDEX_URL = "https://linuxcommand.org/lc3_man_page_index.php"

TARGET_CMDS = [
    "pwd", "cd", "echo", "env", "printenv", "set", "export", "unset",
    "alias", "history", "sleep", "date", "seq",
    "ls", "dir", "vdir", "cp", "mv", "rm", "mkdir", "rmdir", "touch",
    "cat", "head", "tail", "tac", "realpath", "basename", "dirname",
    "grep", "wc", "sort", "uniq", "shuf", "nl", "base64",
    "md5sum", "cksum", "sha512sum", "b2sum",
    "find", "gzip", "gunzip", "zcat", "tar",
    "nslookup", "ping", "netstat", "nmap", "traceroute", "tracert"
]

SYSTEM_PROMPT = """You are a precise data extraction assistant. You will be given the raw text of a Linux man page.

Your task is to extract ALL command-line flags/options from the man page.

Rules:
1. Extract EVERY flag mentioned, including short flags (-x) and long flags (--xxx).
2. For each flag, provide a concise description of what it does.
3. If a short flag and long flag are synonyms (e.g. -v, --verbose), group them together.
4. Output ONLY valid JSON, nothing else. No markdown, no commentary.
5. Use this exact JSON schema:
[
  {
    "flags": ["-x", "--long-name"],
    "description": "What this flag does"
  }
]

IMPORTANT: Output ONLY the JSON array. No other text before or after."""


def fetch_index():
    """Fetch the command index page and extract command -> URL mappings."""
    req = urllib.request.Request(INDEX_URL, headers={'User-Agent': 'Mozilla/5.0'})
    with urllib.request.urlopen(req) as response:
        page_html = response.read().decode('utf-8')

    links = re.findall(r'<a href="(lc3_man_pages/[^"]+)">([^<]+)</a>', page_html)
    return {cmd.strip(): href for href, cmd in links}


def fetch_man_page(cmd, href):
    """Fetch and clean the man page text for a specific command."""
    full_url = "https://linuxcommand.org/" + href
    req = urllib.request.Request(full_url, headers={'User-Agent': 'Mozilla/5.0'})
    with urllib.request.urlopen(req) as response:
        page_html = response.read().decode('utf-8')

    # Extract <pre> content (where the man page text lives)
    pre_match = re.search(r'<pre>(.*?)</pre>', page_html, re.DOTALL | re.IGNORECASE)
    if not pre_match:
        return None

    text = pre_match.group(1)
    # Decode HTML entities
    text = html.unescape(text)
    # Strip HTML tags
    text = re.sub(r'<[^>]+>', '', text)

    # Truncate if excessively long to fit in LLM context
    if len(text) > 8000:
        text = text[:8000] + "\n... [truncated]"

    return text


def query_ollama(man_text, cmd):
    """Send the man page text to Ollama and get structured flag data."""
    prompt = f"Extract all command-line flags from this '{cmd}' man page:\n\n{man_text}"

    payload = json.dumps({
        "model": MODEL,
        "prompt": prompt,
        "system": SYSTEM_PROMPT,
        "stream": False,
        "options": {
            "temperature": 0.1,
            "num_predict": 4096
        }
    }).encode('utf-8')

    req = urllib.request.Request(
        OLLAMA_URL,
        data=payload,
        headers={'Content-Type': 'application/json'},
        method='POST'
    )

    with urllib.request.urlopen(req, timeout=120) as response:
        result = json.loads(response.read().decode('utf-8'))

    raw = result.get("response", "")

    # Try to extract JSON from the response (LLM may wrap it in markdown)
    json_match = re.search(r'\[.*\]', raw, re.DOTALL)
    if json_match:
        try:
            return json.loads(json_match.group(0))
        except json.JSONDecodeError:
            pass

    return None


def build_yaml(all_flags):
    """Convert the extracted flags dict into a structured YAML requirements doc."""
    yaml_data = {
        "metadata": {
            "title": "Command Flag Requirements",
            "source": "linuxcommand.org",
            "extraction_method": "LLM (Ollama llama3:latest)",
            "generated_at": time.strftime("%Y-%m-%d %H:%M:%S")
        },
        "commands": {}
    }

    for cmd, flags in all_flags.items():
        cmd_entry = {
            "total_flags": len(flags),
            "flags": []
        }
        for flag_obj in flags:
            cmd_entry["flags"].append({
                "names": flag_obj.get("flags", []),
                "description": flag_obj.get("description", "No description available")
            })
        yaml_data["commands"][cmd] = cmd_entry

    return yaml_data


def main():
    output_path = "command_requirements.yaml"

    print("=" * 60)
    print("  LLM-Powered Flag Extraction (Ollama + llama3)")
    print("=" * 60)

    # Step 1: Fetch command index
    print("\n[1/3] Fetching command index from linuxcommand.org...")
    try:
        cmd_map = fetch_index()
        print(f"      Found {len(cmd_map)} commands in index.")
    except Exception as e:
        print(f"      ERROR: Failed to fetch index: {e}")
        sys.exit(1)

    # Step 2: For each target command, fetch man page and query LLM
    print(f"\n[2/3] Processing {len(TARGET_CMDS)} target commands with Ollama...")
    all_flags = {}
    failed = []

    for i, cmd in enumerate(TARGET_CMDS):
        progress = f"[{i+1}/{len(TARGET_CMDS)}]"

        if cmd not in cmd_map:
            print(f"  {progress} {cmd}: NOT FOUND in index, skipping")
            continue

        print(f"  {progress} {cmd}: fetching man page...", end="", flush=True)

        try:
            man_text = fetch_man_page(cmd, cmd_map[cmd])
            if not man_text:
                print(" NO <pre> CONTENT FOUND")
                failed.append(cmd)
                continue
        except Exception as e:
            print(f" FETCH ERROR: {e}")
            failed.append(cmd)
            continue

        print(" querying LLM...", end="", flush=True)

        try:
            flags = query_ollama(man_text, cmd)
            if flags:
                all_flags[cmd] = flags
                print(f" ✅ {len(flags)} flags extracted")
            else:
                print(" ❌ LLM returned invalid JSON")
                failed.append(cmd)
        except Exception as e:
            print(f" LLM ERROR: {e}")
            failed.append(cmd)

    # Step 3: Write YAML
    print(f"\n[3/3] Writing YAML requirements document...")
    yaml_data = build_yaml(all_flags)

    with open(output_path, "w") as f:
        yaml.dump(yaml_data, f, default_flow_style=False, sort_keys=False, width=120, allow_unicode=True)

    print(f"      Saved to: {output_path}")

    # Summary
    print("\n" + "=" * 60)
    print(f"  SUMMARY")
    print(f"  Commands processed: {len(all_flags)}/{len(TARGET_CMDS)}")
    total_flags = sum(len(f) for f in all_flags.values())
    print(f"  Total flags extracted: {total_flags}")
    if failed:
        print(f"  Failed commands: {', '.join(failed)}")
    print("=" * 60)


if __name__ == "__main__":
    main()
