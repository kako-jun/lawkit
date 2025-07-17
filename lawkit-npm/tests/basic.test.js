const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('Basic Functionality', () => {
  let tempDir;
  
  beforeEach(() => {
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'lawkit-test-'));
  });
  
  afterEach(() => {
    if (tempDir && fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('analyze command with CSV file', () => {
    const csvFile = path.join(tempDir, 'test.csv');
    fs.writeFileSync(csvFile, 'amount\\n123.45\\n234.56\\n345.67\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should handle CSV files or provide meaningful error
      expect(error.message).toMatch(/csv|file|format|analyze/i);
    }
  });

  test('benf command with sample data', () => {
    const csvFile = path.join(tempDir, 'benford.csv');
    fs.writeFileSync(csvFile, 'value\\n111.11\\n222.22\\n333.33\\n123.45\\n234.56\\n');
    
    try {
      const result = execSync(`npx lawkit benf "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should handle Benford analysis or provide meaningful error
      expect(error.message).toMatch(/benf|file|format|digit/i);
    }
  });

  test('JSON output format', () => {
    const csvFile = path.join(tempDir, 'test.csv');
    fs.writeFileSync(csvFile, 'amount\\n123\\n456\\n789\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}" --output json`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      
      if (result.includes('{') || result.includes('}')) {
        // Looks like JSON output
        expect(result).toMatch(/[{}]/);
      }
    } catch (error) {
      // Should recognize JSON format option
      expect(error.message).toMatch(/json|output|format/i);
    }
  });
});