pub const CSS: &str = r#"
.ui-text-area {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-text-area__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-text-area__textarea {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;

  resize: vertical;

  transition:
    border-color 200ms ease,
    background-color 200ms ease,
    outline-color 200ms ease;
}

@media (prefers-reduced-motion: reduce) {
  .ui-text-area__textarea {
    transition: none;
  }
}

.ui-text-area__textarea::placeholder {
  color: var(--ui-fg-muted);
}

.ui-text-area--focus-visible .ui-text-area__textarea {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-text-area--invalid .ui-text-area__textarea {
  border-color: var(--ui-danger);
}

.ui-text-area--invalid.ui-text-area--focus-visible .ui-text-area__textarea {
  outline-color: var(--ui-danger);
}

.ui-text-area__description,
.ui-text-area__error {
  font-size: 12px;
  line-height: 1.3;
}

.ui-text-area__description {
  color: var(--ui-fg-muted);
}

.ui-text-area__error {
  color: var(--ui-danger);
}

.ui-text-area__textarea:disabled {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}
"#;
