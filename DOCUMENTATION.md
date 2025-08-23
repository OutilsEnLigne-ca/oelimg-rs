# oelimg-rs - Claude Developer Documentation

**A comprehensive guide for Claude instances to implement high-performance image processing using the oelimg-rs WebAssembly library.**

## 📋 Quick Reference

**Package:** `oelimg-rs`  
**Version:** 0.1.0  
**Type:** WebAssembly image processing library  
**Language:** Rust → WASM  
**Environments:** Browser, Node.js, Next.js  
**License:** MIT  

## 🎯 What This Library Does

oelimg-rs provides blazing-fast client-side and server-side image processing through WebAssembly. It offers:

- **Color Adjustments:** HSL, brightness/contrast, temperature, vibrance
- **Artistic Filters:** Blur, sharpen, sepia, vintage effects
- **Transform Operations:** Resize, crop, rotate, flip
- **Correction Tools:** Auto-levels, exposure control, shadow/highlight recovery
- **Instagram Filters:** Pre-built popular filter combinations
- **Format Support:** PNG, JPEG, WebP input/output

## 🚀 Installation & Basic Setup

### npm Installation
```bash
npm install oelimg-rs
```

### WebAssembly Initialization
**CRITICAL:** Always initialize the WASM module before using any functions:

```javascript
import init from 'oelimg-rs';

// Initialize once before using any other functions
await init();

// Now you can use the library
import { WasmImageProcessor } from 'oelimg-rs';
```

## 🔧 Core API Reference

### WasmImageProcessor Class

The main class for all image processing operations.

#### Constructor & Creation
```javascript
// Create from image bytes (recommended)
const processor = WasmImageProcessor.from_bytes(imageBytes);

// Direct constructor (rarely used)
const processor = new WasmImageProcessor(width, height, rgbaData);
```

#### Properties
- `width: number` - Image width in pixels
- `height: number` - Image height in pixels  
- `data: Uint8Array` - Raw RGBA pixel data

#### Export Methods
```javascript
processor.to_png_bytes(): Uint8Array
processor.to_jpeg_bytes(quality: number): Uint8Array  // quality: 0-100
processor.to_webp_bytes(): Uint8Array
```

## 🎨 Filter Operations

### Color Adjustments

#### HSL Adjustment
```javascript
processor.adjust_hsl(hue, saturation, lightness);
// hue: -180 to 180 degrees
// saturation: 0 to 2 (1 = no change)
// lightness: 0 to 2 (1 = no change)

// Example: Warmer, more saturated
processor.adjust_hsl(15, 1.3, 1.05);
```

#### Brightness & Contrast
```javascript
processor.adjust_brightness_contrast(brightness, contrast);
// brightness: -1 to 1 (0 = no change)
// contrast: 0 to 2 (1 = no change)

// Example: Brighter with more contrast
processor.adjust_brightness_contrast(0.15, 1.25);
```

#### Color Temperature
```javascript
processor.adjust_color_temperature(kelvin, strength);
// kelvin: 1000-40000 (6500 = daylight)
// strength: 0-1 (optional, default 1.0)

// Common values:
// 2700K - Warm incandescent
// 3200K - Tungsten
// 5600K - Daylight balanced
// 9000K - Cool shade
processor.adjust_color_temperature(3200, 0.8);
```

#### Vibrance (Smart Saturation)
```javascript
processor.adjust_vibrance(vibrance);
// vibrance: -100 to 100 (0 = no change)

// Enhances colors while protecting skin tones
processor.adjust_vibrance(25);
```

### Transform Operations

#### Cropping
```javascript
processor.crop(x, y, width, height);
// All values in pixels

processor.crop(100, 50, 800, 600);
```

#### Resizing
```javascript
// Exact resize (may distort)
processor.resize(width, height, algorithm);

// Fit within bounds (maintains aspect ratio)
processor.resize_fit(maxWidth, maxHeight, algorithm);

// Algorithms: "Nearest", "Bilinear", "CatmullRom", "Mitchell", "Lanczos3"
processor.resize_fit(1200, 800, "Lanczos3");
```

#### Rotation & Flipping
```javascript
// Rotate by angle (degrees)
processor.rotate(angleDegrees, backgroundColor);
// backgroundColor: [R,G,B,A] array (optional)

processor.rotate(45, [255, 255, 255, 255]); // White background

// Flip operations
processor.flip_horizontal();
processor.flip_vertical();
```

### Artistic Filters

#### Blur Effects
```javascript
processor.gaussian_blur(radius);
// radius: 0 to 100

// Light blur: 1.0
// Medium blur: 3.0  
// Heavy blur: 8.0
processor.gaussian_blur(3.0);
```

#### Sharpening
```javascript
processor.sharpen(strength);
// strength: 0 to 10

// Light: 0.5, Medium: 1.0, Strong: 2.0
processor.sharpen(1.5);
```

#### Sepia Tone
```javascript
processor.sepia(intensity);
// intensity: 0 to 1

processor.sepia(0.8);
```

#### Vintage Film Effect
```javascript
processor.vintage(warmth, vignette, grain, fade);
// warmth: 0-2 (1=neutral)
// vignette: 0-1 (edge darkening)  
// grain: 0-1 (film grain)
// fade: 0-1 (film fade)

// Classic vintage look
processor.vintage(1.3, 0.4, 0.2, 0.1);
```

### Correction Tools

#### Auto Levels
```javascript
processor.auto_levels(); // Automatic histogram-based correction
```

#### Manual Levels
```javascript
processor.adjust_levels(shadows, gamma, highlights);
// shadows: 0-1, gamma: 0.1-3.0, highlights: 0-1

processor.adjust_levels(0.1, 0.9, 0.95);
```

#### Exposure Control
```javascript
processor.adjust_exposure(exposure, highlights, shadows);
// exposure: -5 to 5 (stops)
// highlights: -100 to 0 (recovery)
// shadows: 0 to 100 (lift)

processor.adjust_exposure(0.5, -30, 20);
```

## 🛠️ Utility Functions

### Instagram-Style Filters
```javascript
import { apply_instagram_filter } from 'oelimg-rs';

const filteredBytes = apply_instagram_filter(imageBytes, filterName);

// Available filters:
// "clarendon" - High contrast with lifted shadows
// "gingham" - Soft, dreamy look  
// "moon" - Desaturated, cool tone
// "lark" - Bright and vibrant
// "reyes" - Vintage film look
```

### Thumbnail Generation
```javascript
import { create_thumbnail } from 'oelimg-rs';

const thumbnailBytes = create_thumbnail(imageBytes, size, cropToSquare);
// size: thumbnail size in pixels
// cropToSquare: true for 1:1 aspect ratio, false to maintain original
```

### Version & Format Info
```javascript
import { get_version, get_supported_formats } from 'oelimg-rs';

const version = get_version();
const formats = get_supported_formats(); // ["png", "jpeg", "webp"]
```

## 💻 Usage Patterns

### Pattern 1: Basic Image Processing
```javascript
import init, { WasmImageProcessor } from 'oelimg-rs';

async function processImage(file) {
  // Initialize WASM
  await init();
  
  // Load image
  const arrayBuffer = await file.arrayBuffer();
  const processor = WasmImageProcessor.from_bytes(new Uint8Array(arrayBuffer));
  
  // Apply filters
  processor.adjust_brightness_contrast(0.1, 1.2);
  processor.gaussian_blur(2.0);
  processor.resize_fit(800, 600);
  
  // Export result
  return processor.to_png_bytes();
}
```

### Pattern 2: Filter Chain
```javascript
function applyFilterChain(processor, filters) {
  for (const filter of filters) {
    switch (filter.type) {
      case 'blur':
        processor.gaussian_blur(filter.radius);
        break;
      case 'brightness':
        processor.adjust_brightness_contrast(filter.value, 1.0);
        break;
      case 'hsl':
        processor.adjust_hsl(filter.hue, filter.saturation, filter.lightness);
        break;
      // Add more cases as needed
    }
  }
}
```

### Pattern 3: Error Handling
```javascript
async function safeImageProcess(imageBytes) {
  try {
    await init();
    const processor = WasmImageProcessor.from_bytes(imageBytes);
    
    // Apply operations
    processor.gaussian_blur(3.0);
    
    return processor.to_png_bytes();
  } catch (error) {
    // Common errors:
    // - Invalid image format
    // - Image too large (>64MP or >8192px per side)  
    // - Memory allocation failure
    // - Invalid parameter ranges
    console.error('Image processing failed:', error);
    return null;
  }
}
```

## 🌐 Integration Examples

### Next.js Client Component
```typescript
'use client';
import { useEffect, useState } from 'react';
import init, { WasmImageProcessor } from 'oelimg-rs';

export default function ImageEditor() {
  const [isReady, setIsReady] = useState(false);
  const [processor, setProcessor] = useState<WasmImageProcessor | null>(null);

  useEffect(() => {
    const initWasm = async () => {
      await init();
      setIsReady(true);
    };
    initWasm();
  }, []);

  const handleFileLoad = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file && isReady) {
      const buffer = await file.arrayBuffer();
      const newProcessor = WasmImageProcessor.from_bytes(new Uint8Array(buffer));
      setProcessor(newProcessor);
    }
  };

  const applyBlur = () => {
    if (processor) {
      processor.gaussian_blur(3.0);
      // Trigger re-render or update preview
    }
  };

  if (!isReady) return <div>Loading WebAssembly...</div>;

  return (
    <div>
      <input type="file" accept="image/*" onChange={handleFileLoad} />
      {processor && (
        <button onClick={applyBlur}>Apply Blur</button>
      )}
    </div>
  );
}
```

### Next.js API Route
```typescript
// app/api/process-image/route.ts
import { NextRequest, NextResponse } from 'next/server';
import init, { WasmImageProcessor } from 'oelimg-rs';

// Initialize once at module level
await init();

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData();
    const file = formData.get('image') as File;
    
    const arrayBuffer = await file.arrayBuffer();
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(arrayBuffer));
    
    // Apply processing
    processor.gaussian_blur(2.0);
    processor.adjust_brightness_contrast(0.1, 1.2);
    processor.resize_fit(1200, 800);
    
    const resultBytes = processor.to_webp_bytes();
    
    return new NextResponse(resultBytes, {
      headers: {
        'Content-Type': 'image/webp',
        'Content-Length': resultBytes.length.toString(),
      },
    });
  } catch (error) {
    return NextResponse.json(
      { error: error.message },
      { status: 500 }
    );
  }
}
```

### Express.js Server
```javascript
import express from 'express';
import init, { WasmImageProcessor } from 'oelimg-rs';

const app = express();

// Initialize WASM once on startup
await init();

app.post('/process-image', express.raw({ type: 'image/*', limit: '10mb' }), (req, res) => {
  try {
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(req.body));
    
    // Apply filters based on query params
    if (req.query.blur) {
      processor.gaussian_blur(parseFloat(req.query.blur));
    }
    
    if (req.query.brightness) {
      processor.adjust_brightness_contrast(parseFloat(req.query.brightness), 1.0);
    }
    
    const result = processor.to_png_bytes();
    res.setHeader('Content-Type', 'image/png');
    res.send(Buffer.from(result));
    
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});
```

### React Hook Pattern
```typescript
import { useCallback, useEffect, useState } from 'react';
import init, { WasmImageProcessor } from 'oelimg-rs';

export function useImageProcessor() {
  const [processor, setProcessor] = useState<WasmImageProcessor | null>(null);
  const [isReady, setIsReady] = useState(false);
  
  useEffect(() => {
    const initWasm = async () => {
      await init();
      setIsReady(true);
    };
    initWasm();
  }, []);
  
  const loadImage = useCallback(async (file: File) => {
    if (!isReady) return;
    
    const buffer = await file.arrayBuffer();
    const newProcessor = WasmImageProcessor.from_bytes(new Uint8Array(buffer));
    setProcessor(newProcessor);
  }, [isReady]);
  
  const applyFilter = useCallback((filterFn: (p: WasmImageProcessor) => void) => {
    if (processor) {
      filterFn(processor);
      // Could trigger state update to re-render
    }
  }, [processor]);
  
  return { processor, isReady, loadImage, applyFilter };
}
```

## ⚠️ Error Handling Best Practices

### 1. Always Wrap in Try-Catch
```javascript
try {
  await init();
  const processor = WasmImageProcessor.from_bytes(imageBytes);
  processor.gaussian_blur(3.0);
  return processor.to_png_bytes();
} catch (error) {
  console.error('Processing failed:', error);
  return null;
}
```

### 2. Validate Input Parameters
```javascript
function safeGaussianBlur(processor, radius) {
  if (radius < 0 || radius > 100) {
    throw new Error('Blur radius must be between 0 and 100');
  }
  processor.gaussian_blur(radius);
}
```

### 3. Check Image Dimensions
```javascript
function validateImageSize(processor) {
  const maxDimension = 8192;
  const maxPixels = 64 * 1024 * 1024; // 64MP
  
  if (processor.width > maxDimension || processor.height > maxDimension) {
    throw new Error('Image dimension exceeds 8192px limit');
  }
  
  if (processor.width * processor.height > maxPixels) {
    throw new Error('Image exceeds 64MP limit');
  }
}
```

## 🚀 Performance Guidelines

### 1. Reuse Processors
```javascript
// Good: Apply multiple operations to one processor
processor.gaussian_blur(2.0);
processor.adjust_brightness_contrast(0.1, 1.2);
processor.resize_fit(800, 600);

// Avoid: Creating new processors for each operation
```

### 2. Choose Appropriate Resize Algorithms
```javascript
// For speed: "Bilinear"
processor.resize_fit(800, 600, "Bilinear");

// For quality: "Lanczos3" (slower but better)
processor.resize_fit(800, 600, "Lanczos3");
```

### 3. Resize Large Images Early
```javascript
// Resize before heavy filtering for better performance
if (processor.width > 2048 || processor.height > 2048) {
  processor.resize_fit(2048, 2048);
}

// Then apply filters
processor.gaussian_blur(5.0);
processor.vintage(1.3, 0.4, 0.2, 0.1);
```

### 4. Use WebP for Smaller Output
```javascript
// WebP typically produces smaller files than PNG
const webpBytes = processor.to_webp_bytes();
```

## 🔍 Troubleshooting

### Common Issues & Solutions

#### WASM Module Not Initialized
**Error:** Functions throw "WASM not initialized" errors  
**Solution:** Always call `await init()` before using any functions

```javascript
import init from 'oelimg-rs';
await init(); // This line is essential
```

#### Memory Issues with Large Images
**Error:** Out of memory or allocation failures  
**Solution:** Resize images before processing

```javascript
// Resize large images first
if (processor.width * processor.height > 4000000) { // 4MP threshold
  processor.resize_fit(2048, 2048);
}
```

#### Invalid Parameter Ranges
**Error:** WebAssembly throws parameter validation errors  
**Solution:** Check parameter ranges in documentation

```javascript
// Wrong: radius outside 0-100 range
processor.gaussian_blur(150); // Error!

// Correct: clamp values
const radius = Math.max(0, Math.min(100, inputRadius));
processor.gaussian_blur(radius);
```

#### SSR Issues in Next.js
**Error:** WebAssembly fails during server-side rendering  
**Solution:** Use dynamic imports or client-only components

```javascript
// Use dynamic import in useEffect
useEffect(() => {
  const loadWasm = async () => {
    const { default: init } = await import('oelimg-rs');
    await init();
  };
  loadWasm();
}, []);
```

## 📊 Parameter Ranges Quick Reference

| Operation | Parameter | Range | Default | Notes |
|-----------|-----------|--------|---------|-------|
| HSL | hue | -180 to 180 | 0 | Degrees |
| HSL | saturation | 0 to 2 | 1 | Multiplier |  
| HSL | lightness | 0 to 2 | 1 | Multiplier |
| Brightness | brightness | -1 to 1 | 0 | Additive |
| Contrast | contrast | 0 to 2 | 1 | Multiplier |
| Temperature | kelvin | 1000 to 40000 | 6500 | Color temp |
| Vibrance | vibrance | -100 to 100 | 0 | Percentage |
| Blur | radius | 0 to 100 | 0 | Pixels |
| Sharpen | strength | 0 to 10 | 0 | Multiplier |
| Sepia | intensity | 0 to 1 | 0 | Blend |
| Exposure | exposure | -5 to 5 | 0 | Stops |
| Levels | gamma | 0.1 to 3 | 1 | Curve |

## 🎯 Common Use Cases

### 1. Photo Enhancement Pipeline
```javascript
function enhancePhoto(processor) {
  processor.auto_levels();           // Auto-correct exposure
  processor.adjust_vibrance(15);     // Boost colors
  processor.sharpen(0.8);           // Add sharpness
  processor.adjust_brightness_contrast(0.05, 1.1); // Fine-tune
}
```

### 2. Social Media Optimization
```javascript
function optimizeForSocial(processor) {
  processor.resize_fit(1200, 1200);  // Instagram-friendly size
  processor.adjust_color_temperature(6000, 0.7); // Slightly cool
  processor.adjust_vibrance(20);     // Pop the colors
  return processor.to_webp_bytes();  // Efficient format
}
```

### 3. Thumbnail Generation
```javascript
import { create_thumbnail } from 'oelimg-rs';

function generateThumbnails(imageBytes) {
  return {
    square: create_thumbnail(imageBytes, 150, true),    // Square crop
    landscape: create_thumbnail(imageBytes, 300, false)  // Maintain ratio
  };
}
```

---

## 📝 Summary for Claude Instances

This library is production-ready, well-documented, and provides comprehensive image processing capabilities. Key implementation points:

1. **Always initialize WASM first** with `await init()`
2. **Use WasmImageProcessor.from_bytes()** to load images
3. **Chain operations** on the same processor for efficiency
4. **Handle errors** with try-catch blocks
5. **Validate parameters** within documented ranges
6. **Optimize large images** by resizing early in the pipeline

The library excels at client-side processing, reducing server load while providing professional-grade image manipulation capabilities.