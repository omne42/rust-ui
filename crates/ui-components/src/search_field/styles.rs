pub const CSS: &str = r#"
.ui-search-field {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-search-field__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-search-field__control {
  width: 100%;
  box-sizing: border-box;

  display: flex;
  align-items: center;
  gap: var(--ui-space-xs);

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);

  transition:
    border-color 200ms ease,
    background-color 200ms ease,
    outline-color 200ms ease;
}

.ui-search-field__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--ui-fg-muted);
}

.ui-search-field__icon svg {
  width: calc(var(--ui-space-md) + var(--ui-space-xs));
  height: calc(var(--ui-space-md) + var(--ui-space-xs));
}

.ui-search-field__input {
  flex: 1;
  min-width: 0;

  border: 0;
  outline: none;
  background: transparent;
  color: inherit;
  font: inherit;
  padding: 0;

  -webkit-tap-highlight-color: transparent;
}

.ui-search-field__input::placeholder {
  color: var(--ui-fg-muted);
}

.ui-search-field__input::-webkit-search-cancel-button {
  -webkit-appearance: none;
  display: none;
}

.ui-search-field__clear {
  width: calc(var(--ui-space-lg) + var(--ui-space-sm));
  height: calc(var(--ui-space-lg) + var(--ui-space-sm));

  display: inline-flex;
  align-items: center;
  justify-content: center;

  border: 0;
  border-radius: calc(var(--ui-space-lg) + var(--ui-space-sm));
  background: transparent;
  color: var(--ui-fg-muted);

  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-search-field__clear:hover {
  color: var(--ui-fg);
}

.ui-search-field__clear svg {
  width: calc(var(--ui-space-md) + var(--ui-space-xs));
  height: calc(var(--ui-space-md) + var(--ui-space-xs));
  pointer-events: none;
}

.ui-search-field--focus-visible .ui-search-field__control {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-search-field--invalid .ui-search-field__control {
  border-color: var(--ui-danger);
}

.ui-search-field--invalid.ui-search-field--focus-visible .ui-search-field__control {
  outline-color: var(--ui-danger);
}

.ui-search-field__description,
.ui-search-field__error {
  font-size: 12px;
  line-height: 1.3;
}

.ui-search-field__description {
  color: var(--ui-fg-muted);
}

.ui-search-field__error {
  color: var(--ui-danger);
}

.ui-search-field--disabled .ui-search-field__control {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}

.ui-search-field__input:disabled {
  cursor: not-allowed;
}

.ui-search-field__clear:disabled {
  cursor: not-allowed;
}
"#;
