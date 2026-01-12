#!/usr/bin/env node
import { spawn } from 'child_process';
import * as path from 'path';
import * as os from 'os';

// Detect Platform
const platform = os.platform();
const arch = os.arch();

// Map to binary name
let binaryName = 'zenith-cli';
if (platform === 'win32') {
  binaryName += '.exe';
}

// Locate binary in binaries folder (populated by CI)
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

// Path structure:
// package_root/
//   dist/index.js
//   binaries/linux-amd64/zenith-cli
const binaryPath = path.join(__dirname, '..', 'binaries', targetDir, binaryName);

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true
});

child.on('close', (code: number) => {
  process.exit(code);
});

child.on('error', (err: Error) => {
  console.error('Failed to start Zenith CLI:', err);
  console.error('Binary path tried:', binaryPath);
  console.error('Ensure the package was installed correctly.');
  process.exit(1);
});
