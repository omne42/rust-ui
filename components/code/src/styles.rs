pub const CSS: &str = r#"
.ui-code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
  color: var(--ui-fg, var(--ui-fallback-fg));
  background: var(--ui-bg, var(--ui-fallback-bg));
  border: 1px solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  box-sizing: border-box;
  user-select: text;
}

.ui-code--variant-inline,
.ui-code[data-variant="inline"] {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-code--variant-block,
.ui-code[data-variant="block"] {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-code--state-inline,
.ui-code[data-state="inline"],
.ui-code[data-inline="true"] {
  display: inline-block;
  padding: var(
      --ui-space-3xs,
      var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs))
    )
    var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-code--state-block,
.ui-code[data-state="block"],
.ui-code[data-block="true"] {
  display: block;
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm));
  overflow: auto;
  white-space: pre;
}

.ui-code--custom-class,
.ui-code[data-custom-class="true"] {
  --ui-code-custom-class: 1;
}
"#;
