pub const CSS: &str = r#"
.ui-color-thumb {
  position: absolute;
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 1;
}

.ui-color-thumb--x-start {
  left: 16%;
}

.ui-color-thumb--x-center {
  left: 50%;
}

.ui-color-thumb--x-end {
  left: 84%;
}

.ui-color-thumb--y-start {
  top: 16%;
}

.ui-color-thumb--y-center {
  top: 50%;
}

.ui-color-thumb--y-end {
  top: 84%;
}

.ui-color-thumb__handle {
  pointer-events: auto;
  inline-size: 1.125rem;
  block-size: 1.125rem;
  border-radius: var(--ui-radius-full, 999px);
  border: 2px solid var(--ui-bg);
  box-shadow:
    0 0 0 1px color-mix(in oklch, var(--ui-fg), transparent 56%),
    var(--ui-shadow-sm);
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 4%);
  transition:
    transform 140ms ease,
    box-shadow 160ms ease,
    opacity 120ms ease;
}

.ui-color-thumb__fill,
.ui-color-thumb__loupe-fill {
  display: block;
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
}

.ui-color-thumb__swatch.ui-color-swatch,
.ui-color-thumb__loupe-swatch.ui-color-swatch {
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
  min-inline-size: 0;
  min-block-size: 0;
}

.ui-color-thumb__loupe {
  position: absolute;
  left: 50%;
  bottom: calc(100% + var(--ui-space-xs));
  transform: translateX(-50%);
  inline-size: 1.875rem;
  block-size: 1.875rem;
  border-radius: var(--ui-radius-full, 999px);
  padding: 2px;
  background: var(--ui-bg);
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 24%);
  box-shadow: var(--ui-shadow-md);
  animation: ui-color-thumb-loupe-in 120ms ease-out;
}

.ui-color-thumb--focused .ui-color-thumb__handle,
.ui-color-thumb[data-focused="true"] .ui-color-thumb__handle {
  transform: scale(1.12);
  box-shadow:
    0 0 0 2px color-mix(in oklch, var(--ui-accent), transparent 72%),
    0 0 0 1px color-mix(in oklch, var(--ui-fg), transparent 56%),
    var(--ui-shadow-sm);
}

.ui-color-thumb--dragging .ui-color-thumb__handle,
.ui-color-thumb[data-dragging="true"] .ui-color-thumb__handle {
  transform: scale(1.18);
}

.ui-color-thumb--disabled,
.ui-color-thumb[data-disabled="true"] {
  opacity: 0.58;
}

.ui-color-thumb--disabled .ui-color-thumb__handle,
.ui-color-thumb[data-disabled="true"] .ui-color-thumb__handle {
  pointer-events: none;
}

.ui-color-thumb--custom-class,
.ui-color-thumb[data-custom-class="true"] {
  isolation: isolate;
}

@keyframes ui-color-thumb-loupe-in {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(0.2rem) scale(0.88);
  }

  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0) scale(1);
  }
}
"#;
