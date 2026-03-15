import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// Terminal UI configuration
const term = new Terminal({
  cursorBlink: true,
  theme: {
    background: '#1e1e1e',
    foreground: '#d4d4d4',
    cursor: '#ffffff'
  },
  fontFamily: 'Menlo, Monaco, "Courier New", monospace',
  fontSize: 14
});

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

let currentInput = '';

window.addEventListener("DOMContentLoaded", () => {
  const container = document.getElementById('terminal-container');
  term.open(container);
  fitAddon.fit();

  term.writeln('\x1b[1;32mWelcome to Intelligent Terminal\x1b[0m');
  term.writeln('A cross-platform Rust-based terminal.');
  term.writeln('');
  prompt();

  // Basic Input handling
  term.onKey(({ key, domEvent }) => {
    const printable = !domEvent.altKey && !domEvent.altGraphKey && !domEvent.ctrlKey && !domEvent.metaKey;

    if (domEvent.keyCode === 13) {
      // Enter
      term.writeln('');
      handleCommand(currentInput);
      currentInput = '';
    } else if (domEvent.keyCode === 8) {
      // Backspace
      if (currentInput.length > 0) {
        currentInput = currentInput.slice(0, -1);
        term.write('\b \b');
      }
    } else if (printable) {
      currentInput += key;
      term.write(key);
    }
  });

  window.addEventListener('resize', () => {
    fitAddon.fit();
  });
});

async function handleCommand(cmd) {
  if (cmd.trim() === '') {
    prompt();
    return;
  }
  
  if (cmd.trim() === 'clear') {
    term.clear();
    prompt();
    return;
  }

  try {
    // Send command to Rust backend
    const output = await invoke('execute_command', { command: cmd });
    if (output) {
      // replace newlines with \r\n for xterm
      term.writeln(output.replace(/\n/g, '\r\n'));
    }
  } catch (err) {
    term.writeln(`\x1b[1;31mError:\x1b[0m ${err}`);
  }
  prompt();
}

function prompt() {
  term.write('\x1b[1;34m$ \x1b[0m');
}
