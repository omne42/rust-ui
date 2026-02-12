pub const CSS: &str = r#"
.ui-date-picker {
  display: grid;
  gap: var(--ui-space-2xs);
  width: min(100%, 24rem);
}

.ui-date-picker[data-motion-source="custom"],
.ui-date-picker--custom-motion,
.ui-date-picker[data-custom-motion="true"] {
  --ui-date-picker-custom-motion: 1;
}

.ui-date-picker--tone-default,
.ui-date-picker[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-date-picker--tone-quiet,
.ui-date-picker[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-date-picker--tone-strong,
.ui-date-picker[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 88%, black 12%);
}

.ui-date-picker--open .ui-date-picker__trigger,
.ui-date-picker[data-open="true"] .ui-date-picker__trigger {
  border-color: color-mix(in oklab, var(--ui-accent) 48%, var(--ui-border) 52%);
  box-shadow: 0 0 0 2px color-mix(in oklab, var(--ui-accent-soft) 52%, transparent);
}

.ui-date-picker--disabled,
.ui-date-picker[data-disabled="true"] {
  opacity: 0.7;
}

.ui-date-picker--has-value,
.ui-date-picker[data-has-value="true"] {
  font-variant-numeric: tabular-nums;
}

.ui-date-picker[data-class-source="custom"],
.ui-date-picker--custom-class,
.ui-date-picker[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-date-picker__trigger-wrap {
  display: block;
}

.ui-date-picker__trigger {
  width: 100%;
  justify-content: space-between;
}

.ui-date-picker__panel {
  display: grid;
  gap: var(--ui-space-xs);
  min-width: 18rem;
}

.ui-date-picker__calendar {
  width: 100%;
}
"#;
