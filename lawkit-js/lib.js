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
 * @typedef {'benf'|'pareto'|'zipf'|'normal'|'poisson'|'analyze'|'validate'|'diagnose'|'generate'|'list'} Command
 * @typedef {'text'|'json'|'csv'|'yaml'|'toml'|'xml'} OutputFormat
 */

/**
 * Options for lawkit operations
 * @typedef {Object} LawkitOptions
 * @property {OutputFormat} [output='text'] - Output format (maps to --format option)
 * @property {boolean} [quiet=false] - Suppress output
 * @property {boolean} [verbose=false] - Verbose output
 * @property {string} [filter] - Filter numbers by range
 * @property {number} [minCount] - Minimum count threshold
 * 
 * // Integration options
 * @property {string} [laws] - Laws to analyze (comma-separated)
 * @property {string} [focus] - Analysis focus area
 * @property {number} [threshold] - Analysis threshold for anomaly detection
 * @property {boolean} [recommend=false] - Enable recommendation mode
 * @property {string} [report] - Analysis report type
 * @property {boolean} [consistencyCheck=false] - Enable consistency check
 * @property {boolean} [crossValidation=false] - Enable cross-validation
 * @property {number} [confidenceLevel] - Confidence level
 * @property {string} [purpose] - Analysis purpose
 * 
 * // Benford-specific options
 * @property {string} [thresholdLevel] - Anomaly detection threshold level
 * @property {number} [confidence] - Statistical confidence level
 * @property {number} [sampleSize] - Maximum sample size for large datasets
 * @property {number} [minValue] - Minimum value to include in analysis
 * 
 * // Pareto-specific options
 * @property {number} [concentration] - Concentration threshold
 * @property {boolean} [giniCoefficient=false] - Calculate Gini coefficient
 * @property {string} [percentiles] - Custom percentiles to calculate
 * @property {boolean} [businessAnalysis=false] - Enable business analysis
 * 
 * // Zipf-specific options
 * @property {boolean} [text=false] - Enable text analysis mode
 * @property {number} [words] - Maximum number of words to analyze
 * @property {number} [vocabularySize] - Vocabulary size for text generation
 * @property {number} [exponent] - Zipf exponent
 * 
 * // Normal distribution options
 * @property {string} [test] - Normality test method
 * @property {boolean} [outliers=false] - Enable outlier detection
 * @property {string} [outlierMethod] - Outlier detection method
 * @property {boolean} [qualityControl=false] - Enable quality control analysis
 * @property {string} [specLimits] - Specification limits for quality control
 * @property {boolean} [enableTimeseries=false] - Enable time series analysis
 * @property {number} [timeseriesWindow] - Time series analysis window size
 * @property {number} [mean] - Mean of normal distribution
 * @property {number} [stddev] - Standard deviation of normal distribution
 * 
 * // Poisson distribution options
 * @property {boolean} [predict=false] - Enable probability prediction
 * @property {number} [maxEvents] - Maximum number of events for analysis
 * @property {boolean} [rareEvents=false] - Focus on rare event analysis
 * @property {number} [lambda] - Lambda parameter for Poisson distribution
 * @property {boolean} [timeSeries=false] - Generate time-series event data
 * 
 * // Generation options
 * @property {number} [samples] - Number of samples to generate
 * @property {number} [seed] - Random seed for reproducible generation
 * @property {string} [outputFile] - Output file path
 * @property {number} [fraudRate] - Fraud injection rate for testing
 * @property {string} [range] - Number range for generation
 * @property {number} [scale] - Scale parameter for distributions
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
 * Determine the platform-specific binary name and directory
 * @returns {Object} Platform info with subdir and binaryName
 */
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;
  
  if (platform === 'win32') {
    return { subdir: 'win32-x64', binaryName: 'lawkit.exe' };
  } else if (platform === 'darwin') {
    if (arch === 'arm64') {
      return { subdir: 'darwin-arm64', binaryName: 'lawkit' };
    } else {
      return { subdir: 'darwin-x64', binaryName: 'lawkit' };
    }
  } else if (platform === 'linux') {
    return { subdir: 'linux-x64', binaryName: 'lawkit' };
  } else {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
}

/**
 * Get the path to the lawkit binary
 * @returns {string} Path to lawkit binary
 */
function getLawkitBinaryPath() {
  // Get platform-specific binary path
  const platformInfo = getPlatformInfo();
  const platformBinaryPath = path.join(__dirname, 'bin', platformInfo.subdir, platformInfo.binaryName);
  
  if (fs.existsSync(platformBinaryPath)) {
    return platformBinaryPath;
  }
  
  // Error if binary not found - no system PATH fallback allowed
  throw new Error(`Binary not found at ${platformBinaryPath}. Platform: ${process.platform}-${process.arch}. This might indicate a packaging issue. Please report this at: https://github.com/kako-jun/lawkit/issues`);
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
    
    // Common options
    if (options.output) args.push('--format', options.output);
    if (options.quiet) args.push('--quiet');
    if (options.verbose) args.push('--verbose');
    if (options.filter) args.push('--filter', options.filter);
    if (options.minCount !== undefined) args.push('--min-count', options.minCount.toString());
    
    // Integration options
    if (options.laws) args.push('--laws', options.laws);
    if (options.focus) args.push('--focus', options.focus);
    if (options.threshold !== undefined) args.push('--threshold', options.threshold.toString());
    if (options.recommend) args.push('--recommend');
    if (options.report) args.push('--report', options.report);
    if (options.consistencyCheck) args.push('--consistency-check');
    if (options.crossValidation) args.push('--cross-validation');
    if (options.confidenceLevel !== undefined) args.push('--confidence-level', options.confidenceLevel.toString());
    if (options.purpose) args.push('--purpose', options.purpose);
    
    // Benford-specific options
    if (options.thresholdLevel) args.push('--threshold', options.thresholdLevel);
    if (options.confidence !== undefined) args.push('--confidence', options.confidence.toString());
    if (options.sampleSize !== undefined) args.push('--sample-size', options.sampleSize.toString());
    if (options.minValue !== undefined) args.push('--min-value', options.minValue.toString());
    
    // Pareto-specific options
    if (options.concentration !== undefined) args.push('--concentration', options.concentration.toString());
    if (options.giniCoefficient) args.push('--gini-coefficient');
    if (options.percentiles) args.push('--percentiles', options.percentiles);
    if (options.businessAnalysis) args.push('--business-analysis');
    
    // Zipf-specific options
    if (options.text) args.push('--text');
    if (options.words !== undefined) args.push('--words', options.words.toString());
    if (options.vocabularySize !== undefined) args.push('--vocabulary-size', options.vocabularySize.toString());
    if (options.exponent !== undefined) args.push('--exponent', options.exponent.toString());
    
    // Normal distribution options
    if (options.test) args.push('--test', options.test);
    if (options.outliers) args.push('--outliers');
    if (options.outlierMethod) args.push('--outlier-method', options.outlierMethod);
    if (options.qualityControl) args.push('--quality-control');
    if (options.specLimits) args.push('--spec-limits', options.specLimits);
    if (options.enableTimeseries) args.push('--enable-timeseries');
    if (options.timeseriesWindow !== undefined) args.push('--timeseries-window', options.timeseriesWindow.toString());
    if (options.mean !== undefined) args.push('--mean', options.mean.toString());
    if (options.stddev !== undefined) args.push('--stddev', options.stddev.toString());
    
    // Poisson distribution options
    if (options.predict) args.push('--predict');
    if (options.maxEvents !== undefined) args.push('--max-events', options.maxEvents.toString());
    if (options.rareEvents) args.push('--rare-events');
    if (options.lambda !== undefined) args.push('--lambda', options.lambda.toString());
    if (options.timeSeries) args.push('--time-series');
    
    // Generation options
    if (options.samples !== undefined) args.push('--samples', options.samples.toString());
    if (options.seed !== undefined) args.push('--seed', options.seed.toString());
    if (options.outputFile) args.push('--output-file', options.outputFile);
    if (options.fraudRate !== undefined) args.push('--fraud-rate', options.fraudRate.toString());
    if (options.range) args.push('--range', options.range);
    if (options.scale !== undefined) args.push('--scale', options.scale.toString());
    
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

/**
 * Perform comprehensive analysis using multiple statistical laws
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Analysis result
 */
async function analyze(data, options = {}) {
  return executeAnalysis('analyze', data, options);
}

/**
 * Validate data quality using statistical tests
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Validation result
 */
async function validate(data, options = {}) {
  return executeAnalysis('validate', data, options);
}

/**
 * Diagnose data anomalies and provide recommendations
 * 
 * @param {string|number[]} data - Data file path or array of numbers
 * @param {LawkitOptions} [options={}] - Analysis options
 * @returns {Promise<string|AnalysisResult>} Diagnostic result
 */
async function diagnose(data, options = {}) {
  return executeAnalysis('diagnose', data, options);
}

/**
 * Generate sample data for testing statistical laws
 * 
 * @param {string} law - Statistical law to generate data for ('benf', 'pareto', 'zipf', 'normal', 'poisson')
 * @param {Object} [options={}] - Generation options
 * @param {number} [options.count=1000] - Number of samples to generate
 * @param {string} [options.outputFile] - Output file path
 * @returns {Promise<string>} Generated data or file path
 */
async function generate(law, options = {}) {
  const args = ['generate', law];
  
  // Add generation options
  if (options.count !== undefined) {
    args.push('--count', options.count.toString());
  }
  
  if (options.outputFile) {
    args.push('--output-file', options.outputFile);
  }
  
  // Add other common options
  if (options.output) {
    args.push('--format', options.output);
  }
  
  if (options.quiet) {
    args.push('--quiet');
  }
  
  const { stdout, stderr } = await executeLawkit(args);
  
  // If output format is JSON, parse the result
  if (options.output === 'json') {
    try {
      return JSON.parse(stdout);
    } catch (e) {
      throw new LawkitError(`Failed to parse JSON output: ${e.message}`, -1, '');
    }
  }
  
  return stdout;
}

/**
 * List available statistical laws and commands
 * 
 * @param {Object} [options={}] - List options
 * @returns {Promise<string>} List of available commands
 */
async function list(options = {}) {
  const args = ['list'];
  
  if (options.output) {
    args.push('--format', options.output);
  }
  
  const { stdout, stderr } = await executeLawkit(args);
  
  // If output format is JSON, parse the result
  if (options.output === 'json') {
    try {
      return JSON.parse(stdout);
    } catch (e) {
      throw new LawkitError(`Failed to parse JSON output: ${e.message}`, -1, '');
    }
  }
  
  return stdout;
}

module.exports = {
  benford,
  pareto,
  zipf,
  normal,
  poisson,
  analyze,
  validate,
  diagnose,
  generate,
  list,
  isLawkitAvailable,
  LawkitError
};