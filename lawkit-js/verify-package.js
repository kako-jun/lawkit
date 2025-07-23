#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function checkFile(filePath, description) {
  if (fs.existsSync(filePath)) {
    const stats = fs.statSync(filePath);
    console.log(`✅ ${description}: ${filePath} (${stats.size} bytes)`);
    return true;
  } else {
    console.log(`❌ ${description}: ${filePath} - MISSING`);
    return false;
  }
}

function checkPackageJson() {
  const packagePath = path.join(__dirname, 'package.json');
  if (!fs.existsSync(packagePath)) {
    console.log('❌ package.json not found');
    return false;
  }
  
  const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
  console.log(`✅ Package: ${pkg.name} v${pkg.version}`);
  console.log(`✅ Description: ${pkg.description}`);
  console.log(`✅ Keywords: ${pkg.keywords.join(', ')}`);
  console.log(`✅ License: ${pkg.license}`);
  console.log(`✅ Repository: ${pkg.repository.url}`);
  
  return true;
}

function main() {
  console.log('🔍 Verifying lawkit-js NPM package structure...\n');
  
  let allGood = true;
  
  // Check required files
  allGood &= checkFile('package.json', 'Package configuration');
  allGood &= checkFile('index.js', 'Main wrapper script');
  allGood &= checkFile('README.md', 'Package documentation');
  allGood &= checkFile('scripts/download-binary.js', 'Binary download script');
  allGood &= checkFile('.gitignore', 'Git ignore file');
  allGood &= checkFile('.npmignore', 'NPM ignore file');
  
  // Check optional files
  checkFile('test.js', 'Test script');
  checkFile('examples.js', 'Usage examples');
  checkFile('publish.md', 'Publishing guide');
  
  // Check directories
  if (fs.existsSync('bin')) {
    console.log('✅ bin/ directory exists');
  } else {
    console.log('⚠️  bin/ directory not found (will be created on install)');
  }
  
  if (fs.existsSync('scripts')) {
    console.log('✅ scripts/ directory exists');
  } else {
    console.log('❌ scripts/ directory not found');
    allGood = false;
  }
  
  console.log('\n📦 Package.json validation:');
  allGood &= checkPackageJson();
  
  console.log('\n🎯 Package files summary:');
  console.log('Essential files: package.json, index.js, README.md, download-binary.js');
  console.log('Support files: test.js, examples.js, .gitignore, .npmignore');
  console.log('Documentation: README.md, publish.md');
  
  if (allGood) {
    console.log('\n🎉 Package structure verification PASSED!');
    console.log('✅ Ready for NPM publishing');
  } else {
    console.log('\n❌ Package structure verification FAILED!');
    console.log('⚠️  Fix the issues above before publishing');
  }
  
  console.log('\n💡 Next steps:');
  console.log('1. Test locally: npm pack && npm install <package-file>');
  console.log('2. Verify functionality: npm test && npm run examples');
  console.log('3. Publish to NPM: npm publish');
  console.log('4. Test installation: npm install -g lawkit-js');
}

if (require.main === module) {
  main();
}