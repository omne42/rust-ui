pub const CSS: &str = r#"
.ui-text-area {
  --ui-text-area-label-font-size: var(--ui-font-size-150);
  --ui-text-area-meta-font-size: var(--ui-font-size-100);
  --ui-text-area-focus-outline-width: var(--ui-button-focus-outline-width);
  --ui-text-area-focus-outline-offset: var(--ui-button-focus-outline-offset);

  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-text-area[data-state="disabled"] {
  opacity: 0.72;
}

.ui-text-area[data-state="invalid"] {
  --ui-text-area-invalid: 1;
}

.ui-text-area[data-state="readonly"] {
  --ui-text-area-readonly: 1;
}

.ui-text-area[data-value="filled"] {
  --ui-text-area-has-value: 1;
}

.ui-text-area[data-requirement="required"] {
  --ui-text-area-required: 1;
}

.ui-text-area[data-value-control-mode="controlled"] {
  --ui-text-area-value-control-mode: controlled;
}

.ui-text-area[data-default-value-source="custom"] {
  --ui-text-area-default-value-source: custom;
}

.ui-text-area[data-value-change-source="on_value_change"] {
  --ui-text-area-value-change-source: on_value_change;
}

.ui-text-area[data-value-change-source="set_value"] {
  --ui-text-area-value-change-source: set_value;
}

.ui-text-area[data-label-source="custom"] {
  --ui-text-area-label-source: custom;
}

.ui-text-area[data-description-source="custom"] {
  --ui-text-area-description-source: custom;
}

.ui-text-area[data-error-source="custom"] {
  --ui-text-area-error-source: custom;
}

.ui-text-area[data-placeholder-source="custom"] {
  --ui-text-area-placeholder-source: custom;
}

.ui-text-area[data-rows-source="custom"] {
  --ui-text-area-rows-source: custom;
}

.ui-text-area[data-class-source="custom"],
.ui-text-area[data-custom-class="true"],
.ui-text-area--custom-class {
  border-radius: inherit;
}

.ui-text-area__label {
  font-size: var(--ui-text-area-label-font-size);
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
    border-color var(--ui-text-area-motion-duration) var(--ui-text-area-motion-easing),
    background-color var(--ui-text-area-motion-duration) var(--ui-text-area-motion-easing),
    outline-color var(--ui-text-area-motion-duration) var(--ui-text-area-motion-easing);
}

@media (prefers-reduced-motion: reduce) {
  .ui-text-area__textarea {
    --ui-text-area-motion-duration: 1ms;
  }
}

.ui-text-area__textarea::placeholder {
  color: var(--ui-fg-muted);
}

.ui-text-area--focus-visible .ui-text-area__textarea {
  outline: var(--ui-text-area-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-text-area-focus-outline-offset);
}

.ui-text-area--invalid .ui-text-area__textarea {
  border-color: var(--ui-danger);
}

.ui-text-area--invalid.ui-text-area--focus-visible .ui-text-area__textarea {
  outline-color: var(--ui-danger);
}

.ui-text-area__description,
.ui-text-area__error {
  font-size: var(--ui-text-area-meta-font-size);
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
