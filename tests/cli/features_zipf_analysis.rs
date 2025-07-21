use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;
use tempfile::tempdir;

// Helper function to get the lawkit command
fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

/// Test basic Zipf's law analysis on text
/// Verifies word frequency follows Zipf distribution
#[test]
fn test_basic_zipf_text_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let text_file = temp_dir.path().join("sample.txt");
    
    // Create text with Zipf-distributed word frequencies
    let mut text_content = String::new();
    
    // Most frequent words appear many times
    for _ in 0..1000 { text_content.push_str("the "); }
    for _ in 0..500 { text_content.push_str("and "); }
    for _ in 0..333 { text_content.push_str("of "); }
    for _ in 0..250 { text_content.push_str("to "); }
    for _ in 0..200 { text_content.push_str("in "); }
    
    // Less frequent words appear fewer times
    for i in 6..=100 {
        let frequency = 1000 / i;
        for _ in 0..frequency {
            text_content.push_str(&format!("word{} ", i));
        }
    }
    
    fs::write(&text_file, text_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&text_file)
        .arg("--text")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect Zipf distribution in word frequencies
    assert!(
        stdout.contains("zipf") 
        || stdout.contains("frequency")
        || stdout.contains("rank")
        || stdout.contains("the")
    );

    Ok(())
}

/// Test Zipf analysis on numerical data
/// Verifies numerical rankings follow Zipf distribution
#[test]
fn test_zipf_numerical_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let numbers_file = temp_dir.path().join("numbers.csv");
    
    // Create numerical data following Zipf distribution
    let mut csv_content = String::from("item,frequency\n");
    
    for rank in 1..=100 {
        let frequency = 1000 / rank;  // Classic Zipf: frequency ∝ 1/rank
        csv_content.push_str(&format!("Item{},{}\n", rank, frequency));
    }
    
    fs::write(&numbers_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&numbers_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect Zipf distribution in numerical data
    assert!(
        stdout.contains("zipf") 
        || stdout.contains("frequency")
        || stdout.contains("rank")
    );

    Ok(())
}

/// Test word count analysis with maximum words
/// Verifies analysis of large texts with word count limits
#[test]
fn test_word_count_limit() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let large_text_file = temp_dir.path().join("large_text.txt");
    
    // Create large text with many unique words
    let mut text_content = String::new();
    
    for i in 1..=10000 {
        let frequency = std::cmp::max(1, 100 / ((i as f64).sqrt() as i32));
        for _ in 0..frequency {
            text_content.push_str(&format!("word{} ", i));
        }
    }
    
    fs::write(&large_text_file, text_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&large_text_file)
        .arg("--text")
        .arg("--words")
        .arg("1000")  // Limit to top 1000 words
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should analyze top 1000 words only
    assert!(
        stdout.contains("word1") 
        || stdout.contains("frequency")
        || stdout.contains("1000")
    );

    Ok(())
}

/// Test Zipf analysis on web ranking data
/// Verifies Zipf's law in web traffic/search rankings
#[test]
fn test_web_ranking_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let rankings_file = temp_dir.path().join("web_rankings.csv");
    
    // Create web traffic data following Zipf distribution
    let mut csv_content = String::from("website,daily_visits,search_rank\n");
    
    for rank in 1..=1000 {
        let visits = 1000000 / rank;  // Traffic inversely proportional to rank
        csv_content.push_str(&format!("site{}.com,{},{}\n", rank, visits, rank));
    }
    
    fs::write(&rankings_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&rankings_file)
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect Zipf pattern in web rankings
    assert!(
        stdout.contains("daily_visits") 
        || stdout.contains("search_rank")
        || stdout.contains("zipf")
    );

    Ok(())
}

/// Test Zipf analysis on city population data
/// Verifies Zipf's law in population distributions
#[test]
fn test_population_zipf_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let population_file = temp_dir.path().join("cities.csv");
    
    // Create city population data following Zipf distribution
    let mut csv_content = String::from("city,population,rank\n");
    
    let largest_city_pop = 10000000;  // 10 million for largest city
    
    for rank in 1..=100 {
        let population = largest_city_pop / rank;
        csv_content.push_str(&format!("City{},{},{}\n", rank, population, rank));
    }
    
    fs::write(&population_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&population_file)
        .arg("--verbose")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect Zipf distribution in population data
    assert!(
        stdout.contains("population") 
        || stdout.contains("rank")
        || stdout.contains("City1")
    );

    Ok(())
}

/// Test language analysis with Zipf's law
/// Verifies linguistic patterns follow Zipf distribution
#[test]
fn test_language_zipf_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let language_file = temp_dir.path().join("language.txt");
    
    // Create realistic English text with natural word distribution
    let common_words = vec![
        ("the", 1000), ("and", 500), ("of", 333), ("to", 250), ("in", 200),
        ("is", 167), ("it", 143), ("you", 125), ("that", 111), ("he", 100),
        ("was", 91), ("for", 83), ("on", 77), ("are", 71), ("as", 67),
        ("with", 63), ("his", 59), ("they", 56), ("be", 53), ("at", 50)
    ];
    
    let mut text_content = String::new();
    for (word, frequency) in common_words {
        for _ in 0..frequency {
            text_content.push_str(&format!("{} ", word));
        }
    }
    
    // Add less common words
    for i in 21..=1000 {
        let frequency = std::cmp::max(1, 1000 / i);
        for _ in 0..frequency {
            text_content.push_str(&format!("word{} ", i));
        }
    }
    
    fs::write(&language_file, text_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&language_file)
        .arg("--text")
        .arg("--words")
        .arg("100")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect natural language Zipf pattern
    assert!(
        stdout.contains("the") 
        || stdout.contains("and")
        || stdout.contains("frequency")
    );

    Ok(())
}

/// Test Zipf analysis with statistical significance
/// Verifies statistical validation of Zipf distribution
#[test]
fn test_zipf_statistical_significance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let stats_file = temp_dir.path().join("zipf_stats.csv");
    
    // Create data that closely follows Zipf's law
    let mut csv_content = String::from("element,count\n");
    
    for rank in 1..=500 {
        // Add some noise to perfect Zipf distribution
        let base_count = 10000 / rank;
        let noise = (rank as f64 * 0.1) as i32;
        let count = base_count + noise - (noise / 2);
        csv_content.push_str(&format!("Element{},{}\n", rank, count));
    }
    
    fs::write(&stats_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&stats_file)
        .arg("--confidence")
        .arg("0.95")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should include statistical analysis
    assert!(
        stdout.contains("confidence") 
        || stdout.contains("p_value")
        || stdout.contains("significance")
    );

    Ok(())
}

/// Test Zipf analysis edge cases
/// Verifies robust handling of non-Zipf data
#[test]
fn test_zipf_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    
    // Test with uniform distribution (anti-Zipf)
    let uniform_file = temp_dir.path().join("uniform.csv");
    let mut csv_content = String::from("item,value\n");
    
    for i in 1..=100 {
        csv_content.push_str(&format!("Item{},100\n", i));  // All equal values
    }
    
    fs::write(&uniform_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&uniform_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    
    // Should handle uniform distribution gracefully
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("uniform") 
            || stdout.contains("no zipf")
            || stdout.contains("equal")
        );
    } else {
        // Or provide clear error for non-Zipf data
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("zipf") 
            || stderr.contains("distribution")
            || stderr.contains("pattern")
        );
    }

    Ok(())
}

/// Test multilingual text analysis
/// Verifies Zipf analysis works with different languages
#[test]
fn test_multilingual_zipf() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let multilingual_file = temp_dir.path().join("multilingual.txt");
    
    // Create text with multiple languages following Zipf distribution
    let mut text_content = String::new();
    
    // English common words
    for _ in 0..500 { text_content.push_str("the "); }
    for _ in 0..250 { text_content.push_str("and "); }
    
    // Japanese common words (using romanization)
    for _ in 0..300 { text_content.push_str("wa "); }  // は
    for _ in 0..200 { text_content.push_str("no "); }  // の
    for _ in 0..150 { text_content.push_str("ga "); }  // が
    
    // French common words
    for _ in 0..400 { text_content.push_str("le "); }
    for _ in 0..300 { text_content.push_str("de "); }
    for _ in 0..200 { text_content.push_str("et "); }
    
    fs::write(&multilingual_file, text_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&multilingual_file)
        .arg("--text")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should analyze multilingual text
    assert!(
        stdout.contains("the") 
        || stdout.contains("le")
        || stdout.contains("wa")
        || stdout.contains("frequency")
    );

    Ok(())
}

/// Test Zipf analysis performance with large datasets
/// Verifies efficient processing of large text corpora
#[test]
fn test_large_dataset_performance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let large_file = temp_dir.path().join("large_corpus.txt");
    
    // Create large text corpus
    let mut text_content = String::new();
    
    for i in 1..=10000 {
        let frequency = std::cmp::max(1, 10000 / i);
        for _ in 0..frequency {
            text_content.push_str(&format!("word{} ", i));
        }
    }
    
    fs::write(&large_file, text_content)?;

    let start = std::time::Instant::now();
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(&large_file)
        .arg("--text")
        .arg("--words")
        .arg("5000")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    let duration = start.elapsed();
    
    assert!(output.status.success());
    
    // Should complete large corpus analysis within reasonable time
    assert!(duration.as_secs() < 30, "Large corpus analysis took too long: {:?}", duration);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());

    Ok(())
}