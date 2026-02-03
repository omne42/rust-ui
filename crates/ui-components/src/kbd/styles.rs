pub const CSS: &str = r#"
.ui-kbd {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-xs);
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  border-radius: var(--ui-radius-sm);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  color: var(--ui-fg-muted);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;

  font-family: inherit;
  line-height: 1;
  font-weight: 500;
}

.ui-kbd--size-sm {
  padding: 0 var(--ui-space-xs);
  min-height: 20px;
  font-size: 11px;
}

.ui-kbd--size-md {
  padding: calc(var(--ui-space-xs) / 2) var(--ui-space-sm);
  min-height: 24px;
  font-size: 12px;
}

.ui-kbd__keys {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
}
"#;
