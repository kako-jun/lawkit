/// Calculate chi-square statistic
pub fn calculate_chi_square(observed: &[f64], expected: &[f64]) -> f64 {
    observed
        .iter()
        .zip(expected.iter())
        .map(|(&obs, &exp)| {
            if exp > 0.0 {
                let diff = obs - exp;
                (diff * diff) / exp
            } else {
                0.0
            }
        })
        .sum()
}

/// Calculate p-value from chi-square statistic and degrees of freedom
/// This is a simplified approximation - in production, use a proper statistics library
pub fn calculate_p_value(chi_square: f64, degrees_of_freedom: i32) -> f64 {
    // Simplified p-value calculation using normal approximation
    // For more accuracy, use a proper chi-square distribution function

    if chi_square <= 0.0 {
        return 1.0;
    }

    // Simple approximation based on common critical values
    match degrees_of_freedom {
        8 => {
            if chi_square >= 20.09 {
                0.01
            }
            // p <= 0.01
            else if chi_square >= 15.51 {
                0.05
            }
            // p <= 0.05
            else if chi_square >= 13.36 {
                0.1
            }
            // p <= 0.1
            else {
                0.5
            } // p > 0.1 (approximate)
        }
        _ => {
            // Fallback approximation
            if chi_square >= 20.0 {
                0.01
            } else if chi_square >= 15.0 {
                0.05
            } else if chi_square >= 12.0 {
                0.1
            } else {
                0.5
            }
        }
    }
}

/// Calculate Mean Absolute Deviation (MAD)
pub fn calculate_mad(observed: &[f64], expected: &[f64]) -> f64 {
    let sum: f64 = observed
        .iter()
        .zip(expected.iter())
        .map(|(&obs, &exp)| (obs - exp).abs())
        .sum();

    sum / observed.len() as f64
}
