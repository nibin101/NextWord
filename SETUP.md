# Setup Guide - AI Autocomplete System

This guide walks you through setting up the complete autocomplete system from scratch.

## Prerequisites

### Required Software
- **Rust** (version 1.70 or later)
  - Install from: https://rustup.rs/
  - Verify: `rustc --version`
- **Node.js** (version 18 or later)
  - Install from: https://nodejs.org/
  - Verify: `node --version`
- **VS Code** (version 1.109 or later)
  - Install from: https://code.visualstudio.com/

### System Requirements
- **RAM**: 4GB minimum (8GB recommended for large models)
- **Storage**: 1GB free space (depends on training corpus size)
- **OS**: Windows, macOS, or Linux

## Step-by-Step Setup

### 1. Clone and Navigate to Project
```bash
# If using git
git clone <repository-url>
cd autocomplete

# Or extract from archive and navigate to folder
```

### 2. Prepare Training Data

Create your training corpus:
```bash
# Create training data file
cd trainer
echo "Your training text goes here. Add as much text as possible for better predictions." > data.txt

# For better results, use a large text corpus
# Examples: books, articles, documentation, code comments
# Minimum recommended: 1MB of text
# Optimal: 10MB+ of domain-specific text
```

### 3. Train the Model

```bash
# Build the trainer
cargo build --release

# Run training (this may take several minutes)
cargo run --release

# Expected output:
# Opening data file...
# Building trigram counts...
# Processed 100000 lines
# Pruning rare entries...
# Flattening model...
# Saving model.bin ...
# Saving vocab.txt ...
# Done!
```

**Training Tips:**
- Larger datasets = better predictions
- Clean your text data (remove markup, special characters)
- Domain-specific training data works best
- Training time scales with corpus size

### 4. Copy Model Files

Move the trained model to the server directory:
```bash
# Copy generated files to server
cp model.bin ../autocomplete_server/
cp vocab.txt ../autocomplete_server/

# Verify files exist
ls -la ../autocomplete_server/model.bin
ls -la ../autocomplete_server/vocab.txt
```

### 5. Start the Autocomplete Server

```bash
cd ../autocomplete_server

# Build the server
cargo build --release

# Start server (runs on http://127.0.0.1:3000)
cargo run --release

# Expected output:
# Loading model...
# Loading vocabulary...  
# Rebuilding word_to_id map...
# Server running at http://127.0.0.1:3000
```

**Server Notes:**
- Server must be running for autocomplete to work
- Default port: 3000 (change in main.rs if needed)  
- Server loads entire model into memory for fast predictions
- Restart server if you retrain the model

### 6. Test the Server (Optional)

Test the API directly:
```bash
# Test prediction endpoint
curl -X POST http://127.0.0.1:3000/predict \
  -H "Content-Type: application/json" \
  -d '{"context": "the quick brown"}'

# Expected response:
# {"suggestions": ["fox", "dog", "cat", "horse", "rabbit"]}
```

### 7. Install VS Code Extension

```bash
cd ../autocomplete-client

# Install dependencies
npm install

# Build the extension
npm run compile

# Package extension (optional - for distribution)
npm run package
```

### 8. Load Extension in VS Code

**Method 1: Development Mode**
1. Open VS Code
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
3. Type "Developer: Reload Window" and press Enter
4. Go to Run and Debug view (`Ctrl+Shift+D`)
5. Select "Run Extension" and press F5
6. This opens a new VS Code window with your extension loaded

**Method 2: Install Packaged Extension**
1. In VS Code, press `Ctrl+Shift+P`
2. Type "Extensions: Install from VSIX"
3. Select the generated `.vsix` file (if you ran `npm run package`)

### 9. Test Autocomplete

1. Create a new file in VS Code
2. Type some text: "the quick "
3. Press space - you should see autocomplete suggestions
4. Select a suggestion or continue typing

## Troubleshooting

### Common Issues

**1. Server won't start**
```
Error: Failed to load model.bin
Solution: Ensure you've trained a model and copied files correctly
```

**2. No autocomplete suggestions**
```
Check:
- Is the server running on port 3000?
- Are you typing at least 2 words before space?
- Check VS Code Developer Console for errors
```

**3. Extension not loading**
```
Check:
- Is VS Code version 1.109 or later?
- Try reloading VS Code window
- Check for extension compilation errors
```

**4. Training takes too long**
```
Solutions:
- Use smaller training dataset for testing
- Ensure you're using --release mode
- Consider more powerful hardware
```

### Performance Tuning

**Large Models:**
- Increase system RAM if model loading fails
- Adjust `min_count` threshold in trainer for smaller models
- Consider splitting large corpora into chunks

**Slow Predictions:**
- Ensure server is built with `--release` flag
- Check server logs for performance warnings
- Verify model is fully loaded into memory

**Extension Responsiveness:**
- Adjust trigger characters in `extension.ts`
- Add request timeout handling
- Consider caching recent predictions

## Development Workflow

### Making Changes

**1. Update Training Data:**
```bash
cd trainer
# Edit data.txt with new content
cargo run --release
cp model.bin vocab.txt ../autocomplete_server/
# Restart server
```

**2. Modify Server Logic:**
```bash
cd autocomplete_server  
# Edit main.rs
cargo run --release
```

**3. Update Extension:**
```bash
cd autocomplete-client
# Edit src/extension.ts  
npm run compile
# Reload VS Code window
```

### Production Deployment

**Server Deployment:**
- Build with `cargo build --release`
- Copy binary and model files to production server
- Configure systemd/docker for auto-restart
- Set up reverse proxy (nginx) if needed

**Extension Distribution:**
- Package with `npm run package`
- Publish to VS Code Marketplace
- Or distribute `.vsix` file directly

## Next Steps

1. **Experiment with Training Data**: Try different text corpora
2. **Customize Triggers**: Modify extension trigger characters  
3. **Tune Model Parameters**: Adjust min_count, context window
4. **Monitor Performance**: Add logging and metrics
5. **Enhance UI**: Improve completion item presentation

For architecture details, see [README.md](README.md).
For requirements, see [requirements.txt](requirements.txt).