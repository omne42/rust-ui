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
  font-size: 14px;
  line-height: 1.2;
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
  font: inherit;
  line-height: 1.2;

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
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-radio__indicator {
  width: 16px;
  height: 16px;
  border-radius: 9999px;
  box-sizing: border-box;
  border: 2px solid var(--ui-border);
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
  width: 8px;
  height: 8px;
  border-radius: 9999px;
  background: var(--ui-accent);
  opacity: 0;
  transform: scale(0.5);
}

.ui-radio[data-checked=\"true\"] .ui-radio__dot {
  opacity: 1;
  transform: scale(1);
}
"#;
