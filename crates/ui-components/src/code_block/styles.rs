pub const CSS: &str = r#"
.ui-code-block {
  position: relative;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  overflow: hidden;
  isolation: isolate;

  --ui-code-block-copy-flash: 0;
  --ui-code-block-custom-motion: 0;
}

.ui-code-block--state-multiline,
.ui-code-block[data-state="multiline"],
.ui-code-block[data-multiline="true"] {
  --ui-code-block-is-multiline: 1;
}

.ui-code-block--state-single-line,
.ui-code-block[data-state="single-line"] {
  --ui-code-block-is-multiline: 0;
}

.ui-code-block--header-visible,
.ui-code-block[data-header="visible"] {
  --ui-code-block-header-visible: 1;
}

.ui-code-block--header-hidden,
.ui-code-block[data-header="hidden"] {
  --ui-code-block-header-visible: 0;
}

.ui-code-block--copyable,
.ui-code-block[data-copyable="true"] {
  --ui-code-block-copyable: 1;
}

.ui-code-block--with-label,
.ui-code-block[data-label="true"] {
  --ui-code-block-has-label: 1;
}

.ui-code-block--with-language,
.ui-code-block[data-language="true"] {
  --ui-code-block-has-language: 1;
}

.ui-code-block--empty,
.ui-code-block[data-empty="true"] {
  --ui-code-block-is-empty: 1;
}

.ui-code-block--motion-custom,
.ui-code-block[data-motion-source="custom"] {
  --ui-code-block-custom-motion: 1;
}

.ui-code-block--custom-class,
.ui-code-block[data-custom-class="true"] {
  --ui-code-block-custom-class: 1;
}

.ui-code-block::after {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 0;
  background: color-mix(in oklch, var(--ui-accent-soft) 70%, var(--ui-bg));
  opacity: calc(var(--ui-code-block-copy-flash) * 0.18);
  pointer-events: none;
}

.ui-code-block > * {
  position: relative;
  z-index: 1;
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
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
  font-weight: 600;
}

.ui-code-block__language {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}

.ui-code-block__pre {
  margin: 0;
  padding: var(--ui-space-md);
  overflow: auto;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-code-block__code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono",
    "Courier New", monospace;
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
