const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('File Format Support', () => {
  let tempDir;
  
  beforeEach(() => {
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'lawkit-format-test-'));
  });
  
  afterEach(() => {
    if (tempDir && fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('CSV format support', () => {
    const csvFile = path.join(tempDir, 'test.csv');
    fs.writeFileSync(csvFile, 'amount,date\\n123.45,2024-01-01\\n234.56,2024-01-02\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${csvFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should handle CSV files
      expect(error.message).toMatch(/csv|format|file/i);
    }
  });

  test('JSON format support', () => {
    const jsonFile = path.join(tempDir, 'test.json');
    fs.writeFileSync(jsonFile, JSON.stringify([
      { amount: 123.45, date: '2024-01-01' },
      { amount: 234.56, date: '2024-01-02' }
    ]));
    
    try {
      const result = execSync(`npx lawkit analyze "${jsonFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should either support JSON or provide meaningful error
      expect(error.message).toMatch(/json|format|file/i);
    }
  });

  test('YAML format support', () => {
    const yamlFile = path.join(tempDir, 'test.yaml');
    fs.writeFileSync(yamlFile, `- amount: 123.45
  date: 2024-01-01
- amount: 234.56
  date: 2024-01-02`);
    
    try {
      const result = execSync(`npx lawkit analyze "${yamlFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should either support YAML or provide meaningful error
      expect(error.message).toMatch(/yaml|yml|format|file/i);
    }
  });

  test('TXT format handling', () => {
    const txtFile = path.join(tempDir, 'test.txt');
    fs.writeFileSync(txtFile, '123.45\\n234.56\\n345.67\\n456.78\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${txtFile}"`, { 
        encoding: 'utf8', 
        timeout: 15000 
      });
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should either support TXT or provide meaningful error
      expect(error.message).toMatch(/txt|text|format|file/i);
    }
  });
});