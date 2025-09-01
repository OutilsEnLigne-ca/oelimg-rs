// TypeScript definitions for oelimg-rs

export interface ImageProcessorOptions {
  width: number;
  height: number;
  data: Uint8Array;
}

export interface SvgRenderOptions {
  width?: number;
  height?: number;
  backgroundColor?: [number, number, number, number]; // RGBA
}

export interface FilterOptions {
  intensity?: number;
  strength?: number;
}

export interface ResizeOptions {
  algorithm?: ResizeAlgorithm;
  maintainAspectRatio?: boolean;
}

export interface CropOptions {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface HSLAdjustment {
  hue: number;        // -180 to 180 degrees
  saturation: number; // 0 to 2 (1 = no change)
  lightness: number;  // 0 to 2 (1 = no change)
}

export interface BrightnessContrastAdjustment {
  brightness: number; // -1 to 1 (0 = no change)
  contrast: number;   // 0 to 2 (1 = no change)
}

export interface ColorTemperatureAdjustment {
  kelvin: number;    // 1000 to 40000 (6500 = daylight)
  strength?: number; // 0 to 1 (1 = full effect)
}

export interface VintageOptions {
  warmth: number;    // 0 to 2 (1 = neutral)
  vignette: number;  // 0 to 1
  grain: number;     // 0 to 1
  fade: number;      // 0 to 1
}

export interface LevelsAdjustment {
  shadows: number;    // 0 to 1
  gamma: number;      // 0.1 to 3.0
  highlights: number; // 0 to 1
}

export interface ExposureAdjustment {
  exposure: number;   // -5 to 5 (stops)
  highlights: number; // -100 to 0
  shadows: number;    // 0 to 100
}

export enum ResizeAlgorithm {
  Nearest = "Nearest",
  Bilinear = "Bilinear", 
  CatmullRom = "CatmullRom",
  Mitchell = "Mitchell",
  Lanczos3 = "Lanczos3"
}

export enum ImageFormat {
  PNG = "png",
  JPEG = "jpeg",
  WebP = "webp",
  ICO = "ico",
  SVG = "svg"
}

export type InstagramFilter = 
  | "clarendon" 
  | "gingham" 
  | "moon" 
  | "lark" 
  | "reyes";

export class WasmImageProcessor {
  constructor(width: number, height: number, data: Uint8Array);
  
  static from_bytes(bytes: Uint8Array): WasmImageProcessor;
  static from_svg_bytes(
    bytes: Uint8Array, 
    width?: number, 
    height?: number, 
    backgroundColor?: number[]
  ): WasmImageProcessor;
  
  readonly width: number;
  readonly height: number;
  readonly data: Uint8Array;
  
  // Export methods
  to_png_bytes(): Uint8Array;
  to_jpeg_bytes(quality: number): Uint8Array;
  to_webp_bytes(): Uint8Array;
  to_ico_bytes(): Uint8Array;
  
  // Color adjustments
  adjust_hsl(hue: number, saturation: number, lightness: number): void;
  adjust_brightness_contrast(brightness: number, contrast: number): void;
  adjust_color_temperature(kelvin: number, strength?: number): void;
  adjust_vibrance(vibrance: number): void;
  
  // Transform operations
  crop(x: number, y: number, width: number, height: number): void;
  resize(width: number, height: number, algorithm?: ResizeAlgorithm): void;
  resize_fit(maxWidth: number, maxHeight: number, algorithm?: ResizeAlgorithm): void;
  rotate(angleDegrees: number, backgroundColor?: number[]): void;
  flip_horizontal(): void;
  flip_vertical(): void;
  
  // Artistic filters
  gaussian_blur(radius: number): void;
  sharpen(strength: number): void;
  sepia(intensity: number): void;
  vintage(warmth: number, vignette: number, grain: number, fade: number): void;
  
  // Correction filters
  auto_levels(): void;
  adjust_levels(shadows: number, gamma: number, highlights: number): void;
  adjust_exposure(exposure: number, highlights: number, shadows: number): void;
}

// Utility functions
export function load_image_from_bytes(bytes: Uint8Array): WasmImageProcessor;
export function create_thumbnail(
  bytes: Uint8Array, 
  size: number, 
  cropToSquare?: boolean
): Uint8Array;
export function apply_instagram_filter(
  bytes: Uint8Array, 
  filterName: InstagramFilter
): Uint8Array;

// SVG utility functions
export function convert_svg_to_png(
  svgBytes: Uint8Array,
  width?: number,
  height?: number,
  backgroundColor?: number[]
): Uint8Array;
export function convert_svg_to_webp(
  svgBytes: Uint8Array,
  width?: number,
  height?: number,
  backgroundColor?: number[]
): Uint8Array;
export function is_svg_format(bytes: Uint8Array): boolean;

// ICO utility functions
export function convert_to_ico(bytes: Uint8Array): Uint8Array;
export function create_favicon(bytes: Uint8Array): Uint8Array;

export function get_version(): string;
export function get_supported_formats(): string[];
export function benchmark_filters(bytes: Uint8Array, iterations: number): string;

// Initialize WebAssembly module
export default function init(input?: string | URL | Request): Promise<void>;