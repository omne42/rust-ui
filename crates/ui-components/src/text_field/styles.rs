pub const CSS: &str = r#"
.ui-text-field {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-text-field__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-text-field__input {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;

  transition:
    border-color 200ms ease,
    background-color 200ms ease,
    outline-color 200ms ease;
}

.ui-text-field__input::placeholder {
  color: var(--ui-fg-muted);
}

.ui-text-field--focus-visible .ui-text-field__input {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-text-field--invalid .ui-text-field__input {
  border-color: var(--ui-danger);
}

.ui-text-field--invalid.ui-text-field--focus-visible .ui-text-field__input {
  outline-color: var(--ui-danger);
}

.ui-text-field__description,
.ui-text-field__error {
  font-size: 12px;
  line-height: 1.3;
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
"#;
