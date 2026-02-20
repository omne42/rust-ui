pub const CSS: &str = r#"
.ui-date-input-group {
  display: inline-flex;
  align-items: stretch;
  width: fit-content;
  min-height: 2.25rem;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  overflow: hidden;
}

.ui-date-input-group--variant-primary,
.ui-date-input-group[data-variant="primary"] {
  border-color: var(--ui-border);
  background: var(--ui-bg);
}

.ui-date-input-group--variant-secondary,
.ui-date-input-group[data-variant="secondary"] {
  border-color: color-mix(in oklab, var(--ui-border) 64%, var(--ui-accent) 36%);
  background: color-mix(in oklab, var(--ui-bg) 88%, var(--ui-bg-muted) 12%);
}

.ui-date-input-group--full-width,
.ui-date-input-group[data-width="full"] {
  width: 100%;
}

.ui-date-input-group--disabled,
.ui-date-input-group[data-disabled="true"] {
  opacity: 0.62;
}

.ui-date-input-group--invalid,
.ui-date-input-group[data-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-border) 26%);
}

.ui-date-input-group--segmented .ui-date-input-group__segment,
.ui-date-input-group[data-segmented="true"] .ui-date-input-group__segment {
  gap: var(--ui-space-2xs);
}

.ui-date-input-group__prefix,
.ui-date-input-group__suffix {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2.25rem;
  padding-inline: var(--ui-space-sm);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  color: var(--ui-fg-muted);
  background: color-mix(in oklab, var(--ui-bg-muted), var(--ui-bg) 28%);
}

.ui-date-input-group__prefix {
  border-inline-end: 1px solid color-mix(in oklab, var(--ui-border), transparent 28%);
}

.ui-date-input-group__suffix {
  border-inline-start: 1px solid color-mix(in oklab, var(--ui-border), transparent 28%);
}

.ui-date-input-group__input {
  display: flex;
  align-items: stretch;
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment {
  display: flex;
  align-items: stretch;
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment > * {
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment > .ui-date-field,
.ui-date-input-group__segment > .ui-time-field {
  width: 100%;
}

.ui-date-input-group__segment > .ui-date-field .ui-date-field__control,
.ui-date-input-group__segment > .ui-time-field .ui-time-field__control {
  width: 100%;
  border: 0;
  border-radius: 0;
  background: transparent;
}

.ui-date-input-group--custom-class,
.ui-date-input-group[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
