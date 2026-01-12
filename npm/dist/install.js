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
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
const os = __importStar(require("os"));
const https = __importStar(require("https"));
const REPO = "miyaniakshar1234/zenith-cli";
const VERSION = "v1.2.0"; // Hardcoded for stability, or fetch latest
const BINARY_NAME = "zenith-cli";
function getTarget() {
    const platform = os.platform();
    const arch = os.arch();
    let osType = '';
    let archType = 'amd64'; // Default to amd64 as we only build that
    if (platform === 'win32')
        osType = 'windows';
    else if (platform === 'darwin')
        osType = 'macos';
    else if (platform === 'linux')
        osType = 'linux';
    else
        throw new Error(`Unsupported platform: ${platform}`);
    // Adjust for naming convention
    const ext = platform === 'win32' ? '.exe' : '';
    const asset = `zenith-cli-${osType}-${archType}${ext}`;
    const dir = `${osType}-${archType}`;
    return { dir, asset, exec: BINARY_NAME + ext };
}
function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const request = https.get(url, (response) => {
            // Handle redirects
            if (response.statusCode === 302 || response.statusCode === 301) {
                if (!response.headers.location) {
                    reject(new Error("Redirect location missing"));
                    return;
                }
                downloadFile(response.headers.location, dest).then(resolve).catch(reject);
                return;
            }
            if (response.statusCode !== 200) {
                reject(new Error(`Failed to download: ${response.statusCode}`));
                return;
            }
            const file = fs.createWriteStream(dest);
            response.pipe(file);
            file.on('finish', () => {
                file.close();
                resolve();
            });
            file.on('error', (err) => {
                fs.unlink(dest, () => { });
                reject(err);
            });
        });
        request.on('error', (err) => {
            reject(err);
        });
    });
}
async function install() {
    try {
        const target = getTarget();
        const binDir = path.join(__dirname, '..', 'binaries', target.dir);
        const binPath = path.join(binDir, target.exec);
        if (fs.existsSync(binPath)) {
            console.log("✅ Zenith CLI binary already exists.");
            return;
        }
        console.log(`🚀 Zenith CLI Installer (${target.dir})`);
        console.log(`   Preparing to download ${target.asset}...`);
        if (!fs.existsSync(binDir)) {
            fs.mkdirSync(binDir, { recursive: true });
        }
        // Use 'latest' or specific version tag
        const url = `https://github.com/${REPO}/releases/download/${VERSION}/${target.asset}`;
        await downloadFile(url, binPath);
        if (os.platform() !== 'win32') {
            fs.chmodSync(binPath, 0o755);
        }
        console.log(`✅ Successfully installed to ${binPath}`);
    }
    catch (error) {
        console.error("❌ Installation failed:", error);
        process.exit(1);
    }
}
install();
