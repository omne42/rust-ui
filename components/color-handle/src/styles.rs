pub const CSS: &str = r#"
.ui-color-handle {
  display: grid;
  gap: var(--ui-space-2xs);
}

.ui-color-handle__surface {
  position: relative;
  min-inline-size: 12rem;
  min-block-size: 7rem;
  border-radius: var(--ui-radius-sm);
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 18%);
  background:
    linear-gradient(135deg, color-mix(in oklch, var(--ui-accent), transparent 82%), transparent),
    color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);
  overflow: hidden;
  transition:
    border-color var(--ui-color-handle-motion-duration, 140ms) ease,
    box-shadow var(--ui-color-handle-motion-duration, 140ms) ease;
}

.ui-color-handle__thumb.ui-color-thumb {
  position: absolute;
}

.ui-color-handle--focused .ui-color-handle__surface,
.ui-color-handle[data-focused="true"] .ui-color-handle__surface {
  border-color: color-mix(in oklch, var(--ui-accent), var(--ui-border) 42%);
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent), transparent 86%);
}

.ui-color-handle--dragging .ui-color-handle__surface,
.ui-color-handle[data-dragging="true"] .ui-color-handle__surface {
  border-color: color-mix(in oklch, var(--ui-accent), var(--ui-border) 24%);
}

.ui-color-handle--disabled,
.ui-color-handle[data-disabled="true"] {
  opacity: 0.58;
}

.ui-color-handle--disabled .ui-color-handle__surface,
.ui-color-handle[data-disabled="true"] .ui-color-handle__surface {
  cursor: not-allowed;
}

.ui-color-handle--custom-class,
.ui-color-handle[data-custom-class="true"] {
  isolation: isolate;
}
"#;
