import { invoke } from '@tauri-apps/api/core';

export interface FileInfo {
  path: string;
  size: number;
  modified: number;
  accessed: number;
  created: number;
  ext: string;
}

export interface ScanStatus {
  running: boolean;
  scanned_files: number;
  scanned_bytes: number;
  current_path: string | null;
}

export const scanStart = (roots: string[]) => invoke('scan_start', { roots });
export const scanStatus = () => invoke('scan_status') as Promise<ScanStatus>;
export const scanResults = () => invoke('scan_results') as Promise<FileInfo[]>;
export const scanCancel = () => invoke('scan_cancel');
