pub const CSS: &str = r#"
.ui-radio-group {
  display: flex;
  gap: var(--ui-space-sm);
}

.ui-radio-group--vertical {
  flex-direction: column;
}

.ui-radio-group--horizontal {
  flex-direction: row;
  flex-wrap: wrap;
}

.ui-radio-group__label {
  font-size: var(--ui-font-size-100);
  line-height: var(--ui-line-height-100);
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-radio {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-xs) var(--ui-space-sm);
  border-radius: var(--ui-radius-md);
  border: 1px solid transparent;
  background: transparent;
  color: var(--ui-fg);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 500;

  transform: scale(var(--ui-radio-scale, 1));
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
  outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-button-focus-outline-offset);
}

.ui-radio__indicator {
  width: var(--ui-icon-size-100);
  height: var(--ui-icon-size-100);
  border-radius: var(--ui-button-radius-full);
  box-sizing: border-box;
  border: calc(var(--ui-space-2xs) / 2) solid var(--ui-border);
  background: var(--ui-bg);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.ui-radio[data-checked=\"true\"] .ui-radio__indicator {
  border-color: var(--ui-accent);
}

.ui-radio__dot {
  width: calc(var(--ui-icon-size-100) / 2);
  height: calc(var(--ui-icon-size-100) / 2);
  border-radius: var(--ui-button-radius-full);
  background: var(--ui-accent);
  opacity: 0;
  transform: scale(0.5);
}

.ui-radio[data-checked=\"true\"] .ui-radio__dot {
  opacity: 1;
  transform: scale(1);
}
"#;
