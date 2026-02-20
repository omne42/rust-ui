# VisuallyHidden

`VisuallyHidden` is a non-visual accessibility utility component for screen-reader-only content.

## Layering

- `logic.rs`: prop normalization and typed state/source markers.
- `view.rs`: Leptos rendering + locale a11y attrs wiring.
- `styles.rs`: static hidden/focusable CSS contract.
- `mod.rs`: public exports.
