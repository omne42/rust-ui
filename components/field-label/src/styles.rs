pub const CSS: &str = r#"
.ui-field-label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  color: var(--ui-fg);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 500;
}

.ui-field-label--tone-default,
.ui-field-label[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-field-label--tone-muted,
.ui-field-label[data-tone="muted"] {
  color: var(--ui-fg-muted);
  font-weight: 450;
}

.ui-field-label--tone-strong,
.ui-field-label[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 92%, black 8%);
  font-weight: 600;
  letter-spacing: 0.01em;
}

.ui-field-label--required,
.ui-field-label[data-required="true"] {
  letter-spacing: 0.01em;
}

.ui-field-label--disabled,
.ui-field-label[data-disabled="true"] {
  color: color-mix(in oklab, var(--ui-fg-muted) 76%, var(--ui-bg) 24%);
}

.ui-field-label--for,
.ui-field-label[data-has-for="true"] {
  cursor: pointer;
}

.ui-field-label--text-custom,
.ui-field-label[data-text-source="custom"] {
  text-decoration: underline;
  text-underline-offset: 0.12em;
}

.ui-field-label--indicator-custom,
.ui-field-label[data-indicator-source="custom"] {
  gap: var(--ui-space-xs);
}

.ui-field-label--aria-custom,
.ui-field-label[data-aria-source="custom"] {
  outline-offset: 1px;
}

.ui-field-label--custom-class,
.ui-field-label[data-custom-class="true"] {
  outline: 1px dashed color-mix(in oklab, var(--ui-accent) 30%, transparent);
  outline-offset: 2px;
}

.ui-field-label__text {
  display: inline-flex;
  align-items: center;
}

.ui-field-label__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger);
  font-size: 0.85em;
}
"#;
