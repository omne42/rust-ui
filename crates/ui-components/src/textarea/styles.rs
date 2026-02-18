pub const CSS: &str = r#"
.ui-textarea {
  --ui-textarea-label-font-size: var(--ui-font-size-150);
  --ui-textarea-meta-font-size: var(--ui-font-size-100);
  --ui-textarea-focus-outline-width: var(--ui-button-focus-outline-width);
  --ui-textarea-focus-outline-offset: var(--ui-button-focus-outline-offset);
  --ui-textarea-control-bg: var(--ui-bg);
  --ui-textarea-control-bg-hover: color-mix(in oklab, var(--ui-bg-muted) 38%, var(--ui-bg) 62%);
  --ui-textarea-control-bg-active: color-mix(in oklab, var(--ui-bg-muted) 52%, var(--ui-bg) 48%);
  --ui-textarea-control-border: var(--ui-border);
  --ui-textarea-control-border-hover: color-mix(in oklab, var(--ui-border) 62%, var(--ui-fg) 38%);
  --ui-textarea-control-border-active: color-mix(in oklab, var(--ui-border) 44%, var(--ui-fg) 56%);
  --ui-textarea-control-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-border) 74%, transparent);

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
  font-size: var(--ui-textarea-label-font-size);
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-textarea__textarea {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-textarea-control-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-textarea-control-bg);
  color: var(--ui-fg);
  font: inherit;
  box-shadow: var(--ui-textarea-control-shadow);

  resize: vertical;

  transition:
    border-color var(--ui-textarea-motion-duration) var(--ui-textarea-motion-easing),
    background-color var(--ui-textarea-motion-duration) var(--ui-textarea-motion-easing),
    outline-color var(--ui-textarea-motion-duration) var(--ui-textarea-motion-easing),
    box-shadow var(--ui-textarea-motion-duration) var(--ui-textarea-motion-easing);
}

.ui-textarea__textarea:hover:not(:disabled):not([readonly]) {
  border-color: var(--ui-textarea-control-border-hover);
  background: var(--ui-textarea-control-bg-hover);
}

.ui-textarea__textarea:active:not(:disabled):not([readonly]) {
  border-color: var(--ui-textarea-control-border-active);
  background: var(--ui-textarea-control-bg-active);
}

@media (prefers-reduced-motion: reduce) {
  .ui-textarea__textarea {
    --ui-textarea-motion-duration: 1ms;
  }
}

.ui-textarea__textarea::placeholder {
  color: var(--ui-fg-muted);
}

.ui-textarea--focus-visible .ui-textarea__textarea {
  outline: var(--ui-textarea-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-textarea-focus-outline-offset);
  border-color: color-mix(in oklab, var(--ui-focus-ring) 32%, var(--ui-textarea-control-border) 68%);
}

.ui-textarea--invalid .ui-textarea__textarea {
  border-color: var(--ui-danger);
}

.ui-textarea--invalid.ui-textarea--focus-visible .ui-textarea__textarea {
  outline-color: var(--ui-danger);
}

.ui-textarea__description,
.ui-textarea__error {
  font-size: var(--ui-textarea-meta-font-size);
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
