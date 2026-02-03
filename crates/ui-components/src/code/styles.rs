pub const CSS: &str = r#"
.ui-code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
  color: var(--ui-fg);
  background: var(--ui-bg);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  box-sizing: border-box;
  user-select: text;
}

.ui-code--variant-inline {
  display: inline-block;
  padding: 2px 6px;
  font-size: 12px;
  line-height: 1.2;
}

.ui-code--variant-block {
  display: block;
  padding: var(--ui-space-sm);
  font-size: 12px;
  line-height: 1.5;
  overflow: auto;
  white-space: pre;
}
"#;
