#!/usr/bin/env node

const { runLawkit } = require('./index.js');
const fs = require('fs');
const path = require('path');

// Example datasets
const financialData = [
  "123.45", "234.56", "345.67", "456.78", "567.89",
  "678.90", "789.01", "890.12", "901.23", "112.34",
  "223.45", "334.56", "445.67", "556.78", "667.89",
  "778.90", "889.01", "990.12", "101.23", "212.34"
];

const salesData = [
  "Product A,1500", "Product B,1200", "Product C,800",
  "Product D,600", "Product E,400", "Product F,300",
  "Product G,200", "Product H,150", "Product I,100",
  "Product J,50", "Product K,25", "Product L,10"
];

const eventData = [
  "2024-01-01,3", "2024-01-02,2", "2024-01-03,4",
  "2024-01-04,1", "2024-01-05,3", "2024-01-06,2",
  "2024-01-07,5", "2024-01-08,1", "2024-01-09,3",
  "2024-01-10,2", "2024-01-11,4", "2024-01-12,1"
];

const textData = `
The quick brown fox jumps over the lazy dog. The dog was sleeping under the tree.
The fox was very quick and agile. The tree provided good shade for the dog.
In the forest, many animals live peacefully together. The fox and dog became friends.
`;

async function runExample(title, description, lawkitArgs, tempFile = null) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`📊 ${title}`);
  console.log(`${description}`);
  console.log(`Command: lawkit ${lawkitArgs.join(' ')}`);
  console.log(`${'='.repeat(60)}`);
  
  try {
    const result = await runLawkit(lawkitArgs);
    
    if (result.code === 0) {
      console.log('✅ SUCCESS');
      console.log('\nOutput:');
      console.log(result.stdout);
    } else {
      console.log('❌ FAILED');
      console.log('\nError:');
      console.log(result.stderr);
    }
  } catch (error) {
    console.log('❌ ERROR:', error.message);
  } finally {
    // Clean up temp file if created
    if (tempFile && fs.existsSync(tempFile)) {
      fs.unlinkSync(tempFile);
    }
  }
}

async function main() {
  console.log('🚀 LAWKIT-JS EXAMPLES');
  console.log('Demonstrating various statistical analysis capabilities');
  
  // Example 1: Benford's Law Analysis
  const benfordFile = path.join(__dirname, 'temp-financial.csv');
  fs.writeFileSync(benfordFile, financialData.join('\n'));
  
  await runExample(
    'Benford\'s Law - Fraud Detection',
    'Analyzing financial data for potential fraud indicators using Benford\'s Law',
    ['benf', benfordFile, '--format', 'json'],
    benfordFile
  );
  
  // Example 2: Pareto Analysis
  const paretoFile = path.join(__dirname, 'temp-sales.csv');
  fs.writeFileSync(paretoFile, 'Product,Sales\n' + salesData.join('\n'));
  
  await runExample(
    'Pareto Analysis - 80/20 Rule',
    'Analyzing sales data to identify top-performing products (80/20 rule)',
    ['pareto', paretoFile, '--business-analysis'],
    paretoFile
  );
  
  // Example 3: Poisson Distribution
  const poissonFile = path.join(__dirname, 'temp-events.csv');
  fs.writeFileSync(poissonFile, 'Date,Count\n' + eventData.join('\n'));
  
  await runExample(
    'Poisson Distribution - Event Prediction',
    'Analyzing event data and predicting future occurrences',
    ['poisson', poissonFile, '--predict', '7', '--format', 'json'],
    poissonFile
  );
  
  // Example 4: Zipf's Law
  const zipfFile = path.join(__dirname, 'temp-text.txt');
  fs.writeFileSync(zipfFile, textData);
  
  await runExample(
    'Zipf\'s Law - Text Analysis',
    'Analyzing text content for word frequency patterns',
    ['zipf', zipfFile, '--language', 'en'],
    zipfFile
  );
  
  // Example 5: Generate Sample Data
  await runExample(
    'Data Generation - Benford Distribution',
    'Generating sample data that follows Benford\'s Law',
    ['generate', 'benf', '--samples', '20', '--seed', '42']
  );
  
  // Example 6: Comparative Analysis
  const compareFile = path.join(__dirname, 'temp-compare.csv');
  fs.writeFileSync(compareFile, financialData.join('\n'));
  
  await runExample(
    'Comparative Analysis - Multi-Law Validation',
    'Comparing multiple statistical laws to find the best fit',
    ['compare', compareFile, '--format', 'json'],
    compareFile
  );
  
  // Example 7: Normal Distribution
  const normalData = Array.from({length: 50}, (_, i) => 
    (100 + (Math.random() - 0.5) * 30).toFixed(2)
  );
  const normalFile = path.join(__dirname, 'temp-normal.csv');
  fs.writeFileSync(normalFile, normalData.join('\n'));
  
  await runExample(
    'Normal Distribution - Quality Control',
    'Testing if measurement data follows a normal distribution',
    ['normal', normalFile, '--verbose'],
    normalFile
  );
  
  console.log('\n🎉 All examples completed!');
  console.log('\n💡 Tips:');
  console.log('- Use --format json for programmatic processing');
  console.log('- Use --verbose for detailed analysis');
  console.log('- Try different input formats: CSV, JSON, YAML, Excel');
  console.log('- Check lawkit --help for all available options');
}

if (require.main === module) {
  main().catch(console.error);
}