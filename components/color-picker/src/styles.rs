pub const CSS: &str = r#"
.ui-color-picker {
  --ui-color-picker-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-color-picker-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-picker-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-color-picker-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-color-picker-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-picker-radius-md: var(--ui-radius-md, var(--ui-fallback-radius-md));
  --ui-color-picker-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-color-picker-border: var(--ui-border, var(--ui-fallback-border));
  --ui-color-picker-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-picker-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-picker-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-color-picker-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-picker-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  --ui-color-picker-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  --ui-color-picker-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-color-picker-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-color-picker-overlay-panel-min-width: var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width));
  --ui-color-picker-shadow-md: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  --ui-color-picker-focus-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));
  --ui-color-picker-focus-outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
  --ui-color-picker-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
  --ui-color-picker-trigger-min-inline-size: calc(var(--ui-color-picker-space-md) * 11);
  display: inline-grid;
  gap: var(--ui-color-picker-space-xs);
}

.ui-color-picker[data-motion-source="custom"],
.ui-color-picker[data-custom-motion="true"] {
}

.ui-color-picker__trigger {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-color-picker-space-xs);
  min-inline-size: var(--ui-color-picker-trigger-min-inline-size);
  border: var(--ui-color-picker-border-width) solid color-mix(in oklch, var(--ui-color-picker-border), transparent 12%);
  border-radius: var(--ui-color-picker-radius-sm);
  padding: calc(var(--ui-color-picker-space-2xs) + var(--ui-color-picker-border-width)) var(--ui-color-picker-space-xs);
  background: color-mix(in oklch, var(--ui-color-picker-bg), var(--ui-color-picker-fg) 3%);
  color: var(--ui-color-picker-fg);
  cursor: pointer;
}

.ui-color-picker__trigger:hover {
  border-color: color-mix(in oklch, var(--ui-color-picker-accent), var(--ui-color-picker-border) 48%);
}

.ui-color-picker__trigger:focus-visible {
  outline: var(--ui-color-picker-focus-outline-width) solid color-mix(in oklch, var(--ui-color-picker-accent), transparent 62%);
  outline-offset: var(--ui-color-picker-focus-outline-offset);
}

.ui-color-picker__swatch {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-color-picker__label {
  font-size: var(--ui-color-picker-font-size-150);
  line-height: var(--ui-color-picker-line-height-150);
  font-weight: 600;
}

.ui-color-picker__value {
  margin-inline-start: auto;
  font-size: var(--ui-color-picker-font-size-100);
  line-height: var(--ui-color-picker-line-height-100);
  color: var(--ui-color-picker-fg-muted);
  font-variant-numeric: tabular-nums;
}

.ui-color-picker__panel {
  min-inline-size: var(--ui-color-picker-overlay-panel-min-width);
  border-radius: var(--ui-color-picker-radius-md);
  border: var(--ui-color-picker-border-width) solid color-mix(in oklch, var(--ui-color-picker-border), transparent 20%);
  background: var(--ui-color-picker-bg);
  box-shadow: var(--ui-color-picker-shadow-md);
}

.ui-color-picker__content {
  display: grid;
  gap: var(--ui-color-picker-space-sm);
  padding: var(--ui-color-picker-space-sm);
}

.ui-color-picker--open .ui-color-picker__trigger,
.ui-color-picker[data-open="true"] .ui-color-picker__trigger,
.ui-color-picker[data-state="open"] .ui-color-picker__trigger {
  border-color: color-mix(in oklch, var(--ui-color-picker-accent), var(--ui-color-picker-border) 24%);
  box-shadow: 0 0 0 var(--ui-color-picker-focus-outline-width) color-mix(in oklch, var(--ui-color-picker-accent), transparent 84%);
}

.ui-color-picker--disabled,
.ui-color-picker[data-disabled="true"] {
  opacity: var(--ui-color-picker-disabled-opacity);
}

.ui-color-picker--disabled .ui-color-picker__trigger,
.ui-color-picker[data-disabled="true"] .ui-color-picker__trigger {
  cursor: not-allowed;
}

.ui-color-picker--custom-class,
.ui-color-picker[data-custom-class="true"] {
  isolation: isolate;
}
"#;
