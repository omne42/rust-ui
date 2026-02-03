pub const CSS: &str = r#"
.ui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: var(--ui-radius-md);
  border: 1px solid;
  box-shadow: var(--ui-shadow-sm);
  font-size: 14px;
  line-height: 1;
  user-select: none;
  transform-origin: center;
  will-change: transform, filter;
}

.ui-button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.ui-button:not(:disabled) {
  cursor: pointer;
}

.ui-button--default {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-button--primary {
  background: var(--ui-accent);
  border-color: var(--ui-accent);
  color: var(--ui-accent-fg);
}

.ui-button--focus-visible {
  outline: 2px solid var(--ui-focus-ring);
  outline-offset: 2px;
}
"#;
