pub const CSS: &str = r#"
.ui-disclosure {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-disclosure__trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  width: 100%;

  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  padding: var(--ui-space-sm) var(--ui-space-md);

  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);

  font-size: 14px;
  font-weight: 600;
  line-height: 1.2;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-disclosure__trigger:not(:disabled) {
  cursor: pointer;
}

.ui-disclosure__trigger:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-disclosure__trigger[data-hovered="true"]:not(:disabled) {
  background: var(--ui-bg-muted);
}

.ui-disclosure__trigger--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-disclosure__indicator {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;

  transform: rotate(var(--ui-disclosure-indicator-rotation, 0deg));
  transform-origin: center;
}

.ui-disclosure__panel {
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);

  padding: var(--ui-space-md);
}
"#;
