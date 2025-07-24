/**
 * Node.js native bindings for lawkit - UNIFIED API DESIGN
 * 
 * This module provides a JavaScript API for the lawkit library using native NAPI-RS bindings.
 * It follows the unified API design principle: only the main law() function is exposed.
 */

const { lawJs } = require('./index.js');

/**
 * @typedef {Object} LawkitOptions
 * @property {Array<string>} [lawsToCheck] - Statistical laws to check (['benford', 'pareto', 'zipf', 'normal', 'poisson'])
 * @property {number} [analysisThreshold] - Threshold for analysis
 * @property {number} [confidenceLevel] - Confidence level for statistical tests
 * @property {Array<string>} [statisticalTests] - Statistical tests to perform
 * @property {string} [outputFormat] - Output format ('text', 'json', 'yaml')
 * @property {boolean} [showUnchanged] - Include unchanged values in output
 * @property {boolean} [showTypes] - Include type information in output
 */

/**
 * @typedef {Object} LawkitResult
 * @property {string} type - Type of result
 * @property {string} [lawName] - Name of the statistical law (for 'analysis', 'violation')
 * @property {*} [data] - Analysis data (for 'analysis', 'generatedData')
 * @property {number} [severity] - Violation severity (for 'violation')
 * @property {string} [details] - Details about violation (for 'violation')
 * @property {string} [category] - Insight category (for 'insight')
 * @property {string} [insight] - Insight text (for 'insight')
 * @property {string} [message] - Error message (for 'error')
 * @property {string} [dataType] - Type of generated data (for 'generatedData')
 * 
 * Result types:
 * - 'analysis' - Statistical law analysis result
 * - 'violation' - Law violation detected
 * - 'insight' - Statistical insight
 * - 'error' - Analysis error
 * - 'generatedData' - Generated test data
 */

/**
 * Analyze data using statistical laws
 * 
 * This is the unified entry point for all lawkit functionality.
 * Users should prepare data themselves and call this function with a subcommand.
 * 
 * @param {string} subcommand - Analysis command ('benford', 'pareto', 'zipf', 'normal', 'poisson', 'analyze', 'validate', 'diagnose', 'generate')
 * @param {*} dataOrConfig - Data to analyze or configuration object
 * @param {LawkitOptions} [options] - Optional configuration
 * @returns {Promise<LawkitResult[]>} Array of analysis results
 * 
 * @example
 * const lawkit = require('lawkit-js');
 * const fs = require('fs');
 * 
 * // Benford's Law analysis
 * const financialData = [123, 456, 789, 234, 567, 890, 345, 678, 901];
 * const benfordResults = await lawkit.law('benford', financialData, {
 *   lawsToCheck: ['benford'],
 *   confidenceLevel: 0.95
 * });
 * 
 * // Pareto analysis
 * const salesData = [100, 200, 50, 300, 75, 150, 400, 25, 175, 80];
 * const paretoResults = await lawkit.law('pareto', salesData, {
 *   analysisThreshold: 0.8
 * });
 * 
 * // Multi-law analysis with JSON data
 * const jsonData = JSON.parse(fs.readFileSync('dataset.json', 'utf8'));
 * const analysisResults = await lawkit.law('analyze', jsonData, {
 *   lawsToCheck: ['benford', 'pareto', 'zipf'],
 *   outputFormat: 'json'
 * });
 * 
 * // Data validation
 * const testData = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 * const validationResults = await lawkit.law('validate', testData, {
 *   statisticalTests: ['normality', 'randomness']
 * });
 * 
 * // Generate test data
 * const generationConfig = { law: 'benford', count: 1000 };
 * const generatedResults = await lawkit.law('generate', generationConfig);
 */
async function law(subcommand, dataOrConfig, options = {}) {
    try {
        return lawJs(subcommand, dataOrConfig, options);
    } catch (error) {
        throw new Error(`Lawkit operation failed: ${error.message}`);
    }
}

module.exports = {
    law
};

// For compatibility with CommonJS and ES modules
module.exports.default = module.exports;