/**
 * TypeScript definitions for lawkit-js - UNIFIED API DESIGN
 * Statistical law analysis toolkit for fraud detection and data quality assessment
 */

export interface LawkitOptions {
  /** Statistical laws to check */
  lawsToCheck?: ('benford' | 'pareto' | 'zipf' | 'normal' | 'poisson')[];
  
  /** Analysis threshold value */
  analysisThreshold?: number;
  
  /** Confidence level for statistical tests (0.0-1.0, default: 0.95) */
  confidenceLevel?: number;
  
  /** Statistical tests to perform */
  statisticalTests?: ('chi_square' | 'kolmogorov_smirnov' | 'anderson_darling')[];
  
  /** Output format */
  outputFormat?: 'text' | 'json' | 'yaml';
  
  /** Include unchanged values in output (default: false) */
  showUnchanged?: boolean;
  
  /** Include type information in output (default: false) */
  showTypes?: boolean;
  
  /** Risk threshold level */
  riskThreshold?: 'low' | 'medium' | 'high';
  
  /** Significance level for hypothesis testing (default: 0.05) */
  significanceLevel?: number;
  
  /** Minimum sample size required for analysis (default: 30) */
  minSampleSize?: number;
  
  /** Enable outlier detection (default: false) */
  enableOutlierDetection?: boolean;
  
  /** Enable Japanese numeral support (default: false) */
  enableJapaneseNumerals?: boolean;
  
  /** Enable international numeral support (default: false) */
  enableInternationalNumerals?: boolean;
  
  /** Enable parallel processing (default: false) */
  enableParallelProcessing?: boolean;
  
  /** Memory limit in MB (default: 1000) */
  memoryLimitMb?: number;
  
  /** Tolerance for acceptable deviation */
  tolerance?: number;
  
  /** Filter to apply before analysis */
  filter?: string;
  
  /** Focus analysis on specific aspect */
  focus?: string;
  
  /** Include detailed metadata in results (default: false) */
  includeMetadata?: boolean;
  
  /** Generate comprehensive analysis report (default: false) */
  detailedReport?: boolean;
}

export type LawkitResultType = 
  | 'BenfordAnalysis'
  | 'ParetoAnalysis' 
  | 'ZipfAnalysis'
  | 'NormalAnalysis'
  | 'PoissonAnalysis'
  | 'IntegrationAnalysis'
  | 'ValidationResult'
  | 'DiagnosticResult'
  | 'GeneratedData';

export interface BenfordData {
  observedDistribution: Record<string, number>;
  expectedDistribution: Record<string, number>;
  chiSquare: number;
  pValue: number;
  mad: number;
  riskLevel: string;
  totalNumbers: number;
  analysisSummary: string;
}

export interface ParetoData {
  top20PercentContribution: number;
  paretoRatio: number;
  concentrationIndex: number;
  riskLevel: string;
  totalItems: number;
  analysisSummary: string;
}

export interface ZipfData {
  zipfCoefficient: number;
  correlationCoefficient: number;
  deviationScore: number;
  riskLevel: string;
  totalItems: number;
  analysisSummary: string;
}

export interface NormalData {
  mean: number;
  stdDev: number;
  skewness: number;
  kurtosis: number;
  normalityTestP: number;
  riskLevel: string;
  totalNumbers: number;
  analysisSummary: string;
}

export interface PoissonData {
  lambda: number;
  varianceRatio: number;
  poissonTestP: number;
  riskLevel: string;
  totalEvents: number;
  analysisSummary: string;
}

export interface IntegrationData {
  lawsAnalyzed: string[];
  overallRisk: string;
  conflictingResults: string[];
  recommendations: string[];
  analysisSummary: string;
}

export interface ValidationData {
  validationPassed: boolean;
  issuesFound: string[];
  dataQualityScore: number;
  analysisSummary: string;
}

export interface DiagnosticData {
  diagnosticType: string;
  findings: Record<string, any>;
  confidenceLevel: number;
  analysisSummary: string;
}

export interface GeneratedData {
  dataType: string;
  count: number;
  parameters: Record<string, any>;
  sampleData: any[];
}

export interface LawkitResult {
  /** Type of result */
  type: LawkitResultType;
  
  /** Path or identifier */
  path: string;
  
  /** Analysis data (varies by result type) */
  data?: BenfordData | ParetoData | ZipfData | NormalData | PoissonData | 
         IntegrationData | ValidationData | DiagnosticData | GeneratedData;
  
  /** Raw analysis data for flexible access */
  [key: string]: any;
}

export type LawkitSubcommand = 
  | 'benford'    // Benford's Law analysis
  | 'pareto'     // Pareto distribution analysis  
  | 'zipf'       // Zipf's Law analysis
  | 'normal'     // Normal distribution analysis
  | 'poisson'    // Poisson distribution analysis
  | 'analyze'    // Automatic multi-law analysis
  | 'validate'   // Data validation
  | 'diagnose'   // Diagnostic information
  | 'generate'   // Test data generation
  | 'list'       // List available laws
  | 'selftest';  // Self-test

/**
 * Analyze data using statistical laws or perform utility operations
 * 
 * This is the unified entry point for all lawkit functionality.
 * Users should prepare data themselves and call this function with a subcommand.
 * 
 * @param subcommand - Analysis command or utility operation
 * @param dataOrConfig - Data to analyze or configuration object
 * @param options - Optional configuration
 * @returns Promise resolving to array of analysis results
 * 
 * @example
 * ```typescript
 * import { law, LawkitOptions } from 'lawkit-js';
 * import * as fs from 'fs';
 * 
 * // Benford's Law analysis for fraud detection
 * const financialData = [123, 456, 789, 234, 567, 890, 345, 678, 901];
 * const options: LawkitOptions = {
 *   lawsToCheck: ['benford'],
 *   confidenceLevel: 0.99,
 *   detailedReport: true
 * };
 * 
 * const benfordResults = await law('benford', financialData, options);
 * console.log('Fraud analysis results:', benfordResults);
 * 
 * // Multi-law analysis for data quality assessment
 * const dataset = JSON.parse(fs.readFileSync('dataset.json', 'utf8'));
 * const qualityOptions: LawkitOptions = {
 *   lawsToCheck: ['benford', 'pareto', 'normal'],
 *   outputFormat: 'json',
 *   enableOutlierDetection: true,
 *   includeMetadata: true
 * };
 * 
 * const analysisResults = await law('analyze', dataset, qualityOptions);
 * console.log('Data quality assessment:', analysisResults);
 * 
 * // Generate test data following Benford's Law
 * const generationConfig = { 
 *   law: 'benford', 
 *   count: 1000,
 *   seed: 42
 * };
 * const generatedResults = await law('generate', generationConfig);
 * console.log('Generated test data:', generatedResults);
 * 
 * // Data validation
 * const testData = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 * const validationResults = await law('validate', testData, {
 *   statisticalTests: ['normality', 'randomness']
 * });
 * console.log('Validation results:', validationResults);
 * ```
 */
export function law(
  subcommand: LawkitSubcommand, 
  dataOrConfig: any, 
  options?: LawkitOptions
): Promise<LawkitResult[]>;

export default law;