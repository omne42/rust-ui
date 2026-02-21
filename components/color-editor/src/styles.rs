pub const CSS: &str = r#"
.ui-color-editor {
  --ui-color-editor-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-color-editor-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-editor-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-color-editor-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-color-editor-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-editor-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-color-editor-border: var(--ui-border, var(--ui-fallback-border));
  --ui-color-editor-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-editor-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-editor-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-color-editor-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-editor-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-color-editor-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-color-editor-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  --ui-color-editor-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  --ui-color-editor-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
  --ui-color-editor-focus-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));
  --ui-color-editor-focus-outline-offset: calc(var(--ui-color-editor-focus-outline-width) * -1);
  --ui-color-editor-letter-spacing: var(--ui-command-group-heading-letter-spacing, var(--ui-fallback-command-group-heading-letter-spacing));
  --ui-color-editor-canvas-sidebar-min: calc(var(--ui-color-editor-space-md) * 11);
  --ui-color-editor-canvas-sidebar-max: calc(var(--ui-color-editor-space-md) * 12);
  --ui-color-editor-channel-min: calc(var(--ui-color-editor-space-md) * 4.5);
  --ui-color-editor-channel-min-hex: calc(var(--ui-color-editor-space-md) * 8);
  display: grid;
  gap: var(--ui-color-editor-space-sm);
}

.ui-color-editor__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-color-editor-space-sm);
}

.ui-color-editor__label {
  font-size: var(--ui-color-editor-font-size-150);
  line-height: var(--ui-color-editor-line-height-150);
  font-weight: 600;
  color: var(--ui-color-editor-fg);
}

.ui-color-editor__value {
  font-size: var(--ui-color-editor-font-size-100);
  line-height: var(--ui-color-editor-line-height-100);
  color: var(--ui-color-editor-fg-muted);
  font-variant-numeric: tabular-nums;
}

.ui-color-editor__canvas {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-color-editor-space-sm);
  align-items: start;
}

.ui-color-editor__area.ui-color-area {
  margin: 0;
  flex: 1 1 calc(var(--ui-color-editor-canvas-sidebar-max) * 1.5);
  min-inline-size: 0;
}

.ui-color-editor__sliders {
  display: grid;
  gap: var(--ui-color-editor-space-xs);
  flex: 1 1 var(--ui-color-editor-canvas-sidebar-min);
  max-inline-size: var(--ui-color-editor-canvas-sidebar-max);
  min-inline-size: min(100%, var(--ui-color-editor-canvas-sidebar-min));
}

.ui-color-editor__slider.ui-color-slider {
  margin: 0;
}

.ui-color-editor__controls {
  display: grid;
  gap: var(--ui-color-editor-space-xs);
}

.ui-color-editor__formats {
  display: inline-flex;
  gap: 0;
  border: var(--ui-color-editor-border-width) solid color-mix(in oklch, var(--ui-color-editor-border), transparent 18%);
  border-radius: var(--ui-color-editor-radius-sm);
  background: color-mix(in oklch, var(--ui-color-editor-bg), var(--ui-color-editor-fg) 2%);
  overflow: hidden;
}

.ui-color-editor__format-button {
  appearance: none;
  border: 0;
  background: transparent;
  color: var(--ui-color-editor-fg-muted);
  font-size: var(--ui-color-editor-font-size-100);
  line-height: var(--ui-color-editor-line-height-100);
  font-weight: 600;
  letter-spacing: var(--ui-color-editor-letter-spacing);
  padding: var(--ui-color-editor-space-2xs) var(--ui-color-editor-space-xs);
  cursor: pointer;
}

.ui-color-editor__format-button[data-selected="true"] {
  background: color-mix(in oklch, var(--ui-color-editor-accent), transparent 78%);
  color: var(--ui-color-editor-fg);
}

.ui-color-editor__format-button:focus-visible {
  outline: var(--ui-color-editor-focus-outline-width) solid color-mix(in oklch, var(--ui-color-editor-accent), transparent 62%);
  outline-offset: var(--ui-color-editor-focus-outline-offset);
}

.ui-color-editor__channels {
  display: grid;
  gap: var(--ui-space-3xs, var(--ui-color-editor-space-2xs));
  grid-template-columns: repeat(auto-fit, minmax(var(--ui-color-editor-channel-min), 1fr));
}

.ui-color-editor__channel-row {
  display: grid;
  gap: var(--ui-color-editor-space-2xs);
  border: var(--ui-color-editor-border-width) solid color-mix(in oklch, var(--ui-color-editor-border), transparent 24%);
  border-radius: var(--ui-color-editor-radius-sm);
  padding: var(--ui-color-editor-space-2xs) var(--ui-color-editor-space-xs);
  background: color-mix(in oklch, var(--ui-color-editor-bg), var(--ui-color-editor-fg) 2%);
}

.ui-color-editor__channel-key {
  font-size: calc(var(--ui-color-editor-font-size-100) - (var(--ui-color-editor-border-width) * 2));
  line-height: var(--ui-color-editor-line-height-100);
  color: var(--ui-color-editor-fg-muted);
  text-transform: uppercase;
  letter-spacing: calc(var(--ui-color-editor-letter-spacing) * 2);
}

.ui-color-editor__channel-value {
  font-size: var(--ui-color-editor-font-size-100);
  line-height: var(--ui-color-editor-line-height-100);
  font-variant-numeric: tabular-nums;
  color: var(--ui-color-editor-fg);
}

.ui-color-editor--format-hex .ui-color-editor__channels {
  grid-template-columns: minmax(var(--ui-color-editor-channel-min-hex), 1fr);
}

.ui-color-editor--disabled,
.ui-color-editor[data-disabled="true"] {
  opacity: var(--ui-color-editor-disabled-opacity);
}

.ui-color-editor--disabled .ui-color-editor__format-button,
.ui-color-editor[data-disabled="true"] .ui-color-editor__format-button {
  cursor: not-allowed;
}

.ui-color-editor--alpha-hidden .ui-color-editor__slider--alpha,
.ui-color-editor[data-alpha="hidden"] .ui-color-editor__slider--alpha {
  display: none;
}

.ui-color-editor--custom-class,
.ui-color-editor[data-custom-class="true"] {
  isolation: isolate;
}
"#;
