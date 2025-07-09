const { runLawkit } = require('lawkit-js');
const fs = require('fs');

// Test data
const testData = `amount,category
123,A
456,B
789,A
321,C
654,B
987,A`;

// Write test data to file
fs.writeFileSync('test-data.csv', testData);

async function testNpmPackage() {
    console.log('🧪 Testing npm package lawkit-js...');
    
    try {
        // Test 1: Version
        console.log('\n1. Testing version...');
        const versionResult = await runLawkit(['--version']);
        console.log('Version:', versionResult.stdout.trim());
        
        // Test 2: Help
        console.log('\n2. Testing help...');
        const helpResult = await runLawkit(['--help']);
        console.log('Help available:', helpResult.stdout.includes('USAGE'));
        
        // Test 3: Benford analysis
        console.log('\n3. Testing Benford analysis...');
        const benfResult = await runLawkit(['benf', 'test-data.csv', '--format', 'json']);
        console.log('Benford result code:', benfResult.code);
        
        if (benfResult.code === 0) {
            const jsonData = JSON.parse(benfResult.stdout);
            console.log('Risk level:', jsonData.risk_level);
            console.log('P-value:', jsonData.p_value);
        }
        
        // Test 4: Pareto analysis
        console.log('\n4. Testing Pareto analysis...');
        const paretoResult = await runLawkit(['pareto', 'test-data.csv', '--format', 'json']);
        console.log('Pareto result code:', paretoResult.code);
        
        if (paretoResult.code === 0) {
            const jsonData = JSON.parse(paretoResult.stdout);
            console.log('Concentration 80/20:', jsonData.concentration_80_20);
        }
        
        // Test 5: Multi-law analysis
        console.log('\n5. Testing multi-law analysis...');
        const analyzeResult = await runLawkit(['analyze', 'test-data.csv', '--format', 'json', '--laws', 'benf,pareto']);
        console.log('Analysis result code:', analyzeResult.code);
        
        // Test 6: Data validation
        console.log('\n6. Testing data validation...');
        const validateResult = await runLawkit(['validate', 'test-data.csv', '--format', 'json']);
        console.log('Validation result code:', validateResult.code);
        
        // Test 7: CLI command (direct execution)
        console.log('\n7. Testing CLI command...');
        const { spawn } = require('child_process');
        const cliResult = spawn('npx', ['lawkit-js', 'list'], { stdio: 'pipe' });
        
        let output = '';
        cliResult.stdout.on('data', (data) => {
            output += data.toString();
        });
        
        cliResult.on('close', (code) => {
            console.log('CLI result code:', code);
            console.log('Available laws:', output.includes('benf'));
        });
        
        console.log('\n✅ All tests completed!');
        
    } catch (error) {
        console.error('❌ Error:', error.message);
    } finally {
        // Cleanup
        if (fs.existsSync('test-data.csv')) {
            fs.unlinkSync('test-data.csv');
        }
    }
}

testNpmPackage();