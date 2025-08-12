// Next.js React hook for using oelimg-rs
// Place this in your Next.js project: hooks/useImageProcessor.ts

import { useCallback, useEffect, useState } from 'react';
import { WasmImageProcessor, ResizeAlgorithm, InstagramFilter } from 'oelimg-rs';

interface ImageProcessorHookReturn {
  processor: WasmImageProcessor | null;
  isReady: boolean;
  error: string | null;
  loadImage: (file: File) => Promise<void>;
  applyFilter: (filterType: string, options: any) => void;
  exportImage: (format: 'png' | 'jpeg' | 'webp') => Uint8Array | null;
  reset: () => void;
  getPreview: () => string | null;
}

interface UseImageProcessorOptions {
  maxDimension?: number;
  autoResize?: boolean;
}

export function useImageProcessor(options: UseImageProcessorOptions = {}): ImageProcessorHookReturn {
  const [processor, setProcessor] = useState<WasmImageProcessor | null>(null);
  const [originalProcessor, setOriginalProcessor] = useState<WasmImageProcessor | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize WebAssembly module
  useEffect(() => {
    const initWasm = async () => {
      try {
        // Dynamic import to avoid SSR issues
        const { default: init } = await import('oelimg-rs');
        await init();
        setIsReady(true);
        setError(null);
      } catch (err) {
        setError(`Failed to initialize WebAssembly: ${err}`);
        console.error('WASM init error:', err);
      }
    };

    initWasm();
  }, []);

  const loadImage = useCallback(async (file: File) => {
    if (!isReady) {
      throw new Error('WebAssembly module not ready');
    }

    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      
      const { WasmImageProcessor } = await import('oelimg-rs');
      const newProcessor = WasmImageProcessor.from_bytes(bytes);
      
      // Auto-resize if configured
      if (options.autoResize && options.maxDimension) {
        const { maxDimension } = options;
        if (newProcessor.width > maxDimension || newProcessor.height > maxDimension) {
          newProcessor.resize_fit(maxDimension, maxDimension, ResizeAlgorithm.Lanczos3);
        }
      }

      // Keep original for reset functionality
      const originalBytes = newProcessor.to_png_bytes();
      const originalProcessorInstance = WasmImageProcessor.from_bytes(originalBytes);
      
      setProcessor(newProcessor);
      setOriginalProcessor(originalProcessorInstance);
      setError(null);
    } catch (err) {
      setError(`Failed to load image: ${err}`);
      console.error('Image load error:', err);
    }
  }, [isReady, options.autoResize, options.maxDimension]);

  const applyFilter = useCallback((filterType: string, options: any = {}) => {
    if (!processor) {
      setError('No image loaded');
      return;
    }

    try {
      switch (filterType) {
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
            options.strength || 1
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
          // For Instagram filters, we need to recreate from bytes
          const currentBytes = processor.to_png_bytes();
          const { apply_instagram_filter } = require('oelimg-rs');
          const filteredBytes = apply_instagram_filter(currentBytes, options.filter);
          const newProcessor = WasmImageProcessor.from_bytes(filteredBytes);
          setProcessor(newProcessor);
          return;
          
        default:
          throw new Error(`Unknown filter type: ${filterType}`);
      }
      
      setError(null);
    } catch (err) {
      setError(`Filter error: ${err}`);
      console.error('Filter application error:', err);
    }
  }, [processor]);

  const exportImage = useCallback((format: 'png' | 'jpeg' | 'webp' = 'png') => {
    if (!processor) {
      setError('No image to export');
      return null;
    }

    try {
      switch (format) {
        case 'png':
          return processor.to_png_bytes();
        case 'jpeg':
          return processor.to_jpeg_bytes(90);
        case 'webp':
          return processor.to_webp_bytes();
        default:
          throw new Error(`Unsupported format: ${format}`);
      }
    } catch (err) {
      setError(`Export error: ${err}`);
      console.error('Export error:', err);
      return null;
    }
  }, [processor]);

  const getPreview = useCallback(() => {
    if (!processor) return null;
    
    try {
      const bytes = processor.to_png_bytes();
      const blob = new Blob([bytes], { type: 'image/png' });
      return URL.createObjectURL(blob);
    } catch (err) {
      setError(`Preview error: ${err}`);
      return null;
    }
  }, [processor]);

  const reset = useCallback(() => {
    if (!originalProcessor) {
      setError('No original image to reset to');
      return;
    }

    try {
      const originalBytes = originalProcessor.to_png_bytes();
      const { WasmImageProcessor } = require('oelimg-rs');
      const resetProcessor = WasmImageProcessor.from_bytes(originalBytes);
      setProcessor(resetProcessor);
      setError(null);
    } catch (err) {
      setError(`Reset error: ${err}`);
      console.error('Reset error:', err);
    }
  }, [originalProcessor]);

  return {
    processor,
    isReady,
    error,
    loadImage,
    applyFilter,
    exportImage,
    reset,
    getPreview,
  };
}

// Hook for quick Instagram-style filters
export function useInstagramFilters() {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    const initWasm = async () => {
      try {
        const { default: init } = await import('oelimg-rs');
        await init();
        setIsReady(true);
      } catch (err) {
        console.error('WASM init error:', err);
      }
    };
    initWasm();
  }, []);

  const applyInstagramFilter = useCallback(async (
    imageBytes: Uint8Array, 
    filter: InstagramFilter
  ): Promise<Uint8Array | null> => {
    if (!isReady) return null;

    try {
      const { apply_instagram_filter } = await import('oelimg-rs');
      return apply_instagram_filter(imageBytes, filter);
    } catch (err) {
      console.error('Instagram filter error:', err);
      return null;
    }
  }, [isReady]);

  return {
    isReady,
    applyInstagramFilter,
  };
}