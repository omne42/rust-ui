pub const CSS: &str = r#"
.ui-date-range-picker {
  display: grid;
  gap: var(--ui-space-xs);
  width: min(100%, 34rem);
  padding: var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
}

.ui-date-range-picker--tone-default,
.ui-date-range-picker[data-tone="default"] {
  background: var(--ui-bg);
}

.ui-date-range-picker--tone-quiet,
.ui-date-range-picker[data-tone="quiet"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 65%, var(--ui-bg) 35%);
}

.ui-date-range-picker--tone-strong,
.ui-date-range-picker[data-tone="strong"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 24%, var(--ui-bg) 76%);
  border-color: color-mix(in oklab, var(--ui-accent) 32%, var(--ui-border) 68%);
}

.ui-date-range-picker--disabled,
.ui-date-range-picker[data-disabled="true"] {
  opacity: 0.72;
}

.ui-date-range-picker--partial,
.ui-date-range-picker[data-state="partial"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-date-range-picker--has-full-value,
.ui-date-range-picker[data-has-full-value="true"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-accent) 36%, transparent);
}

.ui-date-range-picker--invalid-range,
.ui-date-range-picker[data-invalid-range="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 56%, var(--ui-border) 44%);
}

.ui-date-range-picker--custom-class,
.ui-date-range-picker[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-date-range-picker__fields {
  display: grid;
  gap: var(--ui-space-sm);
  grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
}

.ui-date-range-picker__field {
  display: grid;
  gap: var(--ui-space-2xs);
}

.ui-date-range-picker__field-label {
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
  font-weight: 600;
  color: var(--ui-fg-muted);
}

.ui-date-range-picker__picker {
  width: 100%;
}

.ui-date-range-picker__hint {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}
"#;
