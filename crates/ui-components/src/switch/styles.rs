pub const CSS: &str = r#"
.ui-switch {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  border: none;
  padding: 0;
  background: transparent;
  color: inherit;
  font: inherit;

  --ui-switch-thumb-x: 0px;
  --ui-switch-thumb-width: 16px;
  --ui-switch-track-bg: var(--ui-bg-muted);
}

.ui-switch[data-state="checked"] {
  --ui-switch-thumb-x: 12px;
  --ui-switch-track-bg: var(--ui-accent);
}

.ui-switch:not(:disabled) {
  cursor: pointer;
}

.ui-switch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.ui-switch--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-md);
}

.ui-switch__track {
  position: relative;
  width: 32px;
  height: 20px;
  border-radius: 999px;
  background: var(--ui-switch-track-bg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;
  flex-shrink: 0;
}

.ui-switch__thumb {
  position: absolute;
  top: 2px;
  left: 2px;

  width: var(--ui-switch-thumb-width);
  height: 16px;
  border-radius: 999px;

  background: var(--ui-bg);
  box-shadow: 0 1px 2px rgba(0,0,0,0.25);

  transform: translateX(var(--ui-switch-thumb-x));
  will-change: transform, width;
  pointer-events: none;
}

.ui-switch__label {
  font-size: 14px;
  line-height: 1.2;
}

.ui-switch[data-hovered="true"]:not(:disabled) .ui-switch__track {
  filter: brightness(0.98);
}
"#;
