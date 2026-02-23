pub const CSS: &str = r#"
.ui-radio-group {
  display: flex;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-radio-group--vertical {
  flex-direction: column;
}

.ui-radio-group--horizontal {
  flex-direction: row;
  flex-wrap: wrap;
}

.ui-radio-group__label {
  --ui-radio-label-font-size: var(--ui-font-size-100);
  font-size: var(--ui-radio-label-font-size, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 500;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-radio {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-xs, var(--ui-fallback-space-xs))
    var(--ui-space-sm, var(--ui-fallback-space-sm));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;
  background: transparent;
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;

  transform: scale(var(--ui-radio-scale, var(--ui-fallback-image-zoom-initial)));
  transform-origin: center;
  will-change: transform;

  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-radio[data-motion-source="custom"],
.ui-radio[data-custom-motion="true"] {
  --ui-radio-custom-motion: 1;
}

.ui-radio:not(:disabled) {
  cursor: pointer;
}

.ui-radio:disabled {
  pointer-events: none;
  opacity: 0.6;
}

.ui-radio--focus-visible {
  --ui-radio-focus-outline-width: var(--ui-button-focus-outline-width);
  --ui-radio-focus-outline-offset: var(--ui-button-focus-outline-offset);
  outline: var(--ui-radio-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-radio-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
}

.ui-radio__indicator {
  --ui-radio-indicator-size: var(--ui-icon-size-100);
  --ui-radio-radius-full: var(--ui-button-radius-full);
  --ui-radio-indicator-border-space-2xs: var(--ui-space-2xs);
  width: var(--ui-radio-indicator-size, var(--ui-fallback-icon-size-100));
  height: var(--ui-radio-indicator-size, var(--ui-fallback-icon-size-100));
  border-radius: var(--ui-radio-radius-full, var(--ui-fallback-button-radius-full));
  box-sizing: border-box;
  border: calc(var(--ui-radio-indicator-border-space-2xs, var(--ui-fallback-space-2xs)) / 2) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ui-radio[data-checked=\"true\"] .ui-radio__indicator {
  border-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-radio__dot {
  width: calc(var(--ui-radio-indicator-size, var(--ui-fallback-icon-size-100)) / 2);
  height: calc(var(--ui-radio-indicator-size, var(--ui-fallback-icon-size-100)) / 2);
  border-radius: var(--ui-radio-radius-full, var(--ui-fallback-button-radius-full));
  background: var(--ui-accent, var(--ui-fallback-accent));
  opacity: 0;
  transform: scale(0.5);
}

.ui-radio[data-checked=\"true\"] .ui-radio__dot {
  opacity: 1;
  transform: scale(1);
}
"#;
