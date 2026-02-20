pub const CSS: &str = r#"
.ui-field-button {
  min-width: var(--ui-button-size-s-icon);
  min-height: var(--ui-button-size-s-height);
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
  transform: scale(var(--ui-button-active-scale, 0.98));
}

.ui-field-button--disabled,
.ui-field-button[data-disabled="true"] {
  opacity: var(--ui-button-disabled-opacity, 0.5);
  cursor: not-allowed;
}

.ui-field-button--focus-visible,
.ui-field-button.ui-button--focus-visible {
  outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-button-focus-outline-offset);
}

.ui-field-button[data-focus-visible="true"] {
  outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-button-focus-outline-offset);
}

.ui-field-button__label,
.ui-field-button .ui-button__label {
  min-width: 0;
  white-space: nowrap;
  font-size: var(--ui-button-size-s-font-size, var(--ui-button-font-size));
  line-height: var(--ui-button-size-s-line-height);
}

.ui-field-button--custom-class,
.ui-field-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}
"#;
