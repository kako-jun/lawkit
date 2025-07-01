/// Test fixtures for various input formats and edge cases

/// Japanese numeral test cases with expected conversions
pub struct JapaneseNumeralTest {
    pub input: &'static str,
    pub expected: &'static str,
    pub description: &'static str,
}

pub const JAPANESE_NUMERAL_TESTS: &[JapaneseNumeralTest] = &[
    // Full-width digits
    JapaneseNumeralTest {
        input: "１２３",
        expected: "123",
        description: "Basic full-width digits",
    },
    JapaneseNumeralTest {
        input: "０１２３４５６７８９",
        expected: "0123456789",
        description: "All full-width digits",
    },
    
    // Basic kanji numerals
    JapaneseNumeralTest {
        input: "一",
        expected: "1",
        description: "Single kanji digit",
    },
    JapaneseNumeralTest {
        input: "一二三四五六七八九",
        expected: "1 2 3 4 5 6 7 8 9",
        description: "Sequential kanji digits",
    },
    
    // Kanji with position values
    JapaneseNumeralTest {
        input: "十",
        expected: "10",
        description: "Ten in kanji",
    },
    JapaneseNumeralTest {
        input: "一十",
        expected: "10",
        description: "Explicit one-ten",
    },
    JapaneseNumeralTest {
        input: "二十",
        expected: "20",
        description: "Twenty in kanji",
    },
    JapaneseNumeralTest {
        input: "二十三",
        expected: "23",
        description: "Twenty-three in kanji",
    },
    JapaneseNumeralTest {
        input: "百",
        expected: "100",
        description: "Hundred in kanji",
    },
    JapaneseNumeralTest {
        input: "一百二十三",
        expected: "123",
        description: "One hundred twenty-three",
    },
    JapaneseNumeralTest {
        input: "千",
        expected: "1000",
        description: "Thousand in kanji",
    },
    JapaneseNumeralTest {
        input: "一千二百三十四",
        expected: "1234",
        description: "One thousand two hundred thirty-four",
    },
    JapaneseNumeralTest {
        input: "万",
        expected: "10000",
        description: "Ten thousand in kanji",
    },
    JapaneseNumeralTest {
        input: "一万二千三百四十五",
        expected: "12345",
        description: "Twelve thousand three hundred forty-five",
    },
    
    // Complex cases
    JapaneseNumeralTest {
        input: "九万八千七百六十五",
        expected: "98765",
        description: "Large number in kanji",
    },
    
    // Mixed patterns
    JapaneseNumeralTest {
        input: "売上１２３万円",
        expected: "売上123万円",
        description: "Full-width in context",
    },
    JapaneseNumeralTest {
        input: "利益一千二百万円",
        expected: "利益1200万円",
        description: "Kanji number in context",
    },
    JapaneseNumeralTest {
        input: "１２３万４５６７円",
        expected: "123万4567円",
        description: "Mixed full-width and half-width",
    },
];

/// Edge cases for number extraction
pub struct NumberExtractionTest {
    pub input: &'static str,
    pub expected: Vec<f64>,
    pub description: &'static str,
}

pub const NUMBER_EXTRACTION_TESTS: &[NumberExtractionTest] = &[
    // Basic cases
    NumberExtractionTest {
        input: "123 456 789",
        expected: vec![123.0, 456.0, 789.0],
        description: "Space-separated integers",
    },
    NumberExtractionTest {
        input: "12.34 56.78 90.12",
        expected: vec![12.34, 56.78, 90.12],
        description: "Space-separated decimals",
    },
    
    // Edge cases for Benford's Law
    NumberExtractionTest {
        input: "0 001 0.123 00456",
        expected: vec![0.0, 1.0, 0.123, 456.0],
        description: "Leading zeros (should be filtered for Benford)",
    },
    NumberExtractionTest {
        input: "-123 -456.78 -0.9",
        expected: vec![-123.0, -456.78, -0.9],
        description: "Negative numbers",
    },
    NumberExtractionTest {
        input: "1,234.56 7,890.12",
        expected: vec![1234.56, 7890.12],
        description: "Numbers with commas",
    },
    
    // Japanese context
    NumberExtractionTest {
        input: "価格：１２３円 数量：４５個",
        expected: vec![123.0, 45.0],
        description: "Japanese text with full-width numbers",
    },
    
    // Empty and invalid cases
    NumberExtractionTest {
        input: "",
        expected: vec![],
        description: "Empty string",
    },
    NumberExtractionTest {
        input: "No numbers here!",
        expected: vec![],
        description: "No numbers in text",
    },
];

/// Test cases for Benford's Law calculations
pub struct BenfordCalculationTest {
    pub numbers: Vec<f64>,
    pub expected_distribution: [f64; 9], // Percentages for digits 1-9
    pub expected_chi_square_range: (f64, f64), // Min and max expected values
    pub expected_risk_level: &'static str,
    pub description: &'static str,
}

pub const BENFORD_CALCULATION_TESTS: &[BenfordCalculationTest] = &[
    // Perfect Benford distribution (theoretical)
    BenfordCalculationTest {
        numbers: vec![], // Will be filled with perfect Benford data
        expected_distribution: [30.1, 17.6, 12.5, 9.7, 7.9, 6.7, 5.8, 5.1, 4.6],
        expected_chi_square_range: (0.0, 2.0), // Should be very low
        expected_risk_level: "LOW",
        description: "Perfect Benford distribution",
    },
    
    // Fraudulent distribution (too many 5s and 6s)
    BenfordCalculationTest {
        numbers: vec![
            5.0, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7, 5.8, 5.9,
            6.0, 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 6.7, 6.8, 6.9,
            1.0, 2.0, 3.0, 4.0, 7.0, 8.0, 9.0,
        ],
        expected_distribution: [3.7, 3.7, 3.7, 3.7, 37.0, 37.0, 3.7, 3.7, 3.7],
        expected_chi_square_range: (100.0, 1000.0), // Should be very high
        expected_risk_level: "CRITICAL",
        description: "Artificially skewed distribution",
    },
];

/// File format test data
pub const SAMPLE_CSV: &str = r#"Name,Amount,Date
Sales,1234.56,2023-01-01
Expenses,567.89,2023-01-02
Revenue,9876.54,2023-01-03
"#;

pub const SAMPLE_JSON: &str = r#"{
    "transactions": [
        {"amount": 1234.56, "type": "income"},
        {"amount": 567.89, "type": "expense"},
        {"amount": 9876.54, "type": "income"}
    ]
}"#;

pub const SAMPLE_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><title>Financial Report</title></head>
<body>
    <h1>Q1 Financial Report</h1>
    <p>Revenue: $1,234,567</p>
    <p>Expenses: $456,789</p>
    <p>Profit: $777,778</p>
    <table>
        <tr><td>Item 1</td><td>$123.45</td></tr>
        <tr><td>Item 2</td><td>$678.90</td></tr>
    </table>
</body>
</html>"#;