pub const CSS: &str = r#"
.ui-textarea {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-textarea[data-state="disabled"] {
  opacity: 0.72;
}

.ui-textarea[data-state="invalid"] {
  --ui-textarea-invalid: 1;
}

.ui-textarea[data-state="readonly"] {
  --ui-textarea-readonly: 1;
}

.ui-textarea[data-value="filled"] {
  --ui-textarea-has-value: 1;
}

.ui-textarea[data-requirement="required"] {
  --ui-textarea-required: 1;
}

.ui-textarea[data-label-source="custom"] {
  --ui-textarea-label-source: custom;
}

.ui-textarea[data-description-source="custom"] {
  --ui-textarea-description-source: custom;
}

.ui-textarea[data-error-source="custom"] {
  --ui-textarea-error-source: custom;
}

.ui-textarea[data-placeholder-source="custom"] {
  --ui-textarea-placeholder-source: custom;
}

.ui-textarea[data-rows-source="custom"] {
  --ui-textarea-rows-source: custom;
}

.ui-textarea--custom-class,
.ui-textarea[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-textarea__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-textarea__textarea {
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
  .ui-textarea__textarea {
    transition: none;
  }
}

.ui-textarea__textarea::placeholder {
  color: var(--ui-fg-muted);
}

.ui-textarea--focus-visible .ui-textarea__textarea {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-textarea--invalid .ui-textarea__textarea {
  border-color: var(--ui-danger);
}

.ui-textarea--invalid.ui-textarea--focus-visible .ui-textarea__textarea {
  outline-color: var(--ui-danger);
}

.ui-textarea__description,
.ui-textarea__error {
  font-size: 12px;
  line-height: 1.3;
}

.ui-textarea__description {
  color: var(--ui-fg-muted);
}

.ui-textarea__error {
  color: var(--ui-danger);
}

.ui-textarea__textarea:disabled {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}
"#;
