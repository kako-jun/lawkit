#!/usr/bin/env node

const { runLawkit } = require('./index.js');
const { benford, pareto, zipf, normal, poisson, analyze, validate, diagnose, generate, list, isLawkitAvailable } = require('./lib.js');

async function testLawkit() {
  console.log('🧪 Testing lawkit-js package...\n');
  
  try {
    // Test 1: Check if lawkit responds to --help
    console.log('Test 1: lawkit --help');
    const helpResult = await runLawkit(['--help']);
    
    if (helpResult.code === 0) {
      console.log('✅ Help command successful');
    } else {
      console.log('❌ Help command failed');
      console.log('Error:', helpResult.stderr);
    }
    
    // Test 2: Check if lawkit responds to --version
    console.log('\nTest 2: lawkit --version');
    const versionResult = await runLawkit(['--version']);
    
    if (versionResult.code === 0) {
      console.log('✅ Version command successful');
      console.log('Version:', versionResult.stdout.trim());
    } else {
      console.log('❌ Version command failed');
      console.log('Error:', versionResult.stderr);
    }
    
    // Test 3: Check if lawkit list works
    console.log('\nTest 3: lawkit list');
    const listResult = await runLawkit(['list']);
    
    if (listResult.code === 0) {
      console.log('✅ List command successful');
      console.log('Available laws:', listResult.stdout.trim());
    } else {
      console.log('❌ List command failed');
      console.log('Error:', listResult.stderr);
    }
    
    // Test 4: Generate sample data and analyze
    console.log('\nTest 4: Generate and analyze sample data');
    const generateResult = await runLawkit(['generate', 'benf', '--samples', '100', '--seed', '42']);
    
    if (generateResult.code === 0) {
      console.log('✅ Generate command successful');
      
      // Save to temp file and analyze
      const fs = require('fs');
      const path = require('path');
      const tempFile = path.join(__dirname, 'temp-data.txt');
      
      fs.writeFileSync(tempFile, generateResult.stdout);
      
      const analyzeResult = await runLawkit(['benf', tempFile, '--format', 'json']);
      
      if (analyzeResult.code === 0) {
        console.log('✅ Analysis command successful');
        const analysis = JSON.parse(analyzeResult.stdout);
        console.log('Risk Level:', analysis.risk_level);
        console.log('MAD:', analysis.mad);
      } else {
        console.log('❌ Analysis command failed');
        console.log('Error:', analyzeResult.stderr);
      }
      
      // Clean up
      fs.unlinkSync(tempFile);
    } else {
      console.log('❌ Generate command failed');
      console.log('Error:', generateResult.stderr);
    }
    
    // Test 5: Test lib.js API
    console.log('\nTest 5: Testing lib.js API');
    
    // Test isLawkitAvailable
    const isAvailable = await isLawkitAvailable();
    if (isAvailable) {
      console.log('✅ lawkit binary is available');
    } else {
      console.log('❌ lawkit binary is not available');
    }
    
    // Test Benford analysis with array data
    console.log('\nTest 6: Benford analysis with array data');
    const testData = [1, 10, 100, 1000, 2000, 3000];
    const benfResult = await benford(testData, { output: 'json' });
    console.log('✅ Benford analysis successful');
    console.log('Benford result risk level:', benfResult.risk_level);
    
    // Test list function
    console.log('\nTest 7: List available laws');
    const lawsList = await list();
    console.log('✅ List function successful');
    console.log('Available laws:', lawsList.substring(0, 100) + '...');
    
    console.log('\n🎉 All tests completed!');
    
  } catch (error) {
    console.error('❌ Test failed with error:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  testLawkit();
}