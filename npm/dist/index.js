#!/usr/bin/env node
"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const child_process_1 = require("child_process");
const path = __importStar(require("path"));
const os = __importStar(require("os"));
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
}
else if (platform === 'darwin') {
    targetDir = 'macos-amd64';
}
else if (platform === 'linux') {
    targetDir = 'linux-amd64';
}
else {
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
}
// Path structure:
// package_root/
//   dist/index.js
//   binaries/linux-amd64/zenith-cli
const binaryPath = path.join(__dirname, '..', 'binaries', targetDir, binaryName);
const child = (0, child_process_1.spawn)(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    windowsHide: true
});
child.on('close', (code) => {
    process.exit(code);
});
child.on('error', (err) => {
    console.error('Failed to start Zenith CLI:', err);
    console.error('Binary path tried:', binaryPath);
    console.error('Ensure the package was installed correctly.');
    process.exit(1);
});
