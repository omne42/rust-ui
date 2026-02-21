pub const CSS: &str = r#"
.ui-native-select {
  position: relative;
  display: inline-flex;
  align-items: center;
  min-width: calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 5.5);
}

.ui-native-select__control {
  width: 100%;
  appearance: none;
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  transition:
    border-color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    box-shadow var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    background-color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
  padding-inline-end: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg))
      + var(--ui-space-xs, var(--ui-fallback-space-xs))
  );
}

.ui-native-select__control:focus-visible {
  outline: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2) solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2);
  border-color: color-mix(
    in oklch,
    var(--ui-focus-ring, var(--ui-fallback-focus-ring)) 60%,
    var(--ui-border, var(--ui-fallback-border))
  );
}

.ui-native-select__control:not(:disabled):hover {
  border-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 36%,
    var(--ui-border, var(--ui-fallback-border))
  );
  background: color-mix(
    in oklch,
    var(--ui-bg, var(--ui-fallback-bg)) 92%,
    var(--ui-accent, var(--ui-fallback-accent))
  );
}

.ui-native-select__control:not(:disabled):active {
  border-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 52%,
    var(--ui-border, var(--ui-fallback-border))
  );
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-native-select__control:disabled {
  cursor: not-allowed;
  opacity: 0.65;
}

.ui-native-select__indicator {
  position: absolute;
  right: var(--ui-space-sm, var(--ui-fallback-space-sm));
  top: 50%;
  transform: translateY(-50%);
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  pointer-events: none;
}

.ui-native-select--size-sm .ui-native-select__control {
  min-height: calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.75);
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  padding-block: 0;
  padding-inline-start: var(--ui-space-xs, var(--ui-fallback-space-xs));
  padding-inline-end: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg))
      + var(--ui-space-xs, var(--ui-fallback-space-xs))
  );
}

.ui-native-select--size-md .ui-native-select__control {
  min-height: calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.85);
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  padding-block: 0;
  padding-inline-start: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding-inline-end: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg))
      + var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
}

.ui-native-select--size-lg .ui-native-select__control {
  min-height: calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.95);
  font-size: var(--ui-button-size-l-font-size, var(--ui-fallback-button-size-l-font-size));
  line-height: var(--ui-button-size-l-line-height, var(--ui-fallback-button-size-l-line-height));
  padding-block: 0;
  padding-inline-start: var(--ui-space-md, var(--ui-fallback-space-md));
  padding-inline-end: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg))
      + var(--ui-space-md, var(--ui-fallback-space-md))
  );
}

.ui-native-select--invalid .ui-native-select__control {
  border-color: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 64%,
    var(--ui-border, var(--ui-fallback-border))
  );
}

.ui-native-select--selected .ui-native-select__control {
  border-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 52%,
    var(--ui-border, var(--ui-fallback-border))
  );
}

.ui-native-select--empty .ui-native-select__control {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-native-select--disabled .ui-native-select__control {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  color: color-mix(
    in oklch,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 86%,
    var(--ui-bg, var(--ui-fallback-bg))
  );
}
"#;
