/// Benford's Law expected percentages for first digits 1-9
pub const BENFORD_EXPECTED_PERCENTAGES: [f64; 9] = [
    30.103, // 1
    17.609, // 2
    12.494, // 3
    9.691,  // 4
    7.918,  // 5
    6.695,  // 6
    5.799,  // 7
    5.115,  // 8
    4.576,  // 9
];

/// Extract first digit from a number
pub fn get_first_digit(number: f64) -> Option<u8> {
    let abs_num = number.abs();
    if abs_num < 1.0 {
        return None; // Skip numbers less than 1
    }

    let mut digit = abs_num;
    while digit >= 10.0 {
        digit /= 10.0;
    }

    let first_digit = digit as u8;
    if (1..=9).contains(&first_digit) {
        Some(first_digit)
    } else {
        None
    }
}

/// Calculate the distribution of first digits in a dataset
pub fn calculate_digit_distribution(numbers: &[f64]) -> [f64; 9] {
    let mut counts = [0; 9];
    let mut total_valid = 0;

    for &number in numbers {
        if let Some(digit) = get_first_digit(number) {
            counts[(digit - 1) as usize] += 1;
            total_valid += 1;
        }
    }

    let mut distribution = [0.0; 9];
    if total_valid > 0 {
        for i in 0..9 {
            distribution[i] = (counts[i] as f64 / total_valid as f64) * 100.0;
        }
    }

    distribution
}

/// Perform Benford's Law analysis on a dataset
pub fn analyze_benford_law(
    numbers: &[f64],
    dataset_name: &str,
) -> crate::error::Result<crate::laws::benford::BenfordResult> {
    crate::laws::benford::BenfordResult::new(dataset_name.to_string(), numbers)
}
