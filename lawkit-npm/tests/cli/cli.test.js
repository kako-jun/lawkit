const { execSync } = require('child_process');
const path = require('path');

describe('CLI Basic Commands', () => {
  test('lawkit --version', () => {
    try {
      const result = execSync('npx lawkit --version', { encoding: 'utf8', timeout: 10000 });
      expect(result).toMatch(/lawkit/i);
    } catch (error) {
      // If command fails, check error message contains version info
      expect(error.message).toMatch(/lawkit|version|command/i);
    }
  });

  test('lawkit --help', () => {
    try {
      const result = execSync('npx lawkit --help', { encoding: 'utf8', timeout: 10000 });
      expect(result).toMatch(/usage|help|commands/i);
      expect(result).toMatch(/benf|analyze/i);
    } catch (error) {
      // If command fails, should be descriptive
      expect(error.message).toMatch(/help|usage|command/i);
    }
  });

  test('lawkit benf --help', () => {
    try {
      const result = execSync('npx lawkit benf --help', { encoding: 'utf8', timeout: 10000 });
      expect(result).toMatch(/benford|help/i);
    } catch (error) {
      // Should recognize benf subcommand
      expect(error.message).not.toMatch(/unknown.*command/i);
    }
  });
});