pub const CSS: &str = r#"
.ui-color-swatch-picker {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  --ui-color-swatch-picker-transition-ms: 140ms;
  --ui-color-swatch-picker-focus-ring-width: 5px;
}

.ui-color-swatch-picker__list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-space-xs);
}

.ui-color-swatch-picker__option {
  appearance: none;
  border: none;
  background: transparent;
  margin: 0;
  padding: 0;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--ui-radius-sm);
  outline: none;
  transition: box-shadow var(--ui-color-swatch-picker-transition-ms) ease;
}

.ui-color-swatch-picker__option[data-selected="true"] {
  box-shadow:
    0 0 0 1px color-mix(in oklab, var(--ui-bg) 80%, transparent),
    0 0 0 3px color-mix(in oklab, var(--ui-accent) 78%, transparent);
}

.ui-color-swatch-picker__option:focus-visible,
.ui-color-swatch-picker__option[data-selected="true"]:focus-visible {
  box-shadow:
    0 0 0 1px color-mix(in oklab, var(--ui-bg) 80%, transparent),
    0 0 0 3px color-mix(in oklab, var(--ui-accent) 84%, transparent),
    0 0 0 var(--ui-color-swatch-picker-focus-ring-width) color-mix(in oklab, var(--ui-accent) 32%, transparent);
}

.ui-color-swatch-picker__option[data-disabled="true"],
.ui-color-swatch-picker--disabled .ui-color-swatch-picker__option {
  cursor: not-allowed;
  opacity: 0.6;
}

.ui-color-swatch-picker[data-empty="true"] .ui-color-swatch-picker__list {
  min-block-size: 1.75rem;
}

.ui-color-swatch-picker--custom-class,
.ui-color-swatch-picker[data-custom-class="true"] {
  --ui-color-swatch-picker-has-custom-class: 1;
}

.ui-color-swatch-picker[data-motion-source="custom"],
.ui-color-swatch-picker[data-custom-motion="true"] {
  --ui-color-swatch-picker-motion-source: custom;
}
"#;
