#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');

// Detect Platform
const platform = os.platform(); // 'win32', 'linux', 'darwin'
const arch = os.arch(); // 'x64', 'arm64'

// Map to binary name
let binaryName = 'zenith-cli';
if (platform === 'win32') {
  binaryName += '.exe';
}

// Locate binary in dist folder (we will bundle them here)
// Structure: npm/dist/windows-x64/zenith-cli.exe
let targetDir = '';

if (platform === 'win32') {
  targetDir = 'windows-amd64';
} else if (platform === 'darwin') {
  targetDir = 'macos-amd64'; // Assuming amd64 for now, can add arm64 logic
} else if (platform === 'linux') {
  targetDir = 'linux-amd64';
} else {
  console.error(`Unsupported platform: ${platform}`);
  process.exit(1);
}

const binaryPath = path.join(__dirname, '..', 'dist', targetDir, binaryName);

// Spawn the binary and pass all arguments
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true
});

child.on('close', (code) => {
  process.exit(code);
});

child.on('error', (err) => {
  console.error('Failed to start Zenith CLI:', err);
  console.error('Binary path tried:', binaryPath);
  process.exit(1);
});
