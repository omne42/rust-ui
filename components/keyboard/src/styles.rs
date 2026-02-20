pub const CSS: &str = r#"
.ui-keyboard {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 1.25rem;
  padding: 0.125rem 0.375rem;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: color-mix(in oklab, var(--ui-bg-muted) 92%, var(--ui-bg) 8%);
  color: var(--ui-fg);
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
}

.ui-keyboard--tone-default,
.ui-keyboard[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-keyboard--tone-muted,
.ui-keyboard[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-keyboard--compact,
.ui-keyboard[data-compact="true"] {
  min-height: 1.125rem;
  padding: 0.0625rem 0.25rem;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-keyboard--custom-class,
.ui-keyboard[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
