pub const CSS: &str = r#"
.ui-number-field {
  --ui-number-field-label-font-size: var(--ui-font-size-150);
  --ui-number-field-label-line-height: var(--ui-line-height-150);
  --ui-number-field-input-font-size: var(--ui-font-size-150);
  --ui-number-field-input-line-height: var(--ui-line-height-150);
  --ui-number-field-meta-font-size: var(--ui-font-size-100);
  --ui-number-field-meta-line-height: var(--ui-line-height-100);

  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-number-field__label {
  font-size: var(--ui-number-field-label-font-size);
  line-height: var(--ui-number-field-label-line-height);
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-number-field__control {
  display: flex;
  align-items: center;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  padding: 0 var(--ui-space-sm);
  gap: var(--ui-space-xs);
}

.ui-number-field--focus-visible .ui-number-field__control {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-number-field__input {
  flex: 1;
  min-width: 0;
  border: 0;
  outline: none;
  background: transparent;
  color: var(--ui-fg);
  font-size: var(--ui-number-field-input-font-size);
  line-height: var(--ui-number-field-input-line-height);
  height: 36px;
}

.ui-number-field__stepper {
  display: inline-flex;
  gap: var(--ui-space-xs);
}

.ui-number-field__description {
  font-size: var(--ui-number-field-meta-font-size);
  line-height: var(--ui-number-field-meta-line-height);
  color: var(--ui-fg-muted);
}

.ui-number-field__error {
  font-size: var(--ui-number-field-meta-font-size);
  line-height: var(--ui-number-field-meta-line-height);
  color: var(--ui-danger);
}

.ui-number-field--invalid .ui-number-field__control {
  border-color: color-mix(in oklch, var(--ui-danger) 40%, var(--ui-border));
}

.ui-number-field--disabled {
  opacity: 0.5;
}

.ui-number-field--disabled .ui-number-field__control {
  pointer-events: none;
}
"#;
