# DocumentBench — evidence-backed behavior sheet

This is an independent implementation. It does not copy Nitro source code, branded icons, or proprietary assets.

## Evidence limits

- [OBSERVED] `NitroPDF.exe` is a 64-bit Windows native executable. It imports Windows printing, Windows App Runtime, UI Automation, and BCG ribbon support (`ev_1d7133825a5ab04ea71d977d7d2035d9d9b6c862960ea07e242bd00a731ff7ed`).
- [OBSERVED] A later live capture showed Nitro PDF Pro's splash screen and a login dialog with “Log in with browser” and “Skip log in,” but not its main ribbon workspace (`ev_7428c8a44fe4284cab9d5d7b619153b74dd51b19cdba145692fe7fc13cc0dd50`). Exact initial ribbon metrics and colors therefore remain [ESTIMATED].
- [OBSERVED] The rebuilt Tauri/React application was compiled and captured live at 1365×780, showing the ribbon, page thumbnails, document tab, and original sample document (`ev_99f12dbc5e339a362e8109bfe1e3cb70f6625c45824ab98f2e934b459a4e8b0b`). Its separate `--screenshot` mode also saves a PNG and exits successfully.
- [OBSERVED] The UI uses a `ribbon_strings` resource bundle; many ribbon strings are not plain strings in the executable (`ev_de1f4d1d70a8ab30bfdf0e6197287ae80db292db03a6fdb8c518b038d38f3873`).

## Navigation and view

- [OBSERVED] Zoom menu values: 25, 50, 75, 100, 125, 150, 200, 400, 800, 1600 percent. Stored command names are `sAVToolZoomMenu_25` through `_1600` (`ev_7a6befdc2221ba8ca92d3a6e2f053214323be5c43435dc6f7741a356aa0d5527`).
- [OBSERVED] Hand, edit, type text, fit page, actual size, fit width, zoom in, and zoom out are separate commands. The shortcut registration table at `0x140452ff0` uses Ctrl+H for hand, Ctrl+E for edit, Ctrl+T for type text, Ctrl+0/1/2 for fit page/actual size/fit width, Ctrl+J/K for zoom out/in. It also registers Ctrl+P, Ctrl+S, Ctrl+N, Ctrl+O, Ctrl+W, Ctrl+Z, Ctrl+Y, Ctrl+D and Ctrl+F (`ev_d2a917e777ed61e06e47763e2e635d9a2d4641ee4e394f2efd71ef247c90b484`). Ctrl is a derived reading of the registration's modifier argument.
- [OBSERVED] Snapshot's quick popup has Hand, separators, Zoom In, Zoom Out, Next Page, Previous Page, Add Note, Add Bookmark, and Document Properties (`0x1404d3f00`, `ev_527248a83fceaebbe394362b24065faa9ad3a0bb327433e98bfc41a495607dfb`).

## Recovery

- [OBSERVED] Auto recovery defaults to 10 minutes, is clamped to 1–120 minutes, and is multiplied by 60,000 ms unless `kPrefGeneralAutoRecoveryInSeconds` is enabled (then 1,000 ms). Function `0x14042aea0`, `ev_c7fbd5e25be0e5d402e1c955c218ce3f99dc5084258c6c621fd3eaf44b7a36f5`.
- [OBSERVED] Recovery files use `(Autosaved).pdf`, then `(Autosaved1).pdf`, `(Autosaved2).pdf`… until an unused path is found. Function `0x140504c40`, `ev_eae131de9cccdd1153be62be207efd15e1ace31099b1d917afc7b9c5c07792cb`.
- [DERIVED] Save a recovery snapshot only when dirty, on the configured interval; remove the pending recovery entry after a successful save or clean close.

## Printing

- [OBSERVED] Preview uses the selected printer's Windows `CreateDCW` device context. Failure is surfaced as “Failed to create printer device context … No print preview generated.” Function `0x1408b09f0`, `ev_05bd35367f201d4c18e96f17d6456a3465734d75a09123fd0a389e6fa10a0f73`, `ev_4b04cf7e2c66147210ad67b5e682f536b95622b9b6286f496b1cf2612abaa9e0`.
- [OBSERVED] Booklet printing, subset, and binding have separate properties (`kPagePrinterBookletPrinting`, `kPagePrinterBookletSubset`, `kPagePrinterBookletBinding`) and are read/written by print configuration functions (`ev_14c563b4312f784a6dfeac02543c5dc23943b1646c74637dc680f3de145a8c7c`).
- [ESTIMATED] The requested range/orientation/scaling/multiple-up/grayscale/markup controls are arranged as a two-pane backstage Print view. Exact visual placement is unverified.
- [IMPLEMENTATION] Print preview is rendered from the current PDF page bitmap and re-rendered when options change. N-up uses a uniform grid; booklet uses saddle-stitch sheet ordering with blank padding. Native Windows printing composes the same page ranges, scale, orientation, grayscale, N-up/booklet layout, content selection, and copies before using the installed print spooler. Four tests passed, including custom/current/odd/even ranges, left/right booklet ordering, and actual PDFium rendering of custom-range and booklet sheets. No physical print job was run; driver-specific duplex behavior is unverified.
- [IMPLEMENTATION] Saving serializes added annotations and field values once, returns the resulting PDF bytes, and clears the pending overlays so a second save cannot flatten the same objects twice. The flattened annotations remain visible in the rendered saved PDF, but are not reopened as editable annotation objects.

## Smart tools and permissions

- [OBSERVED] Table Extract, Form Extract, Form Create, Smart Redact, and Nitro Assistant have independently computed visibility and enabled flags (`0x14067e360`, `ev_472d5fa0c3773c5f90ad32510bc79898ad31156172c6e9dba2fbc71bd3aff938`).
- [OBSERVED] XFA state affects Form Create and Smart Redact eligibility. A JSON response maps the distinct operations `tableExtract`, `formExtract`, `autoForms`, `documentAssistantHandOff`, and `smartRedactHandOff` (`0x14093c1a0`, `ev_11d86c7e7424bf69799758819be2a1886eee7fcd23e0e73a3997ba5c6b198903`).
- [OBSERVED] Redaction and whiteout are separate operations; document/property permissions and edit state contribute to eligibility (`0x140678490`, `ev_a46825595997f0f56e8327c8ea9815eb0ad935312069f4335d430a60cd5e930e`).
- [OBSERVED] Stored redaction commands distinguish `mark_for_redaction`, `repeat_redaction_mark`, `search_and_redact`, `apply_all_pending_redactions`, and `remove_metadata` (`ev_b1f0aa119b2f838b1449d7129a60dbd0066eb2bb57396c408e62a9e9f6801135`).
- [IMPLEMENTATION] The local assistant is an explicit offline helper for summaries, search, extraction, and translation scaffolding. It does not claim cloud generative AI. A provider connection requires user configuration and is not silently simulated.

## Ribbon and backstage hierarchy

- [REQUESTED / NOT LIVE-OBSERVED] Tabs: File, Home, Edit, Convert, Review, Forms, Protect, Share, Smart Tools. Preserve these as separate tabs. Context Format tab appears only with a selected text/image/field object.
- [REQUESTED / NOT LIVE-OBSERVED] Quick Access: Open, Save, Print, Undo, Redo. Alt reveals shortcut badges. File opens backstage, not another ribbon tab.
- [IMPLEMENTATION] A multi-tab document bar sits above the workspace. Left navigation switches Pages, Bookmarks, Attachments, Comments. Status bar provides previous/next, page input, fit controls, zoom slider, and page-size readout.

## Core state transitions

1. Open: native picker -> parse PDF -> add document tab -> render active page + thumbnails -> clear per-document history.
2. Tool click: select tool -> change cursor -> click/drag canvas -> create pending annotation/edit or open tool panel -> commit through command history -> dirty state and recovery eligible.
3. Save: serialize current document -> write to target atomically -> clear dirty -> remove recovery snapshot -> keep current tab and page.
4. Print: Ctrl+P/File > Print -> open Backstage Print -> live preview -> options change updates preview and page ordering -> native print or cancel returns to same document state.
5. Redact: choose Mark or Smart Redact -> select matches -> pending marks are visibly outlined -> Apply All is a separate irreversible command -> save as a new document is recommended.
6. Smart extraction: select Table/Form Extract -> choose pages -> review structured preview -> export Excel-compatible file -> original PDF remains unchanged.
7. Compare: choose second PDF -> side-by-side pages with independent file names -> sync scroll locks both views -> page/text differences highlighted -> close compare restores one-page view.

## Redaction safety limitation

- [IMPLEMENTATION] Applying any redaction mark currently removes all text-showing and image operators on that marked page, not only content inside the mark's rectangle. The confirmation dialog states this explicitly and instructs the person to use a copy and inspect every output page. Coordinate-precise redaction remains unverified and is not claimed.

## Unverified / deliberately honest boundaries

- Exact Nitro v26 pixel metrics, theme colors, icon shapes, all menu accelerators, and license/trial gating.
- Fidelity of arbitrary direct glyph editing, RTL font shaping, XFA execution and JavaScript calculation parity.
- PDF/A-1b/A-3 validation, long-term cryptographic signature validation, and conformance beyond what the local PDF engine provides.
- Driver-specific duplex and booklet hardware settings; OCR language availability depends on installed data.
- Cloud assistant behavior and remote AI output. Local smart tools are deterministic and clearly labelled.

Original-program behavior above is static/decompiled except for the explicitly identified splash/login capture. No exact full-workspace runtime parity is claimed.
