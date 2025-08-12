// Next.js API route for server-side image processing
// Place this in your Next.js project: pages/api/image/process.ts or app/api/image/process/route.ts

import { NextApiRequest, NextApiResponse } from 'next';
// For App Router, use: import { NextRequest, NextResponse } from 'next/server';

interface ProcessImageRequest {
  imageData: string; // Base64 encoded image
  operations: ImageOperation[];
}

interface ImageOperation {
  type: string;
  options: Record<string, any>;
}

interface ProcessImageResponse {
  success: boolean;
  imageData?: string; // Base64 encoded result
  error?: string;
  metadata?: {
    originalSize: { width: number; height: number };
    processedSize: { width: number; height: number };
    processingTime: number;
  };
}

// Pages Router version
export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<ProcessImageResponse>
) {
  if (req.method !== 'POST') {
    return res.status(405).json({ 
      success: false, 
      error: 'Method not allowed' 
    });
  }

  try {
    const startTime = Date.now();
    const { imageData, operations } = req.body as ProcessImageRequest;

    if (!imageData || !operations) {
      return res.status(400).json({
        success: false,
        error: 'Missing imageData or operations'
      });
    }

    // Initialize WebAssembly (server-side)
    const { 
      default: init, 
      WasmImageProcessor 
    } = await import('oelimg-rs');
    
    await init();

    // Decode base64 image
    const imageBytes = Buffer.from(imageData.split(',')[1], 'base64');
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(imageBytes));

    const originalSize = {
      width: processor.width,
      height: processor.height
    };

    // Apply operations in sequence
    for (const operation of operations) {
      await applyOperation(processor, operation);
    }

    const processedSize = {
      width: processor.width,
      height: processor.height
    };

    // Export processed image
    const resultBytes = processor.to_png_bytes();
    const resultBase64 = `data:image/png;base64,${Buffer.from(resultBytes).toString('base64')}`;

    const processingTime = Date.now() - startTime;

    res.status(200).json({
      success: true,
      imageData: resultBase64,
      metadata: {
        originalSize,
        processedSize,
        processingTime
      }
    });

  } catch (error) {
    console.error('Image processing error:', error);
    res.status(500).json({
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error'
    });
  }
}

// App Router version (alternative)
export async function POST(request: Request) {
  try {
    const startTime = Date.now();
    const body = await request.json() as ProcessImageRequest;
    const { imageData, operations } = body;

    if (!imageData || !operations) {
      return Response.json(
        { success: false, error: 'Missing imageData or operations' },
        { status: 400 }
      );
    }

    // Initialize WebAssembly
    const { default: init, WasmImageProcessor } = await import('oelimg-rs');
    await init();

    // Process image
    const imageBytes = Buffer.from(imageData.split(',')[1], 'base64');
    const processor = WasmImageProcessor.from_bytes(new Uint8Array(imageBytes));

    const originalSize = { width: processor.width, height: processor.height };

    // Apply operations
    for (const operation of operations) {
      await applyOperation(processor, operation);
    }

    const processedSize = { width: processor.width, height: processor.height };
    const resultBytes = processor.to_png_bytes();
    const resultBase64 = `data:image/png;base64,${Buffer.from(resultBytes).toString('base64')}`;

    return Response.json({
      success: true,
      imageData: resultBase64,
      metadata: {
        originalSize,
        processedSize,
        processingTime: Date.now() - startTime
      }
    });

  } catch (error) {
    console.error('Image processing error:', error);
    return Response.json(
      { 
        success: false, 
        error: error instanceof Error ? error.message : 'Unknown error' 
      },
      { status: 500 }
    );
  }
}

// Helper function to apply individual operations
async function applyOperation(processor: any, operation: ImageOperation) {
  const { type, options } = operation;

  switch (type) {
    case 'hsl':
      processor.adjust_hsl(
        options.hue || 0,
        options.saturation || 1,
        options.lightness || 1
      );
      break;

    case 'brightness-contrast':
      processor.adjust_brightness_contrast(
        options.brightness || 0,
        options.contrast || 1
      );
      break;

    case 'color-temperature':
      processor.adjust_color_temperature(
        options.kelvin || 6500,
        options.strength
      );
      break;

    case 'vibrance':
      processor.adjust_vibrance(options.vibrance || 0);
      break;

    case 'blur':
      processor.gaussian_blur(options.radius || 1);
      break;

    case 'sharpen':
      processor.sharpen(options.strength || 1);
      break;

    case 'sepia':
      processor.sepia(options.intensity || 0.5);
      break;

    case 'vintage':
      processor.vintage(
        options.warmth || 1.2,
        options.vignette || 0.3,
        options.grain || 0.1,
        options.fade || 0.1
      );
      break;

    case 'crop':
      processor.crop(options.x, options.y, options.width, options.height);
      break;

    case 'resize':
      if (options.fit) {
        processor.resize_fit(options.width, options.height, options.algorithm);
      } else {
        processor.resize(options.width, options.height, options.algorithm);
      }
      break;

    case 'rotate':
      processor.rotate(options.angle || 0, options.backgroundColor);
      break;

    case 'flip-horizontal':
      processor.flip_horizontal();
      break;

    case 'flip-vertical':
      processor.flip_vertical();
      break;

    case 'auto-levels':
      processor.auto_levels();
      break;

    case 'exposure':
      processor.adjust_exposure(
        options.exposure || 0,
        options.highlights || 0,
        options.shadows || 0
      );
      break;

    case 'instagram':
      // Instagram filters need special handling since they work on bytes
      const currentBytes = processor.to_png_bytes();
      const { apply_instagram_filter } = await import('oelimg-rs');
      const filteredBytes = apply_instagram_filter(currentBytes, options.filter);
      
      // Replace processor with filtered result
      const { WasmImageProcessor } = await import('oelimg-rs');
      const newProcessor = WasmImageProcessor.from_bytes(filteredBytes);
      
      // Copy properties back to original processor
      Object.assign(processor, newProcessor);
      break;

    default:
      throw new Error(`Unknown operation type: ${type}`);
  }
}

// Utility function for thumbnail generation endpoint
// Place this in pages/api/image/thumbnail.ts
export async function generateThumbnail(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const { imageData, size = 150, cropToSquare = true } = req.body;

    const { default: init, create_thumbnail } = await import('oelimg-rs');
    await init();

    const imageBytes = Buffer.from(imageData.split(',')[1], 'base64');
    const thumbnailBytes = create_thumbnail(
      new Uint8Array(imageBytes),
      size,
      cropToSquare
    );

    const thumbnailBase64 = `data:image/png;base64,${Buffer.from(thumbnailBytes).toString('base64')}`;

    res.status(200).json({
      success: true,
      thumbnail: thumbnailBase64
    });

  } catch (error) {
    console.error('Thumbnail generation error:', error);
    res.status(500).json({
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error'
    });
  }
}

// Usage example for the client side:
/*
async function processImage(file: File) {
  const reader = new FileReader();
  
  return new Promise((resolve, reject) => {
    reader.onload = async (e) => {
      try {
        const imageData = e.target?.result as string;
        
        const response = await fetch('/api/image/process', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            imageData,
            operations: [
              { type: 'blur', options: { radius: 2 } },
              { type: 'brightness-contrast', options: { brightness: 0.1, contrast: 1.2 } },
              { type: 'resize', options: { width: 800, height: 600, fit: true } }
            ]
          }),
        });

        const result = await response.json();
        resolve(result);
      } catch (error) {
        reject(error);
      }
    };
    
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}
*/