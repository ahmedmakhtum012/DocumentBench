# DocumentBench

DocumentBench is an independent desktop PDF workspace built with Tauri v2, React, TypeScript, Fluent UI icons, and a Rust PDF backend. The application uses original neutral branding and does not copy Nitro artwork or source code.

## Launch the Windows desktop app

Run `launch-app.cmd`, `npm start`, or `target\debug\documentbench.exe`. These start the native Tauri window. Do not open `index.html` or the Vite output as the application: the React interface is hosted inside Tauri and is not a standalone web application. Direct browser access shows a desktop-only notice rather than a limited browser imitation.

## Build

Run `npm run build` for the frontend and `cargo build --manifest-path src-tauri/Cargo.toml` for the desktop backend. On this machine, `build-app.ps1` configures the portable Node.js and Windows SDK paths before running both builds.

## Capture

`target/debug/documentbench.exe --screenshot path\to\capture.png` opens without focus, waits for 120 frame intervals, saves the window as a PNG, and exits. Normal startup opens the same application as a regular desktop window.

## Implementation status

Real PDF loading, rendering, saving, page rotation/reordering/extraction/deletion/combine, annotation overlays, deterministic structured extraction, Excel export, local recovery snapshots, and Windows printing are connected. See `MECHANICS.md` for observed original-program behavior and evidence ids.

Direct glyph editing, interactive AcroForm/XFA authoring, standards-validated digital signatures, PDF/A conformance, OCR without installed language data, cloud AI, and exact pixel-level Nitro parity are not claimed as complete. The interface shows explicit limitations for unfinished workflows.
