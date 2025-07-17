use std::process::Command;
use std::fs;
use regex::Regex;

#[test]
fn test_all_charts_have_consistent_width() {
    // Create test data file
    let test_data = "123 456 789 101 202 303 404 505 606 707 808 909 1010 1111 1212 1313 1414 1515 1616 1717 1818 1919 2020 2121 2222 2323 2424 2525 2626 2727 2828 2929 3030 3131 3232";
    let temp_file = "/tmp/chart_test_data.txt";
    fs::write(temp_file, test_data).expect("Failed to write test data");
    
    const EXPECTED_CHART_WIDTH: usize = 50;
    
    // Test each statistical law
    let laws = vec![
        ("benf", "Benford Law"),
        ("zipf", "Zipf Law"),
        ("normal", "Normal Distribution"),
        ("pareto", "Pareto Analysis"),
        ("poisson", "Poisson Distribution"),
    ];
    
    for (law_cmd, law_name) in laws {
        println!("Testing {} chart consistency...", law_name);
        
        let output = Command::new("target/release/lawkit")
            .arg(law_cmd)
            .arg(temp_file)
            .output()
            .expect(&format!("Failed to execute lawkit {} command", law_cmd));
        
        let output_str = String::from_utf8_lossy(&output.stderr);
        verify_chart_width_in_output(&output_str, EXPECTED_CHART_WIDTH, law_name);
    }
    
    // Clean up
    fs::remove_file(temp_file).ok();
}

fn verify_chart_width_in_output(output: &str, expected_width: usize, law_name: &str) {
    // Regex to match chart lines with bars
    let chart_line_regex = Regex::new(r".*[█░┃].*").unwrap();
    
    let mut chart_lines_found = 0;
    
    for line in output.lines() {
        if chart_line_regex.is_match(line) {
            if let Some(chart_part) = extract_chart_from_line(line) {
                let char_count = chart_part.chars().count();
                
                // Allow some tolerance for different chart formats
                if char_count > 0 {
                    chart_lines_found += 1;
                    assert_eq!(
                        char_count, expected_width,
                        "{} chart line has {} characters, expected {}. Line: '{}'",
                        law_name, char_count, expected_width, line
                    );
                }
            }
        }
    }
    
    // Ensure we found at least some chart lines
    assert!(
        chart_lines_found > 0,
        "No chart lines found in {} output",
        law_name
    );
    
    println!("✓ {} chart consistency verified ({} lines)", law_name, chart_lines_found);
}

fn extract_chart_from_line(line: &str) -> Option<String> {
    // Extract the continuous sequence of chart characters
    let chart_chars: Vec<char> = line.chars()
        .filter(|c| *c == '█' || *c == '░' || *c == '┃')
        .collect();
    
    if chart_chars.is_empty() {
        return None;
    }
    
    // Find the start and end of the chart section
    let line_chars: Vec<char> = line.chars().collect();
    let mut start_idx = None;
    let mut end_idx = None;
    
    for (i, c) in line_chars.iter().enumerate() {
        if *c == '█' || *c == '░' || *c == '┃' {
            if start_idx.is_none() {
                start_idx = Some(i);
            }
            end_idx = Some(i);
        }
    }
    
    if let (Some(start), Some(end)) = (start_idx, end_idx) {
        let chart_section: String = line_chars[start..=end].iter().collect();
        
        // Verify it's a continuous chart section
        if chart_section.chars().all(|c| c == '█' || c == '░' || c == '┃') {
            return Some(chart_section);
        }
    }
    
    None
}