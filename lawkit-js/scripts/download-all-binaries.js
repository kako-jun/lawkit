#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const LAWKIT_VERSION = require('../package.json').version;
const BINARY_DIR = path.join(__dirname, '..', 'bin');

// All platform configurations
const PLATFORMS = [
  { 
    name: 'linux-x64', 
    file: 'lawkit-linux-x86_64.tar.gz',
    binaryName: 'lawkit',
    subdir: 'linux-x64'
  },
  { 
    name: 'linux-arm64', 
    file: 'lawkit-linux-aarch64.tar.gz',
    binaryName: 'lawkit',
    subdir: 'linux-arm64'
  },
  { 
    name: 'darwin-x64', 
    file: 'lawkit-macos-x86_64.tar.gz',
    binaryName: 'lawkit',
    subdir: 'darwin-x64'
  },
  { 
    name: 'darwin-arm64', 
    file: 'lawkit-macos-aarch64.tar.gz',
    binaryName: 'lawkit',
    subdir: 'darwin-arm64'
  },
  { 
    name: 'win32-x64', 
    file: 'lawkit-windows-x86_64.zip',
    binaryName: 'lawkit.exe',
    subdir: 'win32-x64'
  }
];

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading ${path.basename(dest)}...`);
    const file = fs.createWriteStream(dest);
    
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      
      if (response.statusCode !== 200) {
        reject(new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`));
        return;
      }
      
      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        resolve();
      });
      
      file.on('error', (err) => {
        fs.unlink(dest, () => {}); // Delete the file async
        reject(err);
      });
    }).on('error', (err) => {
      reject(err);
    });
  });
}

async function extractArchive(archivePath, extractDir) {
  if (archivePath.endsWith('.zip')) {
    // Use Node.js built-in for cross-platform ZIP extraction
    const { execSync } = require('child_process');
    try {
      // Try with PowerShell on Windows
      if (process.platform === 'win32') {
        execSync(`powershell -command "Expand-Archive -Path '${archivePath}' -DestinationPath '${extractDir}' -Force"`, { stdio: 'inherit' });
      } else {
        // For Linux/macOS, use Python's zipfile module (more widely available than unzip)
        execSync(`python3 -c "import zipfile; zipfile.ZipFile('${archivePath}').extractall('${extractDir}')"`, { stdio: 'inherit' });
      }
    } catch (error) {
      // Fallback to manual extraction using Node.js modules
      const AdmZip = require('adm-zip');
      const zip = new AdmZip(archivePath);
      zip.extractAllTo(extractDir, true);
    }
  } else if (archivePath.endsWith('.tar.gz')) {
    execSync(`tar -xzf "${archivePath}" -C "${extractDir}"`, { stdio: 'inherit' });
  }
}

async function downloadPlatformBinary(platform) {
  const downloadUrl = `https://github.com/kako-jun/lawkit/releases/download/v${LAWKIT_VERSION}/${platform.file}`;
  const platformDir = path.join(BINARY_DIR, platform.subdir);
  const archivePath = path.join(platformDir, platform.file);
  
  // Create platform-specific directory
  if (!fs.existsSync(platformDir)) {
    fs.mkdirSync(platformDir, { recursive: true });
  }
  
  // Always download fresh binaries for release builds
  const binaryPath = path.join(platformDir, platform.binaryName);
  if (fs.existsSync(binaryPath)) {
    console.log(`Removing existing ${platform.name} binary to download fresh version...`);
    fs.unlinkSync(binaryPath);
  }
  
  try {
    // Download archive
    await downloadFile(downloadUrl, archivePath);
    
    // Extract binary
    console.log(`Extracting ${platform.name} binary...`);
    await extractArchive(archivePath, platformDir);
    
    // Clean up archive
    fs.unlinkSync(archivePath);
    
    // Make binary executable on Unix systems
    if (platform.binaryName !== 'lawkit.exe') {
      fs.chmodSync(binaryPath, '755');
    }
    
    console.log(`SUCCESS: ${platform.name} binary installed at ${binaryPath}`);
    
  } catch (error) {
    console.error(`ERROR: Failed to download ${platform.name} binary:`, error.message);
    throw error;
  }
}

async function main() {
  try {
    console.log(`Downloading lawkit v${LAWKIT_VERSION} binaries for all platforms...`);
    
    // Create main bin directory
    if (!fs.existsSync(BINARY_DIR)) {
      fs.mkdirSync(BINARY_DIR, { recursive: true });
    }
    
    // Download all platform binaries
    for (const platform of PLATFORMS) {
      await downloadPlatformBinary(platform);
    }
    
    console.log('\n🎉 All platform binaries downloaded successfully!');
    console.log('\nBinary structure:');
    console.log('bin/');
    for (const platform of PLATFORMS) {
      console.log(`  └── ${platform.subdir}/${platform.binaryName}`);
    }
    
  } catch (error) {
    console.error('ERROR: Failed to download platform binaries:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}