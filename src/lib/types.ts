export interface Author {
  name: string;
  url: string | null;
}

export interface Files {
  thumbnail: string;
  preview: string;
  original: string;
}

export interface Dimensions {
  width: number;
  height: number;
  aspectRatio?: string;
}

export interface Wallpaper {
  id: string;
  title: string;
  author?: Author;
  category: string;
  tags: string[];
  files: Files;
  dimensions: Dimensions;
  fileSizeBytes?: number;
  colorPalette?: string[];
  license?: string;
}

export interface BaseUrls {
  thumbnail: string;
  preview: string;
  original: string;
}

export interface CatalogManifest {
  version: number;
  updatedAt: string;
  baseUrls: BaseUrls;
  categories: string[];
  wallpapers: Wallpaper[];
}
