pub const CSS: &str = r#"
.ui-snippet {
  --ui-snippet-scale: 1;
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
  transform: scale(var(--ui-snippet-scale));
  transform-origin: center;

  font-family: var(--ui-font-family-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace);
}

.ui-snippet--state-multiline,
.ui-snippet[data-state="multiline"],
.ui-snippet[data-multiline="true"] {
  align-items: flex-start;
}

.ui-snippet--state-multiline .ui-snippet__copy-button,
.ui-snippet[data-state="multiline"] .ui-snippet__copy-button,
.ui-snippet[data-multiline="true"] .ui-snippet__copy-button {
  align-self: flex-start;
}

.ui-snippet--state-single-line,
.ui-snippet[data-state="single-line"] {
  align-items: center;
}

.ui-snippet--copyable,
.ui-snippet[data-copy="copyable"],
.ui-snippet[data-copy-actionable="true"] {
  --ui-snippet-copy-visible: 1;
}

.ui-snippet--copy-disabled,
.ui-snippet[data-copy="disabled"] {
  --ui-snippet-copy-visible: 1;
}

.ui-snippet--copy-static,
.ui-snippet[data-copy="static"] {
  --ui-snippet-copy-visible: 0;
}

.ui-snippet--with-label,
.ui-snippet[data-label="true"] {
  --ui-snippet-has-label: 1;
}

.ui-snippet--empty,
.ui-snippet[data-empty="true"] {
  --ui-snippet-empty: 1;
}

.ui-snippet--custom-copied-label,
.ui-snippet[data-copied-label="custom"] {
  --ui-snippet-custom-copied-label: 1;
}

.ui-snippet--custom-class,
.ui-snippet[data-custom-class="true"] {
  --ui-snippet-custom-class: 1;
}

.ui-snippet__label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  padding: 0 var(--ui-space-xs);
  min-height: var(--ui-component-height-100);

  border-radius: var(--ui-radius-sm);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg-muted);
  box-sizing: border-box;

  font-family: inherit;
  font-size: var(--ui-font-size-100);
  line-height: var(--ui-line-height-100, 16px);
  user-select: none;
}

.ui-snippet__pre {
  margin: 0;
  min-width: 0;
  flex: 1;

  white-space: pre-wrap;
  overflow-wrap: anywhere;

  font-size: var(--ui-font-size-100);
  line-height: var(--ui-line-height-100, 16px);
  user-select: text;
}

.ui-snippet__copy-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  padding: calc(var(--ui-space-xs) / 2) var(--ui-space-sm);
  min-height: var(--ui-component-height-100);

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

.ui-snippet[data-copy-status="idle"] .ui-snippet__copy-button {
  color: var(--ui-fg-muted);
}

.ui-snippet[data-copy-status="loading"] .ui-snippet__copy-button,
.ui-snippet__copy-button[data-copying="true"] {
  opacity: 0.72;
}

.ui-snippet[data-copy-status="error"] .ui-snippet__copy-button,
.ui-snippet__copy-button[data-copy-error="true"] {
  color: var(--ui-fg);
}

.ui-snippet[data-copy-status="copied"] .ui-snippet__copy-button,
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
