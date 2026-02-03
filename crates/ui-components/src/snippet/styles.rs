pub const CSS: &str = r#"
.ui-snippet {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  min-width: 0;

  position: relative;
  padding: var(--ui-space-sm) var(--ui-space-md);

  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;

  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
}

.ui-snippet[data-multiline="true"] {
  align-items: flex-start;
}

.ui-snippet[data-multiline="true"] .ui-snippet__copy-button {
  align-self: flex-start;
}

.ui-snippet__label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  padding: 0 var(--ui-space-xs);
  min-height: 20px;

  border-radius: var(--ui-radius-sm);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg-muted);
  box-sizing: border-box;

  font-family: inherit;
  font-size: 12px;
  line-height: 1;
  user-select: none;
}

.ui-snippet__pre {
  margin: 0;
  min-width: 0;
  flex: 1;

  white-space: pre-wrap;
  overflow-wrap: anywhere;

  font-size: 13px;
  line-height: 1.4;
  user-select: text;
}

.ui-snippet__copy-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  padding: calc(var(--ui-space-xs) / 2) var(--ui-space-sm);
  min-height: 28px;

  border-radius: var(--ui-radius-sm);
  border: 1px solid transparent;
  background: transparent;
  color: var(--ui-fg-muted);
  box-sizing: border-box;

  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-snippet__copy-button:not(:disabled) {
  cursor: pointer;
}

.ui-snippet__copy-button:hover:not(:disabled) {
  background: var(--ui-bg);
  color: var(--ui-fg);
}

.ui-snippet__copy-button:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-snippet__copy-button[data-copied="true"] {
  color: var(--ui-accent);
}

.ui-snippet__copy-button:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-snippet__a11y-status {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
"#;
