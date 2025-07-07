#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the platform-specific binary name
let binaryName = 'lawkit';
if (process.platform === 'win32') {
  binaryName = 'lawkit.exe';
}

// Construct the path to the binary
// The binary is downloaded and placed in the 'bin' directory
const binaryPath = path.join(__dirname, 'bin', binaryName);

// Check if the binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`Error: Binary not found at ${binaryPath}`);
  console.error('Please ensure lawkit is properly installed or built for your platform.');
  console.error('Try running: npm install --force');
  process.exit(1);
}

// Function to run lawkit programmatically (for use as a library)
function runLawkit(args) {
  return new Promise((resolve, reject) => {
    const child = spawn(binaryPath, args, {
      stdio: 'pipe',
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    child.on('close', (code) => {
      resolve({
        code: code,
        stdout: stdout,
        stderr: stderr
      });
    });

    child.on('error', (err) => {
      reject(err);
    });
  });
}

// If called directly (not as a library), spawn the process
if (require.main === module) {
  const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
  });

  child.on('close', (code) => {
    process.exit(code);
  });

  child.on('error', (err) => {
    console.error(`Failed to start lawkit: ${err.message}`);
    process.exit(1);
  });
}

// Export the function for programmatic use
module.exports = { runLawkit };