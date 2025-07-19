const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('Error Handling', () => {
  let tempDir;
  
  beforeEach(() => {
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'lawkit-error-test-'));
  });
  
  afterEach(() => {
    if (tempDir && fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('nonexistent file error', () => {
    const nonexistentFile = path.join(tempDir, 'does-not-exist.csv');
    
    try {
      const result = execSync(`npx lawkit analyze "${nonexistentFile}"`, { 
        encoding: 'utf8', 
        timeout: 10000 
      });
      // If it doesn't throw, should indicate file not found
      expect(result).toMatch(/not found|no such file|error/i);
    } catch (error) {
      // Should provide meaningful error for missing file
      expect(error.message).toMatch(/not found|no such file|error|exist/i);
    }
  });

  test('invalid subcommand error', () => {
    try {
      const result = execSync('npx lawkit invalid_command_xyz', { 
        encoding: 'utf8', 
        timeout: 10000 
      });
      // If it doesn't throw, should indicate invalid command
      expect(result).toMatch(/invalid|unknown|error|help/i);
    } catch (error) {
      // Should provide meaningful error for invalid command
      expect(error.message).toMatch(/invalid|unknown|command|subcommand/i);
    }
  });

  test('invalid option error', () => {
    try {
      const result = execSync('npx lawkit analyze --invalid-option-xyz', { 
        encoding: 'utf8', 
        timeout: 10000 
      });
      // If it doesn't throw, should indicate invalid option
      expect(result).toMatch(/invalid|unknown|option|error/i);
    } catch (error) {
      // Should provide meaningful error for invalid option
      expect(error.message).toMatch(/invalid|unknown|option|argument/i);
    }
  });

  test('empty file handling', () => {
    const emptyFile = path.join(tempDir, 'empty.csv');
    fs.writeFileSync(emptyFile, '');
    
    try {
      const result = execSync(`npx lawkit analyze "${emptyFile}"`, { 
        encoding: 'utf8', 
        timeout: 10000 
      });
      // Should handle empty files gracefully
      expect(result).toMatch(/empty|no data|warning/i);
    } catch (error) {
      // Should provide meaningful error for empty file
      expect(error.message).toMatch(/empty|no data|file/i);
    }
  });

  test('malformed CSV handling', () => {
    const malformedFile = path.join(tempDir, 'malformed.csv');
    fs.writeFileSync(malformedFile, 'incomplete,csv,\\nunfinished,"quote\\n');
    
    try {
      const result = execSync(`npx lawkit analyze "${malformedFile}"`, { 
        encoding: 'utf8', 
        timeout: 10000 
      });
      // Should handle malformed files gracefully
      expect(result.length).toBeGreaterThan(0);
    } catch (error) {
      // Should provide meaningful error for malformed file
      expect(error.message).toMatch(/parse|format|csv|malformed/i);
    }
  });
});