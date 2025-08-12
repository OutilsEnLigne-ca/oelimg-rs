#!/bin/bash

# Build script for oelimg-rs WebAssembly package

set -e

echo "🦀 Building oelimg-rs WebAssembly package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it with:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg/
rm -rf target/

# Build for different targets
echo "🔨 Building for web target..."
wasm-pack build --target web --out-dir pkg/web --out-name oelimg_rs --release

echo "🔨 Building for Node.js target..."
wasm-pack build --target nodejs --out-dir pkg/nodejs --out-name oelimg_rs --release

echo "🔨 Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg --out-name oelimg_rs --release

# Generate TypeScript definitions
echo "📝 Copying TypeScript definitions..."
cp pkg/oelimg_rs.d.ts pkg/index.d.ts

# Create package info
echo "📦 Creating package files..."
cat > pkg/README.md << 'EOF'
# oelimg-rs

WebAssembly image processing library with comprehensive filters.

This package provides high-performance image processing capabilities compiled from Rust to WebAssembly.

## Installation

```bash
npm install oelimg-rs
```

## Usage

```javascript
import init, { WasmImageProcessor } from 'oelimg-rs';

async function processImage(imageBytes) {
  await init();
  
  const processor = WasmImageProcessor.from_bytes(imageBytes);
  
  // Apply filters
  processor.gaussian_blur(2.0);
  processor.adjust_brightness_contrast(0.1, 1.2);
  processor.adjust_vibrance(15.0);
  
  // Get processed image
  const processedBytes = processor.to_png_bytes();
  return processedBytes;
}
```

See the main repository README for complete documentation.
EOF

# Create usage examples
echo "📖 Creating usage examples..."
mkdir -p pkg/examples

cat > pkg/examples/basic.js << 'EOF'
import init, { WasmImageProcessor } from '../oelimg_rs.js';

async function example() {
  // Initialize WebAssembly module
  await init();
  
  // Load image from bytes (you would get this from a file input or fetch)
  const imageBytes = new Uint8Array(/* your image data */);
  const processor = WasmImageProcessor.from_bytes(imageBytes);
  
  console.log(`Image size: ${processor.width}x${processor.height}`);
  
  // Apply some filters
  processor.gaussian_blur(2.0);
  processor.adjust_hsl(10.0, 1.2, 1.0); // Hue +10°, Saturation +20%, no lightness change
  processor.resize_fit(800, 600);
  
  // Export as PNG
  const processedBytes = processor.to_png_bytes();
  
  return processedBytes;
}

export { example };
EOF

cat > pkg/examples/filters.js << 'EOF'
import init, { apply_instagram_filter, create_thumbnail } from '../oelimg_rs.js';

async function applyInstagramStyle() {
  await init();
  
  const imageBytes = new Uint8Array(/* your image data */);
  
  // Apply Instagram-style filters
  const clarendon = apply_instagram_filter(imageBytes, 'clarendon');
  const vintage = apply_instagram_filter(imageBytes, 'reyes');
  
  return { clarendon, vintage };
}

async function createThumbnail() {
  await init();
  
  const imageBytes = new Uint8Array(/* your image data */);
  const thumbnailBytes = create_thumbnail(imageBytes, 150, true); // 150px square
  
  return thumbnailBytes;
}

export { applyInstagramStyle, createThumbnail };
EOF

echo "✅ Build completed successfully!"
echo "📁 Output directory: pkg/"
echo "🌐 Web target: pkg/web/"
echo "🟢 Node.js target: pkg/nodejs/"
echo "📦 Bundler target: pkg/"

echo ""
echo "Next steps:"
echo "1. Test the package: npm test (if tests are available)"
echo "2. Use in your project: npm install ./pkg"
echo "3. Publish to npm: npm run publish:npm"