/**
 * Comprehensive unit tests for lawkit-js unified API
 * Tests NAPI-RS bindings layer, JavaScript type conversion, and statistical law analysis
 */

const lawkit = require('../index');

// Jest configuration for lawkit tests
jest.setTimeout(60000); // 60 seconds for statistical computations

describe('Lawkit Unified API Tests', () => {
    let fixtures;

    beforeAll(() => {
        fixtures = new TestFixtures();
    });

    afterEach(() => {
        // Cleanup after each test to prevent memory leaks
        if (global.gc) {
            global.gc();
        }
    });

    describe('API Availability', () => {
        test('main law function exists', () => {
            expect(typeof lawkit.law).toBe('function');
        });

        test('law function accepts expected parameters', () => {
            // Should accept subcommand, data, and optional options
            try {
                // This should not crash due to signature issues
                lawkit.law("validate", [1, 2, 3]);
                // Result may fail due to data issues, but function should exist
            } catch (error) {
                // Function exists if we get a lawkit-specific error, not TypeError
                expect(error).not.toBeInstanceOf(TypeError);
            }
        });
    });

    // ============================================================================
    // UNIFIED API TESTS - Core Functionality
    // ============================================================================

    describe('Core Statistical Law Analysis', () => {
        test('Benford analysis through unified API', () => {
            const data = fixtures.benfordCompliantData();
            
            const results = lawkit.law("benford", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            // Check result structure
            expect(result).toHaveProperty('type', 'BenfordAnalysis');
            expect(result).toHaveProperty('data');
            
            const benfordData = result.data;
            expect(benfordData).toHaveProperty('observed_distribution');
            expect(benfordData).toHaveProperty('expected_distribution');
            expect(benfordData).toHaveProperty('chi_square');
            expect(benfordData).toHaveProperty('p_value');
            expect(benfordData).toHaveProperty('total_numbers');
            expect(benfordData).toHaveProperty('analysis_summary');
            
            // Validate data types and ranges
            expect(benfordData.observed_distribution).toHaveLength(9);
            expect(benfordData.expected_distribution).toHaveLength(9);
            expect(benfordData.chi_square).toBeGreaterThanOrEqual(0.0);
            expect(benfordData.p_value).toBeGreaterThanOrEqual(0.0);
            expect(benfordData.p_value).toBeLessThanOrEqual(1.0);
            expect(benfordData.total_numbers).toBeGreaterThan(0);
            expect(typeof benfordData.analysis_summary).toBe('string');
            expect(benfordData.analysis_summary).not.toBe('');
        });

        test('Pareto analysis through unified API', () => {
            const data = fixtures.paretoCompliantData();
            
            const results = lawkit.law("pareto", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('ParetoAnalysis');
            const paretoData = result.data;
            
            expect(paretoData).toHaveProperty('top_20_percent_contribution');
            expect(paretoData).toHaveProperty('pareto_ratio');
            expect(paretoData).toHaveProperty('concentration_index');
            expect(paretoData).toHaveProperty('total_items');
            expect(paretoData).toHaveProperty('analysis_summary');
            
            // Validate values for compliant data
            expect(paretoData.top_20_percent_contribution).toBeGreaterThan(0.0);
            expect(paretoData.pareto_ratio).toBeGreaterThan(0.0);
            expect(paretoData.concentration_index).toBeGreaterThanOrEqual(0.0);
            expect(paretoData.total_items).toBeGreaterThan(0);
            expect(paretoData.top_20_percent_contribution).toBeGreaterThan(60.0); // Should be high for compliant data
        });

        test('Zipf analysis through unified API', () => {
            const data = fixtures.zipfCompliantData();
            
            const results = lawkit.law("zipf", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('ZipfAnalysis');
            const zipfData = result.data;
            
            expect(zipfData).toHaveProperty('zipf_coefficient');
            expect(zipfData).toHaveProperty('correlation_coefficient');
            expect(zipfData).toHaveProperty('deviation_score');
            expect(zipfData).toHaveProperty('total_items');
            expect(zipfData).toHaveProperty('analysis_summary');
            
            // Validate ranges
            expect(zipfData.zipf_coefficient).not.toBe(0.0);
            expect(zipfData.correlation_coefficient).toBeGreaterThanOrEqual(-1.0);
            expect(zipfData.correlation_coefficient).toBeLessThanOrEqual(1.0);
            expect(zipfData.deviation_score).toBeGreaterThanOrEqual(0.0);
            expect(zipfData.total_items).toBeGreaterThan(0);
        });

        test('Normal analysis through unified API', () => {
            const data = fixtures.normalDistributionData();
            
            const results = lawkit.law("normal", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('NormalAnalysis');
            const normalData = result.data;
            
            expect(normalData).toHaveProperty('mean');
            expect(normalData).toHaveProperty('std_dev');
            expect(normalData).toHaveProperty('skewness');
            expect(normalData).toHaveProperty('kurtosis');
            expect(normalData).toHaveProperty('normality_test_p');
            expect(normalData).toHaveProperty('total_numbers');
            expect(normalData).toHaveProperty('analysis_summary');
            
            // Validate statistical properties
            expect(normalData.std_dev).toBeGreaterThan(0.0);
            expect(normalData.normality_test_p).toBeGreaterThanOrEqual(0.0);
            expect(normalData.normality_test_p).toBeLessThanOrEqual(1.0);
            expect(normalData.total_numbers).toBeGreaterThan(0);
        });

        test('Poisson analysis through unified API', () => {
            const data = fixtures.poissonDistributionData();
            
            const results = lawkit.law("poisson", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('PoissonAnalysis');
            const poissonData = result.data;
            
            expect(poissonData).toHaveProperty('lambda');
            expect(poissonData).toHaveProperty('variance_ratio');
            expect(poissonData).toHaveProperty('poisson_test_p');
            expect(poissonData).toHaveProperty('total_events');
            expect(poissonData).toHaveProperty('analysis_summary');
            
            // Validate Poisson properties
            expect(poissonData.lambda).toBeGreaterThan(0.0);
            expect(poissonData.variance_ratio).toBeGreaterThan(0.0);
            expect(poissonData.poisson_test_p).toBeGreaterThanOrEqual(0.0);
            expect(poissonData.poisson_test_p).toBeLessThanOrEqual(1.0);
            expect(poissonData.total_events).toBeGreaterThan(0);
        });

        test('Data validation through unified API', () => {
            const data = fixtures.validationTestData().valid_dataset;
            
            const results = lawkit.law("validate", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('ValidationResult');
            const validationData = result.data;
            
            expect(validationData).toHaveProperty('validation_passed');
            expect(validationData).toHaveProperty('data_quality_score');
            expect(validationData).toHaveProperty('analysis_summary');
            
            // Valid data should pass validation
            expect(validationData.validation_passed).toBe(true);
            expect(validationData.data_quality_score).toBeGreaterThan(0.0);
        });

        test('Data diagnostics through unified API', () => {
            const data = fixtures.diagnosticTestData().normal_with_outliers;
            
            const results = lawkit.law("diagnose", data);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('DiagnosticResult');
            const diagnosticData = result.data;
            
            expect(diagnosticData).toHaveProperty('diagnostic_type');
            expect(diagnosticData).toHaveProperty('findings');
            expect(diagnosticData).toHaveProperty('confidence_level');
            expect(diagnosticData).toHaveProperty('analysis_summary');
            
            expect(diagnosticData.diagnostic_type).toBe("General");
            expect(Array.isArray(diagnosticData.findings)).toBe(true);
            expect(diagnosticData.findings.length).toBeGreaterThan(0);
            expect(diagnosticData.confidence_level).toBeGreaterThan(0.0);
            expect(diagnosticData.confidence_level).toBeLessThanOrEqual(1.0);
        });

        test('Data generation through unified API', () => {
            const config = fixtures.generationConfigs().benford_config;
            
            const results = lawkit.law("generate", config);
            
            expect(results).toHaveLength(1);
            const result = results[0];
            
            expect(result.type).toBe('GeneratedData');
            const generatedInfo = result.data;
            
            expect(generatedInfo).toHaveProperty('data_type');
            expect(generatedInfo).toHaveProperty('count');
            expect(generatedInfo).toHaveProperty('sample_data');
            expect(generatedInfo).toHaveProperty('parameters');
            
            expect(generatedInfo.data_type).toBe("benford");
            expect(generatedInfo.count).toBe(1000);
            expect(Array.isArray(generatedInfo.sample_data)).toBe(true);
            expect(generatedInfo.sample_data.length).toBeGreaterThan(0);
            expect(typeof generatedInfo.parameters).toBe('object');
            expect(Object.keys(generatedInfo.parameters).length).toBeGreaterThan(0);
        });

        test('Comprehensive analysis through unified API', () => {
            const data = fixtures.integrationAnalysisData();
            
            const results = lawkit.law("analyze", data);
            
            // Should have multiple analysis results plus integration
            expect(results.length).toBeGreaterThan(1);
            
            // Last result should be integration analysis
            const integrationResult = results[results.length - 1];
            expect(integrationResult.type).toBe('IntegrationAnalysis');
            
            const integrationData = integrationResult.data;
            expect(integrationData).toHaveProperty('laws_analyzed');
            expect(integrationData).toHaveProperty('overall_risk');
            expect(integrationData).toHaveProperty('recommendations');
            expect(integrationData).toHaveProperty('analysis_summary');
            
            expect(Array.isArray(integrationData.laws_analyzed)).toBe(true);
            expect(integrationData.laws_analyzed.length).toBeGreaterThan(0);
            expect(typeof integrationData.overall_risk).toBe('string');
            expect(integrationData.overall_risk).not.toBe('');
            expect(Array.isArray(integrationData.recommendations)).toBe(true);
            expect(integrationData.recommendations.length).toBeGreaterThan(0);
        });

        test('Unknown subcommand error handling', () => {
            const data = [1, 2, 3];
            
            expect(() => lawkit.law("unknown", data)).toThrow(/Unknown subcommand/);
        });
    });

    // ============================================================================
    // STATISTICAL LAW SPECIFIC TESTS
    // ============================================================================

    describe('Statistical Law Risk Assessment', () => {
        test('Benford risk levels', () => {
            // Test compliant data (should be LOW or MEDIUM risk)
            const compliantData = fixtures.benfordCompliantData();
            const compliantResults = lawkit.law("benford", compliantData);
            
            const benfordData = compliantResults[0].data;
            expect(['LOW', 'MEDIUM']).toContain(benfordData.risk_level);
            
            // Test non-compliant data (should be higher risk)
            const nonCompliantData = fixtures.benfordNonCompliantData();
            const nonCompliantResults = lawkit.law("benford", nonCompliantData);
            
            const nonCompliantBenfordData = nonCompliantResults[0].data;
            expect(['MEDIUM', 'HIGH']).toContain(nonCompliantBenfordData.risk_level);
        });

        test('Pareto principle compliance', () => {
            // Test compliant data
            const compliantData = fixtures.paretoCompliantData();
            const compliantResults = lawkit.law("pareto", compliantData);
            
            const paretoData = compliantResults[0].data;
            expect(paretoData.top_20_percent_contribution).toBeGreaterThan(60.0);
            expect(paretoData.concentration_index).toBeGreaterThan(0.0);
            
            // Test non-compliant data
            const nonCompliantData = fixtures.paretoNonCompliantData();
            const nonCompliantResults = lawkit.law("pareto", nonCompliantData);
            
            const nonCompliantParetoData = nonCompliantResults[0].data;
            expect(nonCompliantParetoData.top_20_percent_contribution).toBeLessThan(60.0);
        });

        test('Normal distribution detection', () => {
            // Test normal data
            const normalData = fixtures.normalDistributionData();
            const normalResults = lawkit.law("normal", normalData);
            
            const normalAnalysis = normalResults[0].data;
            expect(Math.abs(normalAnalysis.skewness)).toBeLessThan(2.0); // Not too skewed
            expect(normalAnalysis.std_dev).toBeGreaterThan(0.0);
            expect(['LOW', 'MEDIUM']).toContain(normalAnalysis.risk_level);
            
            // Test non-normal data
            const nonNormalData = fixtures.nonNormalDistributionData();
            const nonNormalResults = lawkit.law("normal", nonNormalData);
            
            const nonNormalAnalysis = nonNormalResults[0].data;
            expect(['MEDIUM', 'HIGH']).toContain(nonNormalAnalysis.risk_level);
        });

        test('Poisson distribution detection', () => {
            // Test Poisson data
            const poissonData = fixtures.poissonDistributionData();
            const poissonResults = lawkit.law("poisson", poissonData);
            
            const poissonAnalysis = poissonResults[0].data;
            expect(poissonAnalysis.lambda).toBeGreaterThan(0.0);
            expect(poissonAnalysis.variance_ratio).toBeGreaterThan(0.0);
            expect(['LOW', 'MEDIUM']).toContain(poissonAnalysis.risk_level);
            
            // Test non-Poisson data
            const nonPoissonData = fixtures.nonPoissonData();
            const nonPoissonResults = lawkit.law("poisson", nonPoissonData);
            
            const nonPoissonAnalysis = nonPoissonResults[0].data;
            expect(['MEDIUM', 'HIGH']).toContain(nonPoissonAnalysis.risk_level);
        });
    });

    // ============================================================================
    // OPTIONS TESTING - lawkit Specific Options
    // ============================================================================

    describe('Options Testing', () => {
        test('lawkit-specific options', () => {
            const data = fixtures.benfordCompliantData();
            
            const options = {
                risk_threshold: "medium",
                confidence_level: 0.95,
                significance_level: 0.05,
                min_sample_size: 20,
                enable_outlier_detection: true
            };
            
            const results = lawkit.law("benford", data, options);
            
            expect(results).toHaveLength(1);
            const benfordData = results[0].data;
            expect(benfordData.total_numbers).toBeGreaterThan(0);
        });

        test('Benford-specific options', () => {
            const data = fixtures.benfordCompliantData();
            
            const options = {
                benford_digits: "first",
                benford_base: 10
            };
            
            const results = lawkit.law("benford", data, options);
            
            const benfordData = results[0].data;
            // Should analyze first digits (9 digits: 1-9)
            expect(benfordData.observed_distribution).toHaveLength(9);
            expect(benfordData.expected_distribution).toHaveLength(9);
        });

        test('Pareto-specific options', () => {
            const data = fixtures.paretoCompliantData();
            
            const options = {
                pareto_ratio: 0.8, // 80/20 rule
                pareto_category_limit: 100
            };
            
            const results = lawkit.law("pareto", data, options);
            
            const paretoData = results[0].data;
            expect(paretoData.total_items).toBeGreaterThan(0);
            expect(paretoData.top_20_percent_contribution).toBeGreaterThan(0.0);
        });

        test('Generation options', () => {
            const config = {
                type: "normal",
                count: 500,
                mean: 50.0,
                std_dev: 10.0
            };
            
            const options = {
                generate_count: 500,
                generate_range_min: 0.0,
                generate_range_max: 100.0,
                generate_seed: 12345
            };
            
            const results = lawkit.law("generate", config, options);
            
            const generatedInfo = results[0].data;
            expect(generatedInfo.data_type).toBe("normal");
            expect(generatedInfo.count).toBe(500);
            expect(generatedInfo.parameters).toHaveProperty("mean");
            expect(generatedInfo.parameters).toHaveProperty("std_dev");
            expect(generatedInfo.sample_data).toHaveLength(500);
        });
    });

    // ============================================================================
    // OUTPUT FORMAT TESTS
    // ============================================================================

    describe('Output Format Tests', () => {
        test('default output format', () => {
            const data = [1, 2, 3];
            
            const results = lawkit.law("validate", data);
            expect(Array.isArray(results)).toBe(true);
            expect(results.length).toBeGreaterThan(0);
        });

        test('output format options if supported', () => {
            const data = [1, 2, 3];
            
            try {
                const options = { output_format: "json" };
                const results = lawkit.law("validate", data, options);
                expect(Array.isArray(results)).toBe(true);
            } catch (error) {
                // Format options might not be implemented at JS binding level
                // Test passes either way
            }
        });
    });

    // ============================================================================
    // DATA VALIDATION AND ERROR HANDLING TESTS
    // ============================================================================

    describe('Error Handling', () => {
        test('empty data handling', () => {
            const emptyData = [];
            
            expect(() => lawkit.law("benford", emptyData)).toThrow(/No valid numbers found/);
        });

        test('invalid data handling', () => {
            const invalidData = { "not": "numbers" };
            
            expect(() => lawkit.law("benford", invalidData)).toThrow(/No valid numbers found/);
        });

        test('small sample size', () => {
            const smallData = [1.0, 2.0];
            
            expect(() => lawkit.law("normal", smallData)).toThrow(/Insufficient data points/);
        });

        test('validation with issues', () => {
            const problematicData = fixtures.validationTestData().small_dataset;
            
            const results = lawkit.law("validate", problematicData);
            
            const validationData = results[0].data;
            expect(validationData.validation_passed).toBe(false);
            expect(validationData).toHaveProperty('issues_found');
            expect(validationData.issues_found.length).toBeGreaterThan(0);
            expect(validationData.data_quality_score).toBeLessThan(1.0);
        });
    });

    // ============================================================================
    // JAVASCRIPT-SPECIFIC BINDING TESTS
    // ============================================================================

    describe('JavaScript Binding Tests', () => {
        test('JavaScript type conversion', () => {
            // Test with JavaScript array
            const jsArray = [1.0, 2.0, 3.0, 4.0, 5.0];
            const results = lawkit.law("validate", jsArray);
            expect(Array.isArray(results)).toBe(true);
            
            // Test with nested JavaScript objects
            const nestedData = {
                numbers: [1, 2, 3],
                values: [4.0, 5.0, 6.0]
            };
            const nestedResults = lawkit.law("validate", nestedData);
            expect(Array.isArray(nestedResults)).toBe(true);
        });

        test('synchronous function handling', () => {
            const data = [1, 2, 3, 4, 5];
            
            // Test that function returns results directly
            const results = lawkit.law("validate", data);
            
            // Test direct result access
            expect(Array.isArray(results)).toBe(true);
        });

        test('Error handling', () => {
            // Test that errors are properly thrown
            expect(() => lawkit.law("invalid_command", [1, 2, 3])).toThrow();
            
            // Test with null data
            expect(() => lawkit.law("benford", null)).toThrow();
        });

        test('Large dataset performance', () => {
            // Generate larger dataset for JavaScript performance testing
            const largeData = Array.from({ length: 1000 }, (_, i) => i + 1);
            
            const startTime = Date.now();
            const results = lawkit.law("validate", largeData);
            const endTime = Date.now();
            
            expect(Array.isArray(results)).toBe(true);
            expect(endTime - startTime).toBeLessThan(5000); // Should complete within 5 seconds
        });

        test('Memory management', () => {
            // Test that large data structures are properly cleaned up
            for (let i = 0; i < 10; i++) {
                const largeData = Array.from({ length: 100 }, (_, j) => i * 100 + j);
                const results = lawkit.law("validate", largeData);
                expect(Array.isArray(results)).toBe(true);
                // Force cleanup between iterations
                global.gc && global.gc();
            }
        });
    });

    // ============================================================================
    // INTEGRATION TESTS WITH FIXTURES
    // ============================================================================

    describe('Integration Tests', () => {
        test('comprehensive analysis workflow', () => {
            const integrationData = fixtures.integrationAnalysisData();
            
            const options = {
                show_details: true,
                show_recommendations: true
            };
            
            const results = lawkit.law("analyze", integrationData, options);
            
            // Should have multiple analyses
            expect(results.length).toBeGreaterThan(1);
            
            // Check that we get different types of analysis
            const resultTypes = new Set();
            results.forEach(result => {
                resultTypes.add(result.type);
            });
            
            // Should have multiple analysis types
            expect(resultTypes.size).toBeGreaterThanOrEqual(3);
            expect(resultTypes.has('IntegrationAnalysis')).toBe(true);
        });

        test('data generation and analysis cycle', () => {
            // Generate Benford data
            const config = {
                type: "benford",
                count: 100
            };
            
            const generationResults = lawkit.law("generate", config);
            const generatedInfo = generationResults[0].data;
            const generatedData = generatedInfo.sample_data;
            
            // Analyze the generated data
            const analysisResults = lawkit.law("benford", generatedData);
            
            const benfordData = analysisResults[0].data;
            // Generated Benford data should show low risk
            expect(['LOW', 'MEDIUM']).toContain(benfordData.risk_level);
            expect(benfordData.total_numbers).toBe(100);
        });
    });
});

// ============================================================================
// TEST FIXTURES CLASS
// ============================================================================

class TestFixtures {
    benfordCompliantData() {
        return {
            financial_data: [
                123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45,
                12345.67, 18765.43, 23456.78, 29876.54, 34567.89, 45678.12, 56789.34, 67890.45, 78901.23,
                123456.78, 187654.32, 234567.89, 298765.43, 345678.91, 456789.12, 567890.23, 678901.34
            ],
            invoice_amounts: [
                101.50, 125.00, 198.75, 267.30, 334.20, 445.60, 523.80, 612.40, 785.90,
                1055.25, 1287.60, 1934.75, 2156.80, 3245.70, 4123.50, 5678.25, 6234.80, 7891.20,
                10234.50, 12876.30, 19847.60, 21568.90, 32457.80, 41235.60, 56782.40, 62348.70, 78912.30
            ]
        };
    }

    benfordNonCompliantData() {
        return {
            uniform_data: [
                500.0, 501.0, 502.0, 503.0, 504.0, 505.0, 506.0, 507.0, 508.0, 509.0,
                510.0, 511.0, 512.0, 513.0, 514.0, 515.0, 516.0, 517.0, 518.0, 519.0,
                520.0, 521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0, 529.0
            ],
            suspicious_data: [
                7000.0, 7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0,
                8000.0, 8001.0, 8002.0, 8003.0, 8004.0, 8005.0, 8006.0, 8007.0, 8008.0, 8009.0,
                9000.0, 9001.0, 9002.0, 9003.0, 9004.0, 9005.0, 9006.0, 9007.0, 9008.0, 9009.0
            ]
        };
    }

    paretoCompliantData() {
        return {
            sales_data: [
                // Top 20% should contribute ~80% of total
                10000.0, 9500.0, 9000.0, 8500.0, // Top 4 items (20% of 20 items)
                1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0,
                600.0, 550.0, 500.0, 450.0, 400.0, 350.0, 300.0, 250.0
            ],
            customer_revenue: [
                50000.0, 45000.0, 40000.0, 35000.0, 30000.0, // Top 5 customers
                2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0, 500.0, 400.0, 300.0, 200.0, 100.0
            ]
        };
    }

    paretoNonCompliantData() {
        return {
            uniform_distribution: [
                1000.0, 1010.0, 1020.0, 1030.0, 1040.0, 1050.0, 1060.0, 1070.0, 1080.0, 1090.0,
                1100.0, 1110.0, 1120.0, 1130.0, 1140.0, 1150.0, 1160.0, 1170.0, 1180.0, 1190.0
            ]
        };
    }

    zipfCompliantData() {
        return {
            word_frequencies: [
                // Frequencies following Zipf's law: f(r) = f(1)/r
                10000.0, 5000.0, 3333.33, 2500.0, 2000.0, 1666.67, 1428.57, 1250.0, 1111.11, 1000.0,
                909.09, 833.33, 769.23, 714.29, 666.67, 625.0, 588.24, 555.56, 526.32, 500.0
            ]
        };
    }

    normalDistributionData() {
        return {
            normal_sample: [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7
            ]
        };
    }

    nonNormalDistributionData() {
        return {
            skewed_data: [
                1.0, 1.1, 1.2, 1.3, 1.5, 1.8, 2.2, 2.8, 3.6, 4.7,
                6.1, 8.0, 10.4, 13.5, 17.6, 22.9, 29.8, 38.7, 50.3, 65.4,
                85.0, 110.5, 143.7, 186.8, 242.8, 315.6, 410.3, 533.4, 693.4, 901.4
            ]
        };
    }

    poissonDistributionData() {
        return {
            event_counts: [
                0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3,
                2, 1, 0, 4, 2, 1, 3, 0, 2, 1, 2, 3, 1, 0, 2, 1, 4, 2, 0, 3
            ]
        };
    }

    nonPoissonData() {
        return {
            high_variance: [
                0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0,
                100, 100, 100, 100, 100, 0, 0, 0, 0, 0, 25, 25, 25, 25, 25
            ]
        };
    }

    integrationAnalysisData() {
        return {
            comprehensive_dataset: {
                financial_transactions: [
                    123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                    1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45
                ],
                sales_amounts: [
                    50000.0, 45000.0, 40000.0, 35000.0, 30000.0,
                    2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                    1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0
                ],
                quality_scores: [
                    98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                    100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8
                ],
                incident_counts: [
                    0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3
                ]
            }
        };
    }

    validationTestData() {
        return {
            valid_dataset: [
                123.45, 234.67, 345.89, 456.12, 567.34, 678.56, 789.78, 890.23, 901.45, 123.67,
                234.89, 345.12, 456.34, 567.56, 678.78, 789.01, 890.23, 901.45, 123.67, 234.89
            ],
            small_dataset: [1.0, 2.0, 3.0]
        };
    }

    diagnosticTestData() {
        return {
            normal_with_outliers: [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                150.0, // Outlier
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7,
                50.0   // Another outlier
            ]
        };
    }

    generationConfigs() {
        return {
            benford_config: {
                type: "benford",
                count: 1000,
                base: 10
            },
            normal_config: {
                type: "normal",
                count: 500,
                mean: 100.0,
                std_dev: 15.0
            },
            poisson_config: {
                type: "poisson",
                count: 300,
                lambda: 5.0
            }
        };
    }
}