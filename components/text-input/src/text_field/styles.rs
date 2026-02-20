pub const CSS: &str = r#"
.ui-text-field {
  --ui-text-field-label-font-size: var(--ui-font-size-150);
  --ui-text-field-label-line-height: var(--ui-line-height-150);
  --ui-text-field-meta-font-size: var(--ui-font-size-100);
  --ui-text-field-meta-line-height: var(--ui-line-height-100);
  --ui-text-field-input-font-size: var(--ui-font-size-150);
  --ui-text-field-input-line-height: var(--ui-line-height-150);
  --ui-text-field-focus-outline-width: var(--ui-button-focus-outline-width);
  --ui-text-field-focus-outline-offset: var(--ui-button-focus-outline-offset);
  --ui-text-field-control-bg: var(--ui-bg);
  --ui-text-field-control-bg-hover: color-mix(in oklab, var(--ui-bg-muted) 38%, var(--ui-bg) 62%);
  --ui-text-field-control-border: var(--ui-border);
  --ui-text-field-control-border-hover: color-mix(in oklab, var(--ui-border) 62%, var(--ui-fg) 38%);
  --ui-text-field-control-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-border) 74%, transparent);

  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-text-field__label {
  font-size: var(--ui-text-field-label-font-size);
  line-height: var(--ui-text-field-label-line-height);
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-text-field__input {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-text-field-control-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-text-field-control-bg);
  color: var(--ui-fg);
  font: inherit;
  font-size: var(--ui-text-field-input-font-size);
  line-height: var(--ui-text-field-input-line-height);
  box-shadow: var(--ui-text-field-control-shadow);

  transition:
    border-color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),
    background-color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),
    outline-color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),
    box-shadow var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing);
}

.ui-text-field__input:hover:not(:disabled):not([readonly]) {
  border-color: var(--ui-text-field-control-border-hover);
  background: var(--ui-text-field-control-bg-hover);
}

@media (prefers-reduced-motion: reduce) {
  .ui-text-field__input {
    transition: none;
  }
}

.ui-text-field__input::placeholder {
  color: var(--ui-fg-muted);
}

.ui-text-field--focus-visible .ui-text-field__input {
  outline: var(--ui-text-field-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-text-field-focus-outline-offset);
  border-color: color-mix(in oklab, var(--ui-focus-ring) 32%, var(--ui-text-field-control-border) 68%);
}

.ui-text-field--invalid .ui-text-field__input {
  border-color: var(--ui-danger);
}

.ui-text-field--invalid.ui-text-field--focus-visible .ui-text-field__input {
  outline-color: var(--ui-danger);
}

.ui-text-field__description,
.ui-text-field__error {
  font-size: var(--ui-text-field-meta-font-size);
  line-height: var(--ui-text-field-meta-line-height);
}

.ui-text-field__description {
  color: var(--ui-fg-muted);
}

.ui-text-field__error {
  color: var(--ui-danger);
}

.ui-text-field__input:disabled {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}
.ui-text-field[data-state="disabled"] .ui-text-field__input {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}

.ui-text-field[data-state="invalid"] .ui-text-field__input {
  border-color: var(--ui-danger);
}

.ui-text-field[data-state="readonly"] .ui-text-field__input {
  background: var(--ui-bg-muted);
}

.ui-text-field[data-value="filled"] {
  --ui-text-field-has-value: 1;
}

.ui-text-field[data-requirement="required"] {
  --ui-text-field-required: 1;
}

.ui-text-field[data-value-control-mode="controlled"] {
  --ui-text-field-value-control-mode: controlled;
}

.ui-text-field[data-default-value-source="custom"] {
  --ui-text-field-default-value-source: custom;
}

.ui-text-field[data-value-change-source="on_value_change"] {
  --ui-text-field-value-change-source: on_value_change;
}

.ui-text-field[data-value-change-source="set_value"] {
  --ui-text-field-value-change-source: set_value;
}

.ui-text-field[data-label-source="custom"] {
  --ui-text-field-label-source: custom;
}

.ui-text-field[data-description-source="custom"] {
  --ui-text-field-description-source: custom;
}

.ui-text-field[data-error-source="custom"] {
  --ui-text-field-error-source: custom;
}

.ui-text-field[data-placeholder-source="custom"] {
  --ui-text-field-placeholder-source: custom;
}

.ui-text-field[data-type-source="custom"] {
  --ui-text-field-type-source: custom;
}

.ui-text-field[data-class-source="custom"],
.ui-text-field[data-custom-class="true"],
.ui-text-field--custom-class {
  border-radius: inherit;
}

"#;
