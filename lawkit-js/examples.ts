#!/usr/bin/env tsx

/**
 * lawkit-js TypeScript Examples - UNIFIED API DESIGN
 * 
 * Demonstrates native NAPI-RS API usage for statistical law analysis
 * Users prepare data themselves and call the unified law() function
 */

import { law, LawkitOptions, LawkitResult, LawkitSubcommand } from './index';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

// Colors for console output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m',
    magenta: '\x1b[35m',
    reset: '\x1b[0m'
} as const;

function log(message: string, color: keyof typeof colors = 'reset'): void {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function header(message: string): void {
    log(`\n${message}`, 'cyan');
    log('='.repeat(message.length), 'cyan');
}

function example(title: string, description: string): void {
    log(`\n${title}`, 'yellow');
    log(`   ${description}`, 'blue');
}

// Mock data generators for examples
function generateBenfordData(count: number): number[] {
    // Generate data that should follow Benford's Law
    const data: number[] = [];
    for (let i = 0; i < count; i++) {
        const magnitude = Math.floor(Math.random() * 6) + 1; // 1-6 digits
        const firstDigit = Math.floor(Math.log10(Math.random()) * -1) + 1;
        const value = firstDigit * Math.pow(10, magnitude - 1) + Math.floor(Math.random() * Math.pow(10, magnitude - 1));
        data.push(value);
    }
    return data;
}

function generateParetoData(): number[] {
    // Generate data that follows 80/20 rule
    const data: number[] = [];
    // 20% of items contribute 80% of value
    for (let i = 0; i < 20; i++) data.push(Math.random() * 800 + 200); // High values
    for (let i = 0; i < 80; i++) data.push(Math.random() * 200); // Low values
    return data.sort((a, b) => b - a);
}

function generateSuspiciousData(): number[] {
    // Generate data that might indicate fraud (non-Benford distribution)
    const data: number[] = [];
    for (let i = 0; i < 1000; i++) {
        // Artificially skew toward certain first digits
        const suspicious = Math.random() < 0.7 ? [5, 6, 7, 8, 9] : [1, 2, 3, 4];
        const firstDigit = suspicious[Math.floor(Math.random() * suspicious.length)];
        const magnitude = Math.floor(Math.random() * 4) + 2; // 2-5 digits
        const value = firstDigit * Math.pow(10, magnitude - 1) + Math.floor(Math.random() * Math.pow(10, magnitude - 1));
        data.push(value);
    }
    return data;
}

async function runExamples(): Promise<void> {
    header('lawkit-js Native API Examples');
    
    // Create temporary directory
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'lawkit-examples-'));
    const oldCwd = process.cwd();
    process.chdir(tempDir);

    try {
        // Example 1: Fraud Detection with Benford's Law
        header('1. Financial Fraud Detection');
        
        const legitimateTransactions = generateBenfordData(2000);
        const suspiciousTransactions = generateSuspiciousData();

        example(
            'Benford\'s Law Analysis for Fraud Detection',
            'Compare legitimate vs suspicious financial transaction patterns'
        );

        const fraudOptions: LawkitOptions = {
            confidenceLevel: 0.99,
            detailedReport: true,
            outputFormat: 'json',
            enableOutlierDetection: true
        };

        log('Analyzing legitimate transactions...', 'blue');
        const legitimateResults = law('benford', legitimateTransactions, fraudOptions);
        
        log('Analyzing suspicious transactions...', 'blue');
        const suspiciousResults = law('benford', suspiciousTransactions, fraudOptions);

        // Process results
        legitimateResults.forEach((result: LawkitResult) => {
            if (result.type === 'BenfordAnalysis') {
                log(`✅ Legitimate data risk level: ${result.riskLevel}`, 'green');
                log(`   Chi-square: ${result.chiSquare?.toFixed(4)}`, 'blue');
                log(`   P-value: ${result.pValue?.toFixed(6)}`, 'blue');
            }
        });

        suspiciousResults.forEach((result: LawkitResult) => {
            if (result.type === 'BenfordAnalysis') {
                log(`⚠️  Suspicious data risk level: ${result.riskLevel}`, 'red');
                log(`   Chi-square: ${result.chiSquare?.toFixed(4)}`, 'blue');
                log(`   P-value: ${result.pValue?.toFixed(6)}`, 'blue');
            }
        });

        // Example 2: Business Data Quality Assessment
        header('2. Business Data Quality Assessment');

        const salesData = generateParetoData();
        const customerData = Array.from({ length: 1000 }, () => Math.random() * 100);

        example(
            'Multi-Law Analysis for Data Quality',
            'Assess data quality using multiple statistical laws'
        );

        const qualityOptions: LawkitOptions = {
            lawsToCheck: ['benford', 'pareto', 'normal'],
            includeMetadata: true,
            statisticalTests: ['chi_square', 'kolmogorov_smirnov'],
            outputFormat: 'json'
        };

        const salesAnalysis = law('analyze', { sales: salesData }, qualityOptions);
        const customerAnalysis = law('analyze', { customers: customerData }, qualityOptions);

        log('Sales Data Analysis:', 'green');
        salesAnalysis.forEach((result: LawkitResult) => {
            console.log(`  ${result.type}: ${result.path} - Risk: ${result.overallRisk || 'N/A'}`);
        });

        log('Customer Data Analysis:', 'green');
        customerAnalysis.forEach((result: LawkitResult) => {
            console.log(`  ${result.type}: ${result.path} - Risk: ${result.overallRisk || 'N/A'}`);
        });

        // Example 3: Audit Compliance Testing
        header('3. Audit Compliance Testing');

        const expenseData = [
            234.56, 1234.78, 567.89, 2345.67, 789.12,
            3456.78, 890.23, 4567.89, 123.45, 5678.90,
            345.67, 6789.01, 456.78, 7890.12, 678.90
        ];

        example(
            'Expense Report Compliance Check',
            'Validate expense reports for compliance with expected patterns'
        );

        const auditOptions: LawkitOptions = {
            confidenceLevel: 0.95,
            significance: 0.05,
            minSampleSize: 10,
            riskThreshold: 'medium',
            detailedReport: true
        };

        const auditResults = law('validate', expenseData, auditOptions);
        
        auditResults.forEach((result: LawkitResult) => {
            if (result.type === 'ValidationResult') {
                log(`Audit Result: ${result.validationPassed ? 'PASSED' : 'FAILED'}`, 
                    result.validationPassed ? 'green' : 'red');
                log(`Data Quality Score: ${result.dataQualityScore}`, 'blue');
                if (result.issuesFound && result.issuesFound.length > 0) {
                    log('Issues found:', 'yellow');
                    result.issuesFound.forEach(issue => log(`  - ${issue}`, 'red'));
                }
            }
        });

        // Example 4: Real-time Data Monitoring
        header('4. Real-time Data Stream Monitoring');

        example(
            'Streaming Data Analysis',
            'Monitor incoming data streams for anomalies in real-time'
        );

        // Simulate streaming data
        const streamingOptions: LawkitOptions = {
            enableParallelProcessing: true,
            memoryLimitMb: 100,
            confidenceLevel: 0.98
        };

        for (let batch = 1; batch <= 3; batch++) {
            const batchData = generateBenfordData(500);
            log(`Processing batch ${batch}...`, 'yellow');
            
            const streamResults = law('benford', batchData, streamingOptions);
            
            streamResults.forEach((result: LawkitResult) => {
                if (result.type === 'BenfordAnalysis') {
                    const status = result.riskLevel === 'low' ? '✅' : result.riskLevel === 'medium' ? '⚠️' : '❌';
                    log(`${status} Batch ${batch}: Risk ${result.riskLevel} (p=${result.pValue?.toFixed(4)})`, 'blue');
                }
            });
        }

        // Example 5: International Data Support
        header('5. International Data Support');

        const japaneseNumbers = ['１２３', '２３４', '３４５', '４５６', '５６７', '６７８', '７８９', '８９０', '９０１'];
        const mixedData = [123, 234, 345, '456', '567.89', '€678.90'];

        example(
            'Multi-format Number Analysis',
            'Handle international number formats and mixed data types'
        );

        const internationalOptions: LawkitOptions = {
            enableJapaneseNumerals: true,
            enableInternationalNumerals: true,
            outputFormat: 'json'
        };

        try {
            const intlResults = law('benford', mixedData, internationalOptions);
            log('International data processed successfully', 'green');
            intlResults.forEach((result: LawkitResult) => {
                console.log(`  ${result.type}: ${result.analysisSummary || 'Analysis completed'}`);
            });
        } catch (error) {
            log('International data processing note: Feature may require additional configuration', 'yellow');
        }

        // Example 6: Test Data Generation
        header('6. Test Data Generation');

        example(
            'Generate Test Data Following Statistical Laws',
            'Create synthetic datasets for testing and validation'
        );

        const generationConfigs = [
            { law: 'benford', count: 1000, seed: 42 },
            { law: 'pareto', count: 500, parameters: { alpha: 1.16 } },
            { law: 'zipf', count: 100, parameters: { s: 1.0 } }
        ];

        for (const config of generationConfigs) {
            const generated = law('generate', config);
            
            generated.forEach((result: LawkitResult) => {
                if (result.type === 'GeneratedData') {
                    log(`Generated ${result.count} ${result.dataType} data points`, 'green');
                    if (result.sampleData && result.sampleData.length > 0) {
                        log(`Sample: [${result.sampleData.slice(0, 5).join(', ')}...]`, 'blue');
                    }
                }
            });
        }

        // Example 7: Error Handling and Diagnostics
        header('7. Error Handling and Diagnostics');

        example(
            'Robust Error Handling for Invalid Data',
            'Demonstrate graceful handling of edge cases and invalid inputs'
        );

        const edgeCases = [
            { data: [], description: 'Empty dataset' },
            { data: [1], description: 'Single value' },
            { data: [1, 1, 1, 1, 1], description: 'All identical values' },
            { data: ['invalid', 'data', 'types'], description: 'Non-numeric data' }
        ];

        for (const testCase of edgeCases) {
            try {
                log(`Testing: ${testCase.description}`, 'yellow');
                const diagnostics = law('diagnose', testCase.data);
                
                diagnostics.forEach((result: LawkitResult) => {
                    if (result.type === 'DiagnosticResult') {
                        log(`  Diagnostic: ${result.diagnosticType}`, 'blue');
                        log(`  Confidence: ${result.confidenceLevel}`, 'blue');
                    }
                });
            } catch (error) {
                log(`  Expected error handled: ${error}`, 'red');
            }
        }

        // Example 8: Performance Benchmarking
        header('8. Performance Benchmarking');

        example(
            'Large Dataset Performance Analysis',
            'Benchmark performance with large datasets and memory optimization'
        );

        const largeSizes = [10000, 50000, 100000];
        
        for (const size of largeSizes) {
            const largeDataset = generateBenfordData(size);
            
            const perfOptions: LawkitOptions = {
                enableParallelProcessing: true,
                memoryLimitMb: 500,
                confidenceLevel: 0.95
            };

            const startTime = Date.now();
            const perfResults = law('benford', largeDataset, perfOptions);
            const endTime = Date.now();
            
            const duration = endTime - startTime;
            log(`Dataset size: ${size.toLocaleString()} - Processing time: ${duration}ms`, 'green');
            
            perfResults.forEach((result: LawkitResult) => {
                if (result.type === 'BenfordAnalysis') {
                    log(`  Analysis completed: ${result.totalNumbers} numbers processed`, 'blue');
                }
            });
        }

        // Summary
        header('Summary');
        log('✅ All lawkit examples completed successfully!', 'green');
        log('\nStatistical Law Analysis Benefits:', 'cyan');
        log('  • Fraud detection and prevention', 'blue');
        log('  • Data quality assessment', 'blue');
        log('  • Audit compliance validation', 'blue');
        log('  • Real-time anomaly detection', 'blue');
        log('  • International data support', 'blue');
        log('  • Test data generation', 'blue');
        log('  • Performance optimization', 'blue');

        log('\nSupported Statistical Laws:', 'cyan');
        log('  • Benford\'s Law (fraud detection)', 'blue');
        log('  • Pareto Distribution (80/20 rule)', 'blue');
        log('  • Zipf\'s Law (natural language, populations)', 'blue');
        log('  • Normal Distribution (quality control)', 'blue');
        log('  • Poisson Distribution (event analysis)', 'blue');

        log('\nUse Cases:', 'cyan');
        log('  • Financial audit and compliance', 'blue');
        log('  • Business intelligence and analytics', 'blue');
        log('  • Scientific data validation', 'blue');
        log('  • Market research analysis', 'blue');
        log('  • Risk assessment and management', 'blue');

    } catch (error) {
        log(`\nError running examples: ${error}`, 'red');
        console.error(error);
    } finally {
        // Cleanup
        process.chdir(oldCwd);
        try {
            fs.rmSync(tempDir, { recursive: true, force: true });
        } catch (cleanupErr) {
            log(`Cleanup warning: ${cleanupErr}`, 'yellow');
        }
    }
}

// Run examples if called directly
if (require.main === module) {
    runExamples().catch(console.error);
}

export { runExamples };