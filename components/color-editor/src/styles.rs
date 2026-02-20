pub const CSS: &str = r#"
.ui-color-editor {
  display: grid;
  gap: var(--ui-space-sm);
}

.ui-color-editor__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-sm);
}

.ui-color-editor__label {
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-color-editor__value {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
  font-variant-numeric: tabular-nums;
}

.ui-color-editor__canvas {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(11rem, 12rem);
  gap: var(--ui-space-sm);
  align-items: start;
}

.ui-color-editor__area.ui-color-area {
  margin: 0;
}

.ui-color-editor__sliders {
  display: grid;
  gap: var(--ui-space-xs);
}

.ui-color-editor__slider.ui-color-slider {
  margin: 0;
}

.ui-color-editor__controls {
  display: grid;
  gap: var(--ui-space-xs);
}

.ui-color-editor__formats {
  display: inline-flex;
  gap: 0;
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 18%);
  border-radius: var(--ui-radius-sm);
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);
  overflow: hidden;
}

.ui-color-editor__format-button {
  appearance: none;
  border: 0;
  background: transparent;
  color: var(--ui-fg-muted);
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 600;
  letter-spacing: 0.01em;
  padding: var(--ui-space-2xs) var(--ui-space-xs);
  cursor: pointer;
}

.ui-color-editor__format-button[data-selected="true"] {
  background: color-mix(in oklch, var(--ui-accent), transparent 78%);
  color: var(--ui-fg);
}

.ui-color-editor__format-button:focus-visible {
  outline: 2px solid color-mix(in oklch, var(--ui-accent), transparent 62%);
  outline-offset: -2px;
}

.ui-color-editor__channels {
  display: grid;
  gap: var(--ui-space-3xs);
  grid-template-columns: repeat(auto-fit, minmax(4.5rem, 1fr));
}

.ui-color-editor__channel-row {
  display: grid;
  gap: 2px;
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 24%);
  border-radius: var(--ui-radius-xs);
  padding: var(--ui-space-2xs) var(--ui-space-xs);
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);
}

.ui-color-editor__channel-key {
  font-size: calc(var(--ui-font-size-100, 12px) - 2px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

.ui-color-editor__channel-value {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-variant-numeric: tabular-nums;
  color: var(--ui-fg);
}

.ui-color-editor--format-hex .ui-color-editor__channels {
  grid-template-columns: minmax(8rem, 1fr);
}

.ui-color-editor--disabled,
.ui-color-editor[data-disabled="true"] {
  opacity: 0.62;
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

@media (max-width: 48rem) {
  .ui-color-editor__canvas {
    grid-template-columns: 1fr;
  }
}
"#;
