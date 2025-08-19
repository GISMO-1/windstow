# WinStow (AI Disk Optimizer — Lite)

WinStow is a **safe, usage-aware storage cleaner** for Windows. It scans drives, scores “cold” data, finds duplicates, lets you **simulate** actions, then executes **reversible** cleanups (Recycle Bin, archive moves, ZIP compression). Built with **Tauri (Rust + React)** for small installers and native performance.

## Features (MVP)
- Fast parallel scan of selected drives/folders (path, size, timestamps, extension).
- Coldness score (age + size).
- Duplicate detection (size → BLAKE3 hash).
- Plan & simulate: estimate freed space and risks before acting.
- Safe execute: Recycle Bin deletes, archive moves, rollback last session.
- Reports: export HTML + JSON.

## Tech
- UI: Tauri + React (Vite), Recharts.
- Core: Rust (`walkdir`, `rayon`, `blake3`, `rusqlite`).
- Windows APIs for metadata and Recycle Bin.
- CI: GitHub Actions builds Windows installer on tag pushes.

## Dev Setup
```bash
# Requirements: Node 20+, Rust stable, pnpm or npm
cd app
pnpm i         # or npm ci
pnpm tauri dev # or npm run tauri:dev
