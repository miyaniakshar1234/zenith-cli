import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import * as https from 'https';
import { execSync } from 'child_process';

const REPO = "miyaniakshar1234/zenith-cli";
const VERSION = "v1.2.0"; // Hardcoded for stability, or fetch latest
const BINARY_NAME = "zenith-cli";

function getTarget(): { dir: string, asset: string, exec: string } {
    const platform = os.platform();
    const arch = os.arch();

    let osType = '';
    let archType = 'amd64'; // Default to amd64 as we only build that

    if (platform === 'win32') osType = 'windows';
    else if (platform === 'darwin') osType = 'macos';
    else if (platform === 'linux') osType = 'linux';
    else throw new Error(`Unsupported platform: ${platform}`);

    // Adjust for naming convention
    const ext = platform === 'win32' ? '.exe' : '';
    const asset = `zenith-cli-${osType}-${archType}${ext}`;
    const dir = `${osType}-${archType}`;
    
    return { dir, asset, exec: BINARY_NAME + ext };
}

function downloadFile(url: string, dest: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const request = https.get(url, { timeout: 15000 }, (response) => {
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
                fs.unlink(dest, () => {});
                reject(err);
            });
        });

        request.on('error', (err) => {
            fs.unlink(dest, () => {});
            reject(err);
        });

        request.on('timeout', () => {
            request.destroy();
            reject(new Error("Download timed out"));
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
    } catch (error) {
        console.error("❌ Installation failed:", error);
        process.exit(1);
    }
}

install();
