# Publishing lawkit-js to NPM

This document contains instructions for publishing the lawkit-js package to NPM.

## Prerequisites

1. Ensure you have an NPM account and are logged in:
   ```bash
   npm login
   ```

2. Verify your authentication:
   ```bash
   npm whoami
   ```

## Pre-publish Checklist

1. **Version Check**: Ensure the version in `package.json` matches the lawkit release version
2. **Binary Availability**: Confirm that the GitHub release exists with the required binaries
3. **Test Installation**: Test the package locally:
   ```bash
   npm pack
   npm install lawkit-js-2.1.0.tgz
   ```

## Publishing Steps

### 1. Test the Package Locally

```bash
# Install dependencies (if any)
npm install

# Test the download script
node scripts/download-binary.js

# Test the package
npm test

# Run examples
npm run examples
```

### 2. Publish to NPM

```bash
# Publish to NPM
npm publish
```

### 3. Verify Publication

```bash
# Check if package is available
npm view lawkit-js

# Test installation from NPM
npm install -g lawkit-js

# Test the installed package
lawkit --help
```

## Version Management

When updating the package:

1. Update version in `package.json`
2. Update `LAWKIT_VERSION` in `scripts/download-binary.js`
3. Ensure corresponding GitHub release exists
4. Test thoroughly before publishing

## Troubleshooting

### Binary Download Issues

If users report binary download issues:

1. Check if the GitHub release exists
2. Verify binary naming convention matches `getPlatform()` function
3. Test download URLs manually

### Platform Support

Currently supported platforms:
- Linux x86_64 and aarch64
- macOS x86_64 and aarch64 (Apple Silicon)
- Windows x86_64

To add new platforms:
1. Update `getPlatform()` function in download script
2. Add platform to `package.json` os/cpu fields
3. Ensure binaries are available in GitHub releases

## NPM Package Structure

```
lawkit-js/
├── package.json          # NPM package configuration
├── index.js             # Main wrapper script
├── README.md            # Package documentation
├── scripts/
│   └── download-binary.js # Binary download script
├── bin/                 # Binary directory (populated at install)
├── test.js              # Test script
├── examples.js          # Usage examples
├── .gitignore           # Git ignore rules
├── .npmignore           # NPM ignore rules
└── publish.md           # This file
```

## Notes

- The `bin/` directory is ignored in git but included in NPM package
- Binaries are downloaded automatically during `npm install`
- The package works both as a CLI tool and as a Node.js library
- All platforms are supported through automatic binary detection