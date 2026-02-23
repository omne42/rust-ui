pub const CSS: &str = r#"
.ui-switch {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-switch-gap, var(--ui-fallback-switch-gap));
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  border: none;
  padding: 0;
  background: transparent;
  color: inherit;
  font: inherit;

  --ui-switch-thumb-x: 0px;
  --ui-switch-thumb-width: var(--ui-switch-thumb-size, var(--ui-fallback-switch-thumb-size));
  --ui-switch-track-bg: var(--ui-bg-muted);
}

.ui-switch[data-motion-source="custom"],
.ui-switch[data-custom-motion="true"] {
  --ui-switch-custom-motion: 1;
}

.ui-switch[data-state="checked"] {
  --ui-switch-thumb-x: var(--ui-switch-thumb-checked-x, var(--ui-fallback-switch-thumb-checked-x));
  --ui-switch-track-bg: var(--ui-accent);
}

.ui-switch:not(:disabled) {
  cursor: pointer;
}

.ui-switch:disabled {
  opacity: var(--ui-switch-disabled-opacity, var(--ui-fallback-switch-disabled-opacity));
  cursor: not-allowed;
}

.ui-switch--focus-visible {
  outline: var(--ui-switch-focus-outline-width, var(--ui-fallback-switch-focus-outline-width)) solid var(--ui-focus-ring);
  outline-offset: var(--ui-switch-focus-outline-offset, var(--ui-fallback-switch-focus-outline-offset));
  border-radius: var(--ui-radius-md);
}

.ui-switch__track {
  position: relative;
  width: var(--ui-switch-track-width, var(--ui-fallback-switch-track-width));
  height: var(--ui-switch-track-height, var(--ui-fallback-switch-track-height));
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
  background: var(--ui-switch-track-bg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;
  flex-shrink: 0;
}

.ui-switch__thumb {
  position: absolute;
  top: var(--ui-switch-track-padding, var(--ui-fallback-switch-track-padding));
  left: var(--ui-switch-track-padding, var(--ui-fallback-switch-track-padding));

  width: var(--ui-switch-thumb-width);
  height: var(--ui-switch-thumb-size, var(--ui-fallback-switch-thumb-size));
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));

  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));

  transform: translateX(var(--ui-switch-thumb-x));
  will-change: transform, width;
  pointer-events: none;
}

.ui-switch__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-switch[data-hovered="true"]:not(:disabled) .ui-switch__track {
  filter: brightness(var(--ui-switch-hover-brightness, var(--ui-fallback-switch-hover-brightness)));
}
"#;
