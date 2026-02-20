pub const CSS: &str = r#"
.ui-input {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);

  --ui-input-label-font-size: var(--ui-font-size-150);
  --ui-input-label-line-height: var(--ui-line-height-150);
  --ui-input-meta-font-size: var(--ui-font-size-100);
  --ui-input-meta-line-height: var(--ui-line-height-100);
  --ui-input-control-font-size: var(--ui-font-size-150);
  --ui-input-control-line-height: var(--ui-line-height-150);
  --ui-input-height: 36px;
  --ui-input-padding-x: var(--ui-space-md);
  --ui-input-padding-y: var(--ui-space-sm);
}

.ui-input[data-motion-source="custom"],
.ui-input[data-custom-motion="true"] {
  --ui-input-custom-motion: 1;
}

.ui-input--size-sm {
  --ui-input-height: 32px;
  --ui-input-padding-x: var(--ui-space-sm);
  --ui-input-padding-y: var(--ui-space-xs);
}

.ui-input--size-lg {
  --ui-input-height: 40px;
  --ui-input-padding-x: var(--ui-space-lg);
  --ui-input-padding-y: var(--ui-space-sm);
}

.ui-input__label {
  font-size: var(--ui-input-label-font-size);
  line-height: var(--ui-input-label-line-height);
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-input__label--hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.ui-input__control {
  display: flex;
  align-items: center;
  gap: var(--ui-space-sm);

  height: var(--ui-input-height);
  padding: 0 var(--ui-input-padding-x);
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
}

.ui-input--variant-flat .ui-input__control {
  background: var(--ui-bg-muted);
}

.ui-input--variant-underlined .ui-input__control {
  border-left: 0;
  border-right: 0;
  border-top: 0;
  border-radius: 0;
  padding: 0;
  background: transparent;
}

.ui-input__start,
.ui-input__end {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  color: var(--ui-fg-muted);
}

.ui-input__input {
  flex: 1 1 auto;
  min-width: 0;
  height: 100%;
  border: 0;
  outline: none;
  background: transparent;
  color: inherit;
  font: inherit;
  font-size: var(--ui-input-control-font-size);
  line-height: var(--ui-input-control-line-height);
  padding: var(--ui-input-padding-y) 0;
}

.ui-input__input::placeholder {
  color: var(--ui-fg-muted);
}

.ui-input__clear {
  --ui-input-clear-opacity: 0;
  --ui-input-clear-scale: 0.85;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: calc(var(--ui-input-height) - 12px);
  height: calc(var(--ui-input-height) - 12px);
  border-radius: 999px;
  border: 0;
  background: transparent;
  color: var(--ui-fg-muted);
  cursor: pointer;
  opacity: var(--ui-input-clear-opacity, 0);
  transform: scale(var(--ui-input-clear-scale));
  will-change: transform, opacity;
  pointer-events: none;
}

.ui-input__clear[data-visible=\"true\"] {
  --ui-input-clear-opacity: 1;
  --ui-input-clear-scale: 1;
  pointer-events: auto;
}

.ui-input__clear:hover {
  color: var(--ui-fg);
}

.ui-input__clear svg {
  width: 16px;
  height: 16px;
}

.ui-input--focus-visible .ui-input__control {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-input--invalid .ui-input__control {
  border-color: var(--ui-danger);
}

.ui-input--invalid.ui-input--focus-visible .ui-input__control {
  outline-color: var(--ui-danger);
}

.ui-input--disabled .ui-input__control {
  opacity: 0.6;
  background: var(--ui-bg-muted);
  cursor: not-allowed;
}

.ui-input__description,
.ui-input__error {
  font-size: var(--ui-input-meta-font-size);
  line-height: var(--ui-input-meta-line-height);
}

.ui-input__description {
  color: var(--ui-fg-muted);
}

.ui-input__error {
  color: var(--ui-danger);
}
"#;
