pub const CSS: &str = r#"
.ui-code-block {
  position: relative;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  overflow: hidden;
}

.ui-code-block__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-sm) var(--ui-space-md);
  border-bottom: 1px solid var(--ui-border);
  background: color-mix(in oklch, var(--ui-bg-muted) 70%, var(--ui-bg));
}

.ui-code-block__meta {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  min-width: 0;
}

.ui-code-block__label {
  font-size: 13px;
  font-weight: 600;
  line-height: 1;
}

.ui-code-block__language {
  font-size: 12px;
  line-height: 1;
  color: var(--ui-fg-muted);
}

.ui-code-block__pre {
  margin: 0;
  padding: var(--ui-space-md);
  overflow: auto;
  font-size: 12px;
  line-height: 1.55;
}

.ui-code-block__code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\",
    \"Courier New\", monospace;
}

.ui-code-block__a11y-status {
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

.ui-code-block__copy-button svg {
  width: 16px;
  height: 16px;
}
"#;
