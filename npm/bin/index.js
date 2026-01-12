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
let targetDir = '';

if (platform === 'win32') {
  targetDir = 'windows-amd64';
} else if (platform === 'darwin') {
  targetDir = 'macos-amd64';
} else if (platform === 'linux') {
  targetDir = 'linux-amd64';
} else {
  console.error(`Unsupported platform: ${platform}`);
  process.exit(1);
}

// Check if we are running from npx/global install (bundled) or dev
// In bundled mode, binary is in ../dist
// We might need to download it if not bundled? 
// For now, assume bundled via GitHub Action.

const binaryPath = path.join(__dirname, '..', 'dist', targetDir, binaryName);

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
  console.error('Ensure you installed via npm/npx or built the dist folder.');
  process.exit(1);
});
