pub const CSS: &str = r#"
.ui-color-field {
  --ui-color-field-gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-field-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-field-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-color-field-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  --ui-color-field-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-color-field-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  --ui-color-field-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-field-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-color-field-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-field-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-field-success: var(--ui-success, var(--ui-accent, var(--ui-fallback-accent)));
  --ui-color-field-danger: var(--ui-danger, var(--ui-fallback-danger));
  --ui-color-field-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-color-field-preview-size: var(
    --ui-color-swatch-size-sm,
    var(--ui-fallback-color-swatch-size-sm)
  );
  --ui-color-field-padding-y: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-field-padding-x: var(--ui-space-sm, var(--ui-fallback-space-sm));

  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-color-field-gap);
  min-inline-size: min(
    100%,
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))
  );
}

.ui-color-field__label {
  color: var(--ui-color-field-fg-muted);
  font-size: var(--ui-color-field-font-size-100);
  font-weight: 600;
  line-height: var(--ui-color-field-line-height-100);
}

.ui-color-field__control {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-color-field-gap);
  min-inline-size: 0;
}

.ui-color-field__preview {
  inline-size: var(--ui-color-field-preview-size);
  block-size: var(--ui-color-field-preview-size);
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-color-field__input {
  inline-size: 100%;
  min-inline-size: 0;
  border: var(--ui-color-field-border-width) solid
    color-mix(in oklab, var(--ui-color-field-fg-muted) 32%, transparent);
  border-radius: var(--ui-color-field-radius);
  background: color-mix(in oklab, var(--ui-color-field-bg) 96%, transparent);
  color: var(--ui-color-field-fg);
  font-size: var(--ui-color-field-font-size-150);
  line-height: var(--ui-color-field-line-height-150);
  padding: var(--ui-color-field-padding-y) var(--ui-color-field-padding-x);
}

.ui-color-field__input::placeholder {
  color: color-mix(in oklab, var(--ui-color-field-fg-muted) 78%, transparent);
}

.ui-color-field__input:focus-visible {
  outline: calc(var(--ui-color-field-border-width) * 2) solid
    color-mix(in oklab, var(--ui-color-field-accent) 82%, transparent);
  outline-offset: var(--ui-color-field-border-width);
}

.ui-color-field__clear {
  border: var(--ui-color-field-border-width) solid
    color-mix(in oklab, var(--ui-color-field-fg-muted) 26%, transparent);
  border-radius: var(--ui-color-field-radius);
  background: color-mix(in oklab, var(--ui-color-field-bg) 98%, transparent);
  color: var(--ui-color-field-fg-muted);
  font-size: var(--ui-color-field-font-size-100);
  line-height: var(--ui-color-field-line-height-100);
  padding: var(--ui-color-field-padding-y) var(--ui-color-field-padding-x);
  cursor: pointer;
}

.ui-color-field__clear:hover {
  color: var(--ui-color-field-fg);
}

.ui-color-field[data-state="valid"] .ui-color-field__input,
.ui-color-field[data-valid="true"] .ui-color-field__input {
  border-color: color-mix(in oklab, var(--ui-color-field-success) 54%, transparent);
}

.ui-color-field[data-state="invalid"] .ui-color-field__input,
.ui-color-field[data-invalid="true"] .ui-color-field__input,
.ui-color-field__input[aria-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-color-field-danger) 58%, transparent);
}

.ui-color-field--disabled,
.ui-color-field[data-disabled="true"] {
  opacity: 0.68;
}

.ui-color-field--disabled .ui-color-field__input,
.ui-color-field[data-disabled="true"] .ui-color-field__input,
.ui-color-field--disabled .ui-color-field__clear,
.ui-color-field[data-disabled="true"] .ui-color-field__clear {
  cursor: not-allowed;
}

.ui-color-field--custom-class,
.ui-color-field[data-custom-class="true"],
.ui-color-field[data-class-source="custom"] {
  --ui-color-field-custom-class: 1;
}
"#;
