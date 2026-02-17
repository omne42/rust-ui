pub const CSS: &str = r#"
.ui-field-button {
  min-width: 2rem;
  min-height: 2rem;
}

.ui-field-button--quiet,
.ui-field-button[data-quiet="true"] {
  background: transparent;
  border-color: transparent;
}

.ui-field-button--invalid,
.ui-field-button[data-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 55%, var(--ui-border));
  color: color-mix(in oklab, var(--ui-danger) 70%, var(--ui-fg));
}

.ui-field-button.is-hovered,
.ui-field-button[data-hovered="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 80%, var(--ui-accent) 20%);
}

.ui-field-button.is-active,
.ui-field-button[data-active="true"],
.ui-field-button[data-pressed="true"] {
  transform: scale(0.98);
}

.ui-field-button--disabled,
.ui-field-button[data-disabled="true"] {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-field-button--focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 45%, transparent);
  outline-offset: 2px;
}

.ui-field-button[data-focus-visible="true"] {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 45%, transparent);
  outline-offset: 2px;
}

.ui-field-button__label {
  min-width: 0;
  white-space: nowrap;
  font-size: 0.875rem;
  line-height: 1;
}

.ui-field-button--custom-class,
.ui-field-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}
"#;
