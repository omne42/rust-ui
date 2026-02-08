pub const CSS: &str = r#"
.ui-color-field {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  min-inline-size: min(100%, 18rem);
}

.ui-color-field__label {
  color: var(--ui-fg-muted);
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1.2;
}

.ui-color-field__control {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  min-inline-size: 0;
}

.ui-color-field__preview {
  inline-size: 1.75rem;
  block-size: 1.75rem;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-color-field__input {
  inline-size: 100%;
  min-inline-size: 0;
  border: 1px solid color-mix(in oklab, var(--ui-fg-muted) 32%, transparent);
  border-radius: var(--ui-radius-sm);
  background: color-mix(in oklab, var(--ui-bg) 96%, transparent);
  color: var(--ui-fg);
  font-size: 0.875rem;
  line-height: 1.3;
  padding: 0.45rem 0.625rem;
}

.ui-color-field__input::placeholder {
  color: color-mix(in oklab, var(--ui-fg-muted) 78%, transparent);
}

.ui-color-field__input:focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 82%, transparent);
  outline-offset: 1px;
}

.ui-color-field__clear {
  border: 1px solid color-mix(in oklab, var(--ui-fg-muted) 26%, transparent);
  border-radius: var(--ui-radius-sm);
  background: color-mix(in oklab, var(--ui-bg) 98%, transparent);
  color: var(--ui-fg-muted);
  font-size: 0.75rem;
  line-height: 1;
  padding: 0.4rem 0.55rem;
  cursor: pointer;
}

.ui-color-field__clear:hover {
  color: var(--ui-fg);
}

.ui-color-field[data-state="valid"] .ui-color-field__input,
.ui-color-field[data-valid="true"] .ui-color-field__input {
  border-color: color-mix(in oklab, var(--ui-success) 54%, transparent);
}

.ui-color-field[data-state="invalid"] .ui-color-field__input,
.ui-color-field[data-invalid="true"] .ui-color-field__input,
.ui-color-field__input[aria-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 58%, transparent);
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
