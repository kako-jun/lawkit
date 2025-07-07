#!/usr/bin/env node

const { runLawkit } = require('./index.js');

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
    
    console.log('\n🎉 All tests completed!');
    
  } catch (error) {
    console.error('❌ Test failed with error:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  testLawkit();
}