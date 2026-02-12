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

@media (prefers-reduced-motion: reduce) {
  .ui-text-field__input {
    transition: none;
  }
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
