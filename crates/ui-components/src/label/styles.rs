pub const CSS: &str = r#"
.ui-label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  color: var(--ui-fg);
  font-size: 0.875rem;
  line-height: 1.25;
  font-weight: 500;
}

.ui-label--emphasis-default,
.ui-label[data-emphasis="default"] {
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-label--emphasis-subtle,
.ui-label[data-emphasis="subtle"] {
  font-weight: 450;
  color: var(--ui-fg-muted);
}

.ui-label--emphasis-strong,
.ui-label[data-emphasis="strong"] {
  font-weight: 600;
  color: color-mix(in oklab, var(--ui-fg) 92%, black 8%);
}

.ui-label--required,
.ui-label[data-required="true"] {
  letter-spacing: 0.01em;
}

.ui-label--disabled,
.ui-label[data-disabled="true"] {
  color: color-mix(in oklab, var(--ui-fg-muted) 76%, var(--ui-bg) 24%);
}

.ui-label--for,
.ui-label[data-has-for="true"] {
  cursor: pointer;
}

.ui-label--text-custom,
.ui-label[data-label-source="custom"] {
  text-decoration: underline;
  text-underline-offset: 0.12em;
}

.ui-label--indicator-custom,
.ui-label[data-indicator-source="custom"] {
  gap: var(--ui-space-xs);
}

.ui-label--custom-class,
.ui-label[data-custom-class="true"] {
  outline: 1px dashed color-mix(in oklab, var(--ui-accent) 30%, transparent);
  outline-offset: 2px;
}

.ui-label__text {
  display: inline-flex;
  align-items: center;
}

.ui-label__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger);
  font-size: 0.85em;
}
"#;
