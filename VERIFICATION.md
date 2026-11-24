# DocumentBench verification

Verified in this resumed run using `build-app.cmd`:

- TypeScript and Vite production build completed successfully.
- Rust/Tauri debug application compiled successfully.
- Four Rust tests passed: current/subset page ranges, custom-range ordering/clamping/deduplication, saddle-stitch booklet ordering, and actual PDFium rendering of custom-range and booklet sheets.
- `target\debug\documentbench.exe --screenshot checks\resumed.png` exited with code 0.
- The resulting PNG is 1370 × 779 pixels, 173100 bytes, and has 96 distinct sampled colors in the screenshot validation script.

## Changes in this pass

- Aligned the React print preview and native composition for page ranges, N-up, booklet sheets, scaling, sheet navigation, and preview zoom.
- Fixed PDFium loading by passing the selected DLL path directly to the library binding. The fallback also finds the bundled development resource from Cargo's hashed test executable directory.
- Printing uses the current serialized document, including unsaved annotations and form values, with content filtering applied before composition.
- Added native file-dialog saving and binary-safe writing for extracted PDF pages, page images, and other exports.
- Custom ranges preserve requested order while removing repeated page numbers.
- Redaction UI explicitly warns that all text-showing and image operators on marked pages are removed, including outside marked rectangles.

## Launch verification

`launch-app.cmd` is the Windows launcher and `npm start` invokes it. It starts `target\debug\documentbench.exe`; the launcher does not invoke Vite, a browser, or a web server. Direct browser access to the frontend displays a desktop-only notice instead of the workspace.



- No physical printer was exercised. Printer-driver, duplex, paper-source, and hardware-booklet behavior remain unverified.
- Exact Nitro workspace parity remains unverified because the original main workspace was not captured. Original Nitro icons and branded artwork were not copied.
- Redaction is deliberately conservative and page-scoped, not coordinate-precise Nitro-equivalent redaction.
- Arbitrary PDF/font/form preservation, XFA calculations, direct glyph editing, PDF/A conformance, password/security parity, and cryptographic-signature parity are not established.
- OCR is an explicit configuration/unavailable message, not implemented recognition. Cloud AI translation and generative assistant behavior are not simulated.
- Word, PowerPoint, and other advertised external-format conversion shortcuts remain honest unavailable-workflow messages rather than completed converters.
