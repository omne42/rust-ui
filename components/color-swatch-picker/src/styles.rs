pub const CSS: &str = r#"
.ui-color-swatch-picker {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-swatch-picker-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-swatch-picker-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-swatch-picker-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-swatch-picker-selected-border-width: var(--ui-color-swatch-border-width, var(--ui-fallback-color-swatch-border-width));
  --ui-color-swatch-picker-selected-ring-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));
  --ui-color-swatch-picker-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
  --ui-color-swatch-picker-transition-ms: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));
  --ui-color-swatch-picker-transition-easing: var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
  --ui-color-swatch-picker-focus-ring-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));
}

.ui-color-swatch-picker__list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
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
  border-radius: var(--ui-color-swatch-picker-radius);
  outline: none;
  transition: box-shadow var(--ui-color-swatch-picker-transition-ms) var(--ui-color-swatch-picker-transition-easing);
}

.ui-color-swatch-picker__option[data-selected="true"] {
  box-shadow:
    0 0 0 var(--ui-color-swatch-picker-selected-border-width) color-mix(in oklab, var(--ui-color-swatch-picker-bg) 80%, transparent),
    0 0 0 var(--ui-color-swatch-picker-selected-ring-width) color-mix(in oklab, var(--ui-color-swatch-picker-accent) 78%, transparent);
}

.ui-color-swatch-picker__option:focus-visible,
.ui-color-swatch-picker__option[data-selected="true"]:focus-visible {
  box-shadow:
    0 0 0 var(--ui-color-swatch-picker-selected-border-width) color-mix(in oklab, var(--ui-color-swatch-picker-bg) 80%, transparent),
    0 0 0 var(--ui-color-swatch-picker-selected-ring-width) color-mix(in oklab, var(--ui-color-swatch-picker-accent) 84%, transparent),
    0 0 0 var(--ui-color-swatch-picker-focus-ring-width) color-mix(in oklab, var(--ui-color-swatch-picker-accent) 32%, transparent);
}

.ui-color-swatch-picker__option[data-disabled="true"],
.ui-color-swatch-picker--disabled .ui-color-swatch-picker__option {
  cursor: not-allowed;
  opacity: var(--ui-color-swatch-picker-disabled-opacity);
}

.ui-color-swatch-picker[data-empty="true"] .ui-color-swatch-picker__list {
  min-block-size: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
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
