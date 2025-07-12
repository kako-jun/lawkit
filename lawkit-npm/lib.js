/**
 * Node.js API wrapper for lawkit CLI tool
 * 
 * This module provides a JavaScript API for the lawkit CLI tool,
 * allowing you to analyze statistical laws programmatically.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const { writeFileSync, mkdtempSync, rmSync } = require('fs');
const { tmpdir } = require('os');

/**
 * @typedef {'benf'|'pareto'|'zipf'|'normal'|'poisson'|'analyze'|'validate'|'diagnose'|'generate'} Command
 * @typedef {'text'|'json'|'csv'|'yaml'|'toml'|'xml'} OutputFormat
 */

/**
 * Options for lawkit operations
 * @typedef {Object} LawkitOptions
 * @property {OutputFormat} [output='text'] - Output format
 * @property {number} [minCount] - Minimum count threshold
 * @property {number} [confidence] - Confidence level
 * @property {number} [sampleSize] - Sample size for optimization
 * @property {number} [minValue] - Minimum value threshold
 * @property {boolean} [quiet=false] - Suppress output
 * @property {boolean} [verbose=false] - Verbose output
 * @property {string} [outputFile] - Output file path
 */

/**
 * Result of a lawkit analysis
 * @typedef {Object} AnalysisResult
 * @property {string} law - Statistical law analyzed
 * @property {number} pValue - P-value of the test
 * @property {number} statistic - Test statistic
 * @property {boolean} significant - Whether result is statistically significant
 * @property {string} conclusion - Conclusion of the analysis
 */

/**
 * Error thrown when lawkit command fails
 */
class LawkitError extends Error {
  constructor(message, exitCode, stderr) {
    super(message);
    this.name = 'LawkitError';
    this.exitCode = exitCode;
    this.stderr = stderr;
  }
}

/**
 * Get the path to the lawkit binary
 * @returns {string} Path to lawkit binary
 */
function getLawkitBinaryPath() {
  // Check if local binary exists (installed via postinstall)
  const binaryName = process.platform === 'win32' ? 'lawkit.exe' : 'lawkit';
  const localBinaryPath = path.join(__dirname, 'bin', binaryName);
  
  if (fs.existsSync(localBinaryPath)) {
    return localBinaryPath;
  }
  
  // Fall back to system PATH
  return 'lawkit';
}

/**
 * Execute lawkit command
 * @param {string[]} args - Command arguments
 * @returns {Promise<{stdout: string, stderr: string}>} Command output
 */
function executeLawkit(args) {
  return new Promise((resolve, reject) => {
    const lawkitPath = getLawkitBinaryPath();
    
    const child = spawn(lawkitPath, args, {
      stdio: ['pipe', 'pipe', 'pipe']
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
      if (code === 0) {
        resolve({ stdout, stderr });
      } else {
        reject(new LawkitError(
          `lawkit exited with code ${code}`,
          code,
          stderr
        ));
      }
    });
    
    child.on('error', (err) => {
      if (err.code === 'ENOENT') {
        reject(new LawkitError(
          'lawkit command not found. Please install lawkit CLI tool.',
          -1,
          ''
        ));
      } else {
        reject(new LawkitError(err.message, -1, ''));
      }
    });
  });
}

/**
 * Analyze data using Benford's Law
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 * 
 * @example
 * // Analyze file
 * const result = await benford('data.csv');
 * console.log(result);
 * 
 * @example
 * // Analyze array with JSON output
 * const numbers = [1, 10, 100, 1000, 2000];
 * const result = await benford(numbers, { output: 'json' });
 * console.log(result);
 */
async function benford(data, options = {}) {
  return executeAnalysis('benf', data, options);
}

/**
 * Analyze data using Pareto Principle
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 */
async function pareto(data, options = {}) {
  return executeAnalysis('pareto', data, options);
}

/**
 * Analyze data using Zipf's Law
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 */
async function zipf(data, options = {}) {
  return executeAnalysis('zipf', data, options);
}

/**
 * Analyze data using Normal Distribution
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 */
async function normal(data, options = {}) {
  return executeAnalysis('normal', data, options);
}

/**
 * Analyze data using Poisson Distribution
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 */
async function poisson(data, options = {}) {
  return executeAnalysis('poisson', data, options);
}

/**
 * Common function to execute analysis
 * @private
 */
async function executeAnalysis(command, data, options = {}) {
  let inputPath = data;
  let tempFile = null;
  
  // If data is an array, write to temporary file
  if (Array.isArray(data)) {
    const tmpDir = mkdtempSync(path.join(tmpdir(), 'lawkit-'));
    tempFile = path.join(tmpDir, 'data.txt');
    writeFileSync(tempFile, data.join('\n'), 'utf8');
    inputPath = tempFile;
  }
  
  try {
    const args = [command];
    
    // Add output format option
    if (options.output) {
      args.push('--output', options.output);
    }
    
    // Add min-count option
    if (options.minCount !== undefined) {
      args.push('--min-count', options.minCount.toString());
    }
    
    // Add confidence option
    if (options.confidence !== undefined) {
      args.push('--confidence', options.confidence.toString());
    }
    
    // Add sample-size option
    if (options.sampleSize !== undefined) {
      args.push('--sample-size', options.sampleSize.toString());
    }
    
    // Add min-value option
    if (options.minValue !== undefined) {
      args.push('--min-value', options.minValue.toString());
    }
    
    // Add quiet option
    if (options.quiet) {
      args.push('--quiet');
    }
    
    // Add verbose option
    if (options.verbose) {
      args.push('--verbose');
    }
    
    // Add output file option
    if (options.outputFile) {
      args.push('--output-file', options.outputFile);
    }
    
    // Add input file
    args.push(inputPath);
    
    const { stdout, stderr } = await executeLawkit(args);
    
    // If output format is JSON, parse the result
    if (options.output === 'json') {
      try {
        return JSON.parse(stdout);
      } catch (e) {
        throw new LawkitError(`Failed to parse JSON output: ${e.message}`, -1, '');
      }
    }
    
    // Return raw output for other formats
    return stdout;
  } finally {
    // Clean up temporary file if created
    if (tempFile) {
      rmSync(path.dirname(tempFile), { recursive: true, force: true });
    }
  }
}

/**
 * Check if lawkit command is available in the system
 * 
 * @returns {Promise<boolean>} True if lawkit is available, false otherwise
 * 
 * @example
 * if (!(await isLawkitAvailable())) {
 *   console.error('Please install lawkit CLI tool');
 *   process.exit(1);
 * }
 */
async function isLawkitAvailable() {
  try {
    await executeLawkit(['--version']);
    return true;
  } catch (err) {
    return false;
  }
}

module.exports = {
  benford,
  pareto,
  zipf,
  normal,
  poisson,
  isLawkitAvailable,
  LawkitError
};