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
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 500;
}

.ui-kbd--size-sm,
.ui-kbd[data-size="sm"] {
  padding: 0 var(--ui-space-xs);
  min-height: 20px;
  font-size: var(--ui-font-size-100, 12px);
}

.ui-kbd--size-md,
.ui-kbd[data-size="md"] {
  padding: calc(var(--ui-space-xs) / 2) var(--ui-space-sm);
  min-height: 24px;
  font-size: var(--ui-font-size-100, 12px);
}

.ui-kbd--state-with-keys,
.ui-kbd[data-state="with-keys"],
.ui-kbd[data-keys="true"] {
  gap: var(--ui-space-xs);
}

.ui-kbd--state-label-only,
.ui-kbd[data-state="label-only"] {
  gap: 0;
}

.ui-kbd--custom-class,
.ui-kbd[data-custom-class="true"] {
  --ui-kbd-custom-class: 1;
}

.ui-kbd__keys {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
}

.ui-kbd__label {
  display: inline-flex;
  align-items: center;
}
"#;
