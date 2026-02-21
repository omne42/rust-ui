pub const CSS: &str = r#"
.ui-checkbox {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap));
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  --ui-checkbox-scale: 1;
  --ui-checkbox-indicator: 0;

  border: none;
  padding: 0;
  background: transparent;
  color: inherit;
  font: inherit;

  transform: scale(var(--ui-checkbox-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-checkbox[data-motion-source="custom"],
.ui-checkbox[data-custom-motion="true"] {
  --ui-checkbox-custom-motion: 1;
}

.ui-checkbox[data-state="checked"] {
  --ui-checkbox-indicator: 1;
}

.ui-checkbox[data-enabled="true"] {
  cursor: pointer;
}

.ui-checkbox[data-disabled="true"] {
  opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
  cursor: not-allowed;
}

.ui-checkbox--focus-visible,
.ui-checkbox[data-focus-visible="true"] {
  outline: var(--ui-checkbox-focus-outline-width, var(--ui-fallback-checkbox-focus-outline-width)) solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-checkbox-focus-outline-offset, var(--ui-fallback-checkbox-focus-outline-offset));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}

.ui-checkbox__box {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-accent-fg, var(--ui-fallback-accent-fg));

  box-sizing: border-box;
  transition:
    background-color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    border-color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
}

@media (prefers-reduced-motion: reduce) {
  .ui-checkbox__box {
    transition: none;
  }
}

.ui-checkbox[data-state="checked"] .ui-checkbox__box {
  background: var(--ui-accent, var(--ui-fallback-accent));
  border-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-checkbox[data-state="unchecked"] .ui-checkbox__box {
  background: var(--ui-bg, var(--ui-fallback-bg));
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: transparent;
}

.ui-checkbox--variant-accent[data-state="unchecked"] .ui-checkbox__box {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
}

.ui-checkbox__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;

  opacity: var(--ui-checkbox-indicator, 0);
  transform: scale(
    calc(0.8 + (var(--ui-checkbox-indicator, 0) * 0.2))
  );
  transform-origin: center;
  will-change: transform, opacity;
}

.ui-checkbox__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-checkbox--size-default .ui-checkbox__box {
  width: var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default));
  height: var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default));
  border-radius: var(--ui-checkbox-radius-default, var(--ui-fallback-checkbox-radius-default));
}

.ui-checkbox--size-default .ui-checkbox__indicator svg {
  width: var(--ui-checkbox-indicator-size-default, var(--ui-fallback-checkbox-indicator-size-default));
  height: var(--ui-checkbox-indicator-size-default, var(--ui-fallback-checkbox-indicator-size-default));
}

.ui-checkbox--size-sm .ui-checkbox__box {
  width: var(--ui-checkbox-size-sm, var(--ui-fallback-checkbox-size-sm));
  height: var(--ui-checkbox-size-sm, var(--ui-fallback-checkbox-size-sm));
  border-radius: var(--ui-checkbox-radius-sm, var(--ui-fallback-checkbox-radius-sm));
}

.ui-checkbox--size-sm .ui-checkbox__indicator svg {
  width: var(--ui-checkbox-indicator-size-sm, var(--ui-fallback-checkbox-indicator-size-sm));
  height: var(--ui-checkbox-indicator-size-sm, var(--ui-fallback-checkbox-indicator-size-sm));
}

.ui-checkbox--size-lg .ui-checkbox__box {
  width: var(--ui-checkbox-size-lg, var(--ui-fallback-checkbox-size-lg));
  height: var(--ui-checkbox-size-lg, var(--ui-fallback-checkbox-size-lg));
  border-radius: var(--ui-checkbox-radius-lg, var(--ui-fallback-checkbox-radius-lg));
}

.ui-checkbox--size-lg .ui-checkbox__indicator svg {
  width: var(--ui-checkbox-indicator-size-lg, var(--ui-fallback-checkbox-indicator-size-lg));
  height: var(--ui-checkbox-indicator-size-lg, var(--ui-fallback-checkbox-indicator-size-lg));
}
"#;
