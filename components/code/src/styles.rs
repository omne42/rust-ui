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

.ui-code--variant-inline,
.ui-code[data-variant="inline"] {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-code--variant-block,
.ui-code[data-variant="block"] {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-code--state-inline,
.ui-code[data-state="inline"],
.ui-code[data-inline="true"] {
  display: inline-block;
  padding: 2px 6px;
}

.ui-code--state-block,
.ui-code[data-state="block"],
.ui-code[data-block="true"] {
  display: block;
  padding: var(--ui-space-sm);
  overflow: auto;
  white-space: pre;
}

.ui-code--custom-class,
.ui-code[data-custom-class="true"] {
  --ui-code-custom-class: 1;
}
"#;
