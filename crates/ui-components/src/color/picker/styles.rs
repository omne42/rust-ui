pub const CSS: &str = r#"
.ui-color-picker {
  display: inline-grid;
  gap: var(--ui-space-xs);
}

.ui-color-picker[data-motion-source="custom"],
.ui-color-picker[data-custom-motion="true"] {
  --ui-color-picker-custom-motion: 1;
}

.ui-color-picker__trigger {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  min-inline-size: 11rem;
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 12%);
  border-radius: var(--ui-radius-sm);
  padding: calc(var(--ui-space-2xs) + 1px) var(--ui-space-xs);
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 3%);
  color: var(--ui-fg);
  cursor: pointer;
}

.ui-color-picker__trigger:hover {
  border-color: color-mix(in oklch, var(--ui-accent), var(--ui-border) 48%);
}

.ui-color-picker__trigger:focus-visible {
  outline: 2px solid color-mix(in oklch, var(--ui-accent), transparent 62%);
  outline-offset: 2px;
}

.ui-color-picker__swatch {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-color-picker__label {
  font-size: 0.875rem;
  font-weight: 600;
}

.ui-color-picker__value {
  margin-inline-start: auto;
  font-size: 0.75rem;
  color: var(--ui-fg-muted);
  font-variant-numeric: tabular-nums;
}

.ui-color-picker__panel {
  min-inline-size: 14rem;
  border-radius: var(--ui-radius-md);
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 20%);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-md);
}

.ui-color-picker__content {
  display: grid;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-sm);
}

.ui-color-picker--open .ui-color-picker__trigger,
.ui-color-picker[data-open="true"] .ui-color-picker__trigger,
.ui-color-picker[data-state="open"] .ui-color-picker__trigger {
  border-color: color-mix(in oklch, var(--ui-accent), var(--ui-border) 24%);
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent), transparent 84%);
}

.ui-color-picker--disabled,
.ui-color-picker[data-disabled="true"] {
  opacity: 0.62;
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
