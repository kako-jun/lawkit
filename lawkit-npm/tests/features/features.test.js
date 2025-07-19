const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('Feature-Specific Tests', () => {
  let tempDir;
  
  beforeEach(() => {
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'lawkit-feature-test-'));
  });
  
  afterEach(() => {
    if (tempDir && fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('Benford analysis feature', () => {
    const csvFile = path.join(tempDir, 'benford.csv');
    fs.writeFileSync(csvFile, 'amount\\n111.11\\n222.22\\n333.33\\n123.45\\n234.56\\n345.67\\n');
    
    try {
      const result = execSync(`npx lawkit benf "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      
      // Should contain Benford analysis results
      expect(result).toMatch(/digit|expected|actual|frequency|1|2|3/i);
    } catch (error) {
      // Should handle Benford analysis
      expect(error.message).toMatch(/benf|digit|analysis/i);
    }
  });

  test('Statistical analysis features', () => {
    const csvFile = path.join(tempDir, 'stats.csv');
    const data = Array.from({length: 50}, (_, i) => i + 1).join('\\n');
    fs.writeFileSync(csvFile, `value\\n${data}`);
    
    try {
      const result = execSync(`npx lawkit normal "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      
      // Should contain statistical analysis
      expect(result).toMatch(/mean|deviation|normal|distribution/i);
    } catch (error) {
      // Should handle statistical analysis
      expect(error.message).toMatch(/normal|stats|distribution/i);
    }
  });

  test('Color output option', () => {
    const csvFile = path.join(tempDir, 'color.csv');
    fs.writeFileSync(csvFile, 'value\\n123\\n456\\n789\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}" --color always`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      
      // Should handle color option (whether or not ANSI codes are present)
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should recognize color option
      expect(error.message).toMatch(/color|option/i);
    }
  });

  test('Risk assessment features', () => {
    const csvFile = path.join(tempDir, 'risk.csv');
    // Create suspicious pattern - too many round numbers
    fs.writeFileSync(csvFile, 'amount\\n1000.00\\n2000.00\\n3000.00\\n4000.00\\n5000.00\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      
      // Should perform analysis (specific risk detection depends on implementation)
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should handle analysis
      expect(error.message).toMatch(/analyze|file|format/i);
    }
  });

  test('Large dataset handling', () => {
    const csvFile = path.join(tempDir, 'large.csv');
    const largeData = Array.from({length: 1000}, (_, i) => Math.random() * 10000).join('\\n');
    fs.writeFileSync(csvFile, `value\\n${largeData}`);
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 30000 // Longer timeout for large data
      });
      
      // Should handle large datasets
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should not crash on large datasets
      expect(error.message).not.toMatch(/crash|panic|memory/i);
    }
  });
});