# 🚀 oelimg-rs Development Guide

Complete guide for developers working with the oelimg-rs WebAssembly image processing library.

## 📚 Documentation Resources

### 1. **API Reference** (`API_REFERENCE.md`)
Complete backend developer reference with:
- Method signatures and parameters
- Parameter ranges and defaults
- Usage examples for each operation
- Backend integration patterns
- Error handling guide

### 2. **Rust Documentation** (rustdoc)
Generated Rust documentation with implementation details:

```bash
# Generate and view Rust docs
cargo doc --open --no-deps

# Or serve docs locally
python3 serve-docs.py
# Opens http://localhost:8080/oelimg_rs/
```

### 3. **TypeScript Definitions**
Auto-generated TypeScript definitions in `pkg/oelimg_rs.d.ts` provide:
- Complete type safety
- IDE autocompletion
- Parameter validation

## 🏗️ Architecture Overview

```
oelimg-rs/
├── src/
│   ├── core/           # Core utilities
│   │   ├── image_buffer.rs    # Image data management
│   │   ├── color_space.rs     # Color space conversions
│   │   └── histogram.rs       # Histogram analysis
│   ├── filters/        # Filter implementations
│   │   ├── color/      # HSL, brightness, temperature, vibrance
│   │   ├── artistic/   # Blur, sharpen, vintage, sepia
│   │   ├── correction/ # Levels, exposure, curves
│   │   └── transform/  # Crop, resize, rotate, flip
│   ├── utils/          # Utility functions
│   ├── wasm.rs         # WebAssembly interface
│   └── lib.rs          # Main library entry point
└── pkg/                # Generated WebAssembly output
    ├── oelimg_rs.js    # JavaScript bindings
    ├── oelimg_rs.wasm  # WebAssembly binary
    └── oelimg_rs.d.ts  # TypeScript definitions
```

## 🔧 Development Workflow

### Building the Package

```bash
# Build all targets (web, nodejs, bundler)
./build.sh

# Or build specific targets
npm run build           # Web target
npm run build:nodejs    # Node.js target
npm run build:bundler   # Bundler target

# Development build (faster compilation)
npm run dev
```

### Testing Changes

```bash
# Check compilation without full build
cargo check

# Run Rust tests
cargo test

# Generate documentation
cargo doc --no-deps

# Serve documentation locally
python3 serve-docs.py
```

### Adding New Filters

1. **Create filter struct** in appropriate module (e.g., `src/filters/artistic/new_filter.rs`)
2. **Implement ImageFilter trait**
3. **Add WASM bindings** in `src/wasm.rs`
4. **Update module exports** in `mod.rs` files
5. **Add TypeScript definitions** if needed
6. **Update API documentation**

Example filter structure:
```rust
use crate::filters::{ImageFilter, ImageData};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NewFilter {
    param1: f32,
    param2: f32,
}

#[wasm_bindgen]
impl NewFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(param1: f32, param2: f32) -> Self {
        Self { param1, param2 }
    }
}

impl ImageFilter for NewFilter {
    fn apply(&self, image: &mut ImageData) -> Result<(), JsValue> {
        // Filter implementation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "New Filter"
    }
}
```

## 🧪 Backend Integration Patterns

### Express.js Middleware Pattern

```javascript
import express from 'express';
import init, { WasmImageProcessor } from 'oelimg-rs';

const app = express();

// Initialize WASM once at startup
let wasmReady = false;
init().then(() => {
    wasmReady = true;
    console.log('WASM initialized');
});

// Middleware to ensure WASM is ready
const ensureWasmReady = (req, res, next) => {
    if (!wasmReady) {
        return res.status(503).json({ error: 'Service initializing, please retry' });
    }
    next();
};

// Image processing endpoint
app.post('/process', ensureWasmReady, express.raw({ type: 'image/*' }), (req, res) => {
    try {
        const processor = WasmImageProcessor.from_bytes(new Uint8Array(req.body));
        
        // Apply operations based on query parameters
        const operations = JSON.parse(req.query.operations || '[]');
        for (const op of operations) {
            applyOperation(processor, op);
        }
        
        const result = processor.to_png_bytes();
        res.setHeader('Content-Type', 'image/png');
        res.send(Buffer.from(result));
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});
```

### Next.js API Route Pattern

```typescript
// app/api/image/process/route.ts
import { NextRequest, NextResponse } from 'next/server';
import init, { WasmImageProcessor } from 'oelimg-rs';

// Initialize at module level
let initialized = false;
const initPromise = init().then(() => { initialized = true; });

export async function POST(request: NextRequest) {
    // Ensure WASM is initialized
    if (!initialized) await initPromise;
    
    try {
        const formData = await request.formData();
        const imageFile = formData.get('image') as File;
        const operations = JSON.parse(formData.get('operations') as string);
        
        const arrayBuffer = await imageFile.arrayBuffer();
        const processor = WasmImageProcessor.from_bytes(new Uint8Array(arrayBuffer));
        
        // Process operations
        for (const op of operations) {
            switch (op.type) {
                case 'resize':
                    processor.resize_fit(op.width, op.height);
                    break;
                case 'blur':
                    processor.gaussian_blur(op.radius);
                    break;
                // ... more operations
            }
        }
        
        const result = processor.to_png_bytes();
        return new NextResponse(result, {
            headers: { 'Content-Type': 'image/png' },
        });
    } catch (error) {
        return NextResponse.json(
            { error: error.message },
            { status: 400 }
        );
    }
}
```

### Operation Factory Pattern

```javascript
// operations.js - Centralized operation handling
export class ImageOperationFactory {
    static applyOperation(processor, operation) {
        switch (operation.type) {
            case 'hsl':
                return processor.adjust_hsl(
                    operation.hue || 0,
                    operation.saturation || 1,
                    operation.lightness || 1
                );
            
            case 'brightness-contrast':
                return processor.adjust_brightness_contrast(
                    operation.brightness || 0,
                    operation.contrast || 1
                );
            
            case 'color-temperature':
                return processor.adjust_color_temperature(
                    operation.kelvin || 6500,
                    operation.strength || 1
                );
            
            case 'crop':
                return processor.crop(
                    operation.x,
                    operation.y,
                    operation.width,
                    operation.height
                );
            
            case 'resize':
                if (operation.fit) {
                    return processor.resize_fit(operation.width, operation.height);
                } else {
                    return processor.resize(operation.width, operation.height);
                }
            
            case 'artistic':
                switch (operation.filter) {
                    case 'blur':
                        return processor.gaussian_blur(operation.radius || 1);
                    case 'sharpen':
                        return processor.sharpen(operation.strength || 1);
                    case 'sepia':
                        return processor.sepia(operation.intensity || 0.5);
                    case 'vintage':
                        return processor.vintage(
                            operation.warmth || 1.2,
                            operation.vignette || 0.3,
                            operation.grain || 0.1,
                            operation.fade || 0.1
                        );
                }
                break;
            
            default:
                throw new Error(`Unknown operation type: ${operation.type}`);
        }
    }
    
    static validateOperation(operation) {
        // Add operation validation logic
        const required = ['type'];
        for (const field of required) {
            if (!operation.hasOwnProperty(field)) {
                throw new Error(`Missing required field: ${field}`);
            }
        }
        
        // Type-specific validation
        switch (operation.type) {
            case 'crop':
                const cropRequired = ['x', 'y', 'width', 'height'];
                for (const field of cropRequired) {
                    if (typeof operation[field] !== 'number') {
                        throw new Error(`Crop operation requires numeric ${field}`);
                    }
                }
                break;
            // ... more validations
        }
        
        return true;
    }
}
```

## 🎯 Performance Optimization

### Memory Management
- **Reuse processors** for multiple operations on the same image
- **Release processors** explicitly when done (they implement Drop)
- **Use appropriate image sizes** - resize large images early in the pipeline

### Operation Ordering
1. **Geometric transforms first** (crop, resize, rotate)
2. **Color corrections** (levels, exposure, temperature)
3. **Creative adjustments** (HSL, vibrance, brightness/contrast)
4. **Artistic effects last** (blur, vintage, sepia)

### Batch Processing
```javascript
async function processBatch(images, operations) {
    // Process images in smaller batches to manage memory
    const BATCH_SIZE = 5;
    const results = [];
    
    for (let i = 0; i < images.length; i += BATCH_SIZE) {
        const batch = images.slice(i, i + BATCH_SIZE);
        const batchResults = await Promise.all(
            batch.map(async (image) => {
                const processor = WasmImageProcessor.from_bytes(image);
                
                for (const op of operations) {
                    ImageOperationFactory.applyOperation(processor, op);
                }
                
                return processor.to_png_bytes();
            })
        );
        
        results.push(...batchResults);
        
        // Optional: Add small delay to prevent blocking
        await new Promise(resolve => setTimeout(resolve, 10));
    }
    
    return results;
}
```

## 🔍 Debugging Tips

### Enable Console Logging
```javascript
// The panic hook is already set up for better error messages in console
import init from 'oelimg-rs';
await init();
// Any panics will now show detailed stack traces in browser console
```

### Parameter Validation
```javascript
function validateHSLParams(hue, saturation, lightness) {
    if (hue < -180 || hue > 180) {
        throw new Error(`Hue must be between -180 and 180, got ${hue}`);
    }
    if (saturation < 0 || saturation > 2) {
        throw new Error(`Saturation must be between 0 and 2, got ${saturation}`);
    }
    if (lightness < 0 || lightness > 2) {
        throw new Error(`Lightness must be between 0 and 2, got ${lightness}`);
    }
}
```

### Performance Monitoring
```javascript
function benchmarkOperation(processor, operationName, operationFn) {
    const start = performance.now();
    operationFn();
    const end = performance.now();
    console.log(`${operationName} took ${end - start}ms`);
}

// Usage
benchmarkOperation(processor, 'Gaussian Blur', () => {
    processor.gaussian_blur(3.0);
});
```

## 📋 Quick Reference

### Common Parameter Ranges
- **HSL hue**: -180 to 180 degrees
- **HSL saturation/lightness**: 0 to 2 (1 = no change)
- **Brightness**: -1 to 1 (0 = no change)
- **Contrast**: 0 to 2 (1 = no change)
- **Color temperature**: 1000 to 40000K (6500K = daylight)
- **Vibrance**: -100 to 100 (0 = no change)
- **Blur radius**: 0 to 100 pixels
- **Sharpen strength**: 0 to 10
- **Sepia intensity**: 0 to 1

### Error Handling Best Practices
```javascript
function safeImageProcessing(imageBytes, operations) {
    try {
        const processor = WasmImageProcessor.from_bytes(imageBytes);
        
        for (const op of operations) {
            try {
                ImageOperationFactory.applyOperation(processor, op);
            } catch (opError) {
                console.warn(`Failed to apply operation ${op.type}:`, opError);
                // Continue with other operations or fail fast based on requirements
            }
        }
        
        return processor.to_png_bytes();
    } catch (error) {
        console.error('Image processing failed:', error);
        throw new Error(`Image processing failed: ${error.message}`);
    }
}
```

This development guide should provide you with all the resources and patterns you need to effectively work with the oelimg-rs library in your backend applications! 🚀