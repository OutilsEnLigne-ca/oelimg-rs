# oelimg-rs API Reference

Complete API documentation for backend developers using the oelimg-rs WebAssembly image processing library.

## 📋 Table of Contents

- [Initialization](#initialization)
- [Core Classes](#core-classes)
- [Color Adjustment Methods](#color-adjustment-methods)
- [Transform Operations](#transform-operations)
- [Artistic Filters](#artistic-filters)
- [Correction Tools](#correction-tools)
- [Utility Functions](#utility-functions)
- [Error Handling](#error-handling)
- [Type Definitions](#type-definitions)
- [Backend Integration Examples](#backend-integration-examples)

## 🚀 Initialization

### `init(): Promise<void>`
Initialize the WebAssembly module. Must be called before using any other functions.

```javascript
import init from 'oelimg-rs';
await init();
```

## 🏗️ Core Classes

### `WasmImageProcessor`

Main class for image processing operations.

#### Constructor
```javascript
new WasmImageProcessor(width: number, height: number, data: Uint8Array)
```

#### Static Methods
```javascript
WasmImageProcessor.from_bytes(bytes: Uint8Array): WasmImageProcessor
```

#### Properties
- `width: number` - Image width in pixels
- `height: number` - Image height in pixels  
- `data: Uint8Array` - Raw RGBA pixel data

#### Export Methods
```javascript
to_png_bytes(): Uint8Array
to_jpeg_bytes(quality: number): Uint8Array  // quality: 0-100
to_webp_bytes(): Uint8Array
```

## 🎨 Color Adjustment Methods

### `adjust_hsl(hue, saturation, lightness)`
Adjust hue, saturation, and lightness values.

**Parameters:**
- `hue: number` - Hue shift in degrees (-180 to 180)
- `saturation: number` - Saturation multiplier (0.0 to 2.0, 1.0 = no change)  
- `lightness: number` - Lightness multiplier (0.0 to 2.0, 1.0 = no change)

**Example:**
```javascript
processor.adjust_hsl(10, 1.2, 1.0); // +10° hue, +20% saturation, no lightness change
```

### `adjust_brightness_contrast(brightness, contrast)`
Adjust brightness and contrast independently.

**Parameters:**
- `brightness: number` - Brightness adjustment (-1.0 to 1.0, 0.0 = no change)
- `contrast: number` - Contrast multiplier (0.0 to 2.0, 1.0 = no change)

**Example:**
```javascript
processor.adjust_brightness_contrast(0.1, 1.3); // +10% brightness, +30% contrast
```

### `adjust_color_temperature(kelvin, strength?)`
Adjust color temperature for warm/cool color balance.

**Parameters:**
- `kelvin: number` - Color temperature (1000 to 40000, 6500 = daylight)
- `strength?: number` - Effect strength (0.0 to 1.0, default: 1.0)

**Common Values:**
- 2700K - Warm incandescent
- 3200K - Tungsten
- 5600K - Daylight
- 6500K - Standard daylight
- 9000K - Cool shade

**Example:**
```javascript
processor.adjust_color_temperature(3200, 0.8); // Tungsten warmth at 80% strength
```

### `adjust_vibrance(vibrance)`
Smart saturation adjustment that protects skin tones.

**Parameters:**
- `vibrance: number` - Vibrance adjustment (-100 to 100, 0 = no change)

**Example:**
```javascript
processor.adjust_vibrance(25); // +25% vibrance enhancement
```

## 🔄 Transform Operations

### `crop(x, y, width, height)`
Crop image to specified rectangle.

**Parameters:**
- `x: number` - Left offset in pixels
- `y: number` - Top offset in pixels
- `width: number` - Crop width in pixels
- `height: number` - Crop height in pixels

**Example:**
```javascript
processor.crop(100, 50, 400, 300); // Crop 400x300 area starting at (100,50)
```

### `resize(width, height, algorithm?)`
Resize image to exact dimensions.

**Parameters:**
- `width: number` - Target width in pixels
- `height: number` - Target height in pixels
- `algorithm?: ResizeAlgorithm` - Resize algorithm (default: Lanczos3)

**Algorithms:**
- `"Nearest"` - Fast, pixelated
- `"Bilinear"` - Smooth, good for photos
- `"CatmullRom"` - Sharp, good for graphics  
- `"Mitchell"` - Balanced quality/performance
- `"Lanczos3"` - Highest quality (default)

**Example:**
```javascript
processor.resize(800, 600, "Lanczos3"); // Resize to 800x600 with highest quality
```

### `resize_fit(maxWidth, maxHeight, algorithm?)`
Resize image to fit within bounds while maintaining aspect ratio.

**Parameters:**
- `maxWidth: number` - Maximum width
- `maxHeight: number` - Maximum height
- `algorithm?: ResizeAlgorithm` - Resize algorithm

**Example:**
```javascript
processor.resize_fit(1200, 800); // Fit within 1200x800, maintaining aspect ratio
```

### `rotate(angleDegrees, backgroundColor?)`
Rotate image by arbitrary angle.

**Parameters:**
- `angleDegrees: number` - Rotation angle in degrees
- `backgroundColor?: number[]` - Background fill color [R,G,B,A] (default: transparent)

**Example:**
```javascript
processor.rotate(45, [255, 255, 255, 255]); // 45° rotation with white background
```

### `flip_horizontal()` / `flip_vertical()`
Mirror image horizontally or vertically.

**Example:**
```javascript
processor.flip_horizontal(); // Mirror left-to-right
processor.flip_vertical();   // Mirror top-to-bottom
```

## 🎭 Artistic Filters

### `gaussian_blur(radius)`
Apply Gaussian blur effect.

**Parameters:**
- `radius: number` - Blur radius (0 to 100, 0 = no blur)

**Presets:**
- Light blur: 1.0
- Medium blur: 3.0
- Heavy blur: 8.0

**Example:**
```javascript
processor.gaussian_blur(3.0); // Medium blur effect
```

### `sharpen(strength)`
Sharpen image details.

**Parameters:**
- `strength: number` - Sharpening strength (0.0 to 10.0)

**Presets:**
- Light: 0.5
- Medium: 1.0
- Strong: 2.0

**Example:**
```javascript
processor.sharpen(1.5); // Enhanced sharpening
```

### `sepia(intensity)`
Apply sepia tone effect.

**Parameters:**
- `intensity: number` - Effect intensity (0.0 to 1.0)

**Example:**
```javascript
processor.sepia(0.8); // Strong sepia effect
```

### `vintage(warmth, vignette, grain, fade)`
Apply vintage film effect with multiple parameters.

**Parameters:**
- `warmth: number` - Color warmth (0.0 to 2.0, 1.0 = neutral)
- `vignette: number` - Edge darkening (0.0 to 1.0)
- `grain: number` - Film grain intensity (0.0 to 1.0)
- `fade: number` - Film fade effect (0.0 to 1.0)

**Presets:**
- Classic: `(1.3, 0.4, 0.2, 0.1)`
- Warm: `(1.6, 0.3, 0.15, 0.15)`
- Faded: `(1.1, 0.5, 0.1, 0.4)`

**Example:**
```javascript
processor.vintage(1.3, 0.4, 0.2, 0.1); // Classic vintage look
```

## 🔧 Correction Tools

### `auto_levels()`
Automatically adjust levels based on image histogram.

**Example:**
```javascript
processor.auto_levels(); // Automatic color correction
```

### `adjust_levels(shadows, gamma, highlights)`
Manual levels adjustment with shadow, gamma, and highlight controls.

**Parameters:**
- `shadows: number` - Shadow input level (0.0 to 1.0)
- `gamma: number` - Gamma correction (0.1 to 3.0, 1.0 = no change)
- `highlights: number` - Highlight input level (0.0 to 1.0)

**Example:**
```javascript
processor.adjust_levels(0.1, 0.9, 0.95); // Lift shadows, reduce gamma, preserve highlights
```

### `adjust_exposure(exposure, highlights, shadows)`
Professional exposure control with highlight and shadow recovery.

**Parameters:**
- `exposure: number` - Exposure adjustment in stops (-5.0 to 5.0)
- `highlights: number` - Highlight recovery (-100 to 0)
- `shadows: number` - Shadow lift (0 to 100)

**Example:**
```javascript
processor.adjust_exposure(0.5, -30, 20); // +0.5 stop exposure, recover highlights, lift shadows
```

## 🛠️ Utility Functions

### `load_image_from_bytes(bytes: Uint8Array): WasmImageProcessor`
Create processor from image bytes (any supported format).

### `create_thumbnail(bytes: Uint8Array, size: number, cropToSquare?: boolean): Uint8Array`
Generate thumbnail from image bytes.

**Parameters:**
- `bytes: Uint8Array` - Source image data
- `size: number` - Thumbnail size in pixels
- `cropToSquare?: boolean` - Crop to square aspect ratio (default: true)

### `apply_instagram_filter(bytes: Uint8Array, filterName: string): Uint8Array`
Apply Instagram-style filter presets.

**Available Filters:**
- `"clarendon"` - High contrast with lifted shadows
- `"gingham"` - Soft, dreamy look
- `"moon"` - Desaturated, cool tone
- `"lark"` - Bright and vibrant
- `"reyes"` - Vintage film look

### `get_version(): string`
Get library version string.

### `get_supported_formats(): string[]`
Get list of supported image formats.

### `benchmark_filters(bytes: Uint8Array, iterations: number): string`
Run performance benchmark on image processing operations.

## ⚠️ Error Handling

All methods that can fail return `Result<T, JsValue>`. In JavaScript, this means they will throw exceptions on error. Always wrap calls in try-catch blocks:

```javascript
try {
  const processor = WasmImageProcessor.from_bytes(imageBytes);
  processor.gaussian_blur(3.0);
  const result = processor.to_png_bytes();
} catch (error) {
  console.error('Image processing failed:', error);
}
```

**Common Errors:**
- Invalid image format
- Image too large (>64MP or >8192px per side)
- Invalid parameter ranges
- Memory allocation failures

## 📐 Type Definitions

### `ResizeAlgorithm`
```typescript
type ResizeAlgorithm = "Nearest" | "Bilinear" | "CatmullRom" | "Mitchell" | "Lanczos3";
```

### `InstagramFilter`
```typescript
type InstagramFilter = "clarendon" | "gingham" | "moon" | "lark" | "reyes";
```

## 🔗 Backend Integration Examples

### Express.js API Route
```javascript
import express from 'express';
import init, { WasmImageProcessor } from 'oelimg-rs';

const app = express();

// Initialize WASM module
await init();

app.post('/api/image/process', express.raw({ type: 'image/*', limit: '10mb' }), (req, res) => {
  try {
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(req.body));
    
    // Apply filters based on query parameters
    if (req.query.blur) processor.gaussian_blur(parseFloat(req.query.blur));
    if (req.query.brightness) processor.adjust_brightness_contrast(parseFloat(req.query.brightness), 1.0);
    if (req.query.resize) {
      const [width, height] = req.query.resize.split('x').map(Number);
      processor.resize_fit(width, height);
    }
    
    // Export result
    const resultBytes = processor.to_png_bytes();
    
    res.setHeader('Content-Type', 'image/png');
    res.send(Buffer.from(resultBytes));
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});
```

### Next.js API Route (App Router)
```typescript
import { NextRequest, NextResponse } from 'next/server';
import init, { WasmImageProcessor } from 'oelimg-rs';

// Initialize WASM (can be done at module level)
await init();

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData();
    const file = formData.get('image') as File;
    const operations = JSON.parse(formData.get('operations') as string);
    
    const arrayBuffer = await file.arrayBuffer();
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(arrayBuffer));
    
    // Apply operations
    for (const op of operations) {
      switch (op.type) {
        case 'blur':
          processor.gaussian_blur(op.radius);
          break;
        case 'brightness':
          processor.adjust_brightness_contrast(op.value, 1.0);
          break;
        case 'resize':
          processor.resize_fit(op.width, op.height);
          break;
        case 'instagram':
          // Note: Instagram filters work on bytes, so need special handling
          const currentBytes = processor.to_png_bytes();
          const { apply_instagram_filter } = await import('oelimg-rs');
          const filteredBytes = apply_instagram_filter(currentBytes, op.filter);
          // Replace processor with new one from filtered bytes
          const newProcessor = WasmImageProcessor.from_bytes(filteredBytes);
          Object.assign(processor, newProcessor);
          break;
      }
    }
    
    const resultBytes = processor.to_png_bytes();
    
    return new NextResponse(resultBytes, {
      headers: {
        'Content-Type': 'image/png',
        'Content-Length': resultBytes.length.toString(),
      },
    });
  } catch (error) {
    return NextResponse.json(
      { error: error.message },
      { status: 400 }
    );
  }
}
```

### Batch Processing Example
```javascript
import init, { WasmImageProcessor, create_thumbnail } from 'oelimg-rs';

await init();

async function processBatch(imageFiles, operations) {
  const results = [];
  
  for (const file of imageFiles) {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const processor = WasmImageProcessor.from_bytes(new Uint8Array(arrayBuffer));
      
      // Apply all operations
      for (const op of operations) {
        switch (op.type) {
          case 'resize':
            processor.resize_fit(op.width, op.height);
            break;
          case 'quality':
            // Apply quality enhancement
            processor.sharpen(0.5);
            processor.adjust_brightness_contrast(0.05, 1.1);
            break;
          case 'web-optimize':
            // Optimize for web delivery
            processor.resize_fit(1920, 1080);
            processor.sharpen(0.3);
            break;
        }
      }
      
      // Generate multiple outputs
      const fullSize = processor.to_webp_bytes();
      const thumbnail = create_thumbnail(processor.to_png_bytes(), 300, true);
      
      results.push({
        original: file.name,
        fullSize,
        thumbnail,
        dimensions: { width: processor.width, height: processor.height }
      });
    } catch (error) {
      results.push({
        original: file.name,
        error: error.message
      });
    }
  }
  
  return results;
}
```

## 🎯 Performance Tips

1. **Reuse processors** when applying multiple operations
2. **Use appropriate resize algorithms** - Lanczos3 for quality, Bilinear for speed
3. **Batch operations** instead of creating new processors for each filter
4. **Consider image size limits** - resize large images before heavy filtering
5. **Use WebP format** for smaller file sizes when supported
6. **Cache initialized WASM module** in server environments

## 📏 Parameter Ranges Summary

| Operation | Parameter | Range | Default | Notes |
|-----------|-----------|--------|---------|--------|
| HSL | hue | -180 to 180 | 0 | Degrees |
| HSL | saturation | 0 to 2 | 1 | Multiplier |
| HSL | lightness | 0 to 2 | 1 | Multiplier |
| Brightness | brightness | -1 to 1 | 0 | Additive |
| Contrast | contrast | 0 to 2 | 1 | Multiplier |
| Temperature | kelvin | 1000 to 40000 | 6500 | Color temp |
| Vibrance | vibrance | -100 to 100 | 0 | Percentage |
| Blur | radius | 0 to 100 | 0 | Pixels |
| Sharpen | strength | 0 to 10 | 0 | Multiplier |
| Sepia | intensity | 0 to 1 | 0 | Blend factor |
| Exposure | exposure | -5 to 5 | 0 | Stops |
| Levels | gamma | 0.1 to 3 | 1 | Gamma curve |

This comprehensive API reference should give you everything you need to implement robust backend image processing routes! 🚀