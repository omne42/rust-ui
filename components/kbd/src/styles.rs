pub const CSS: &str = r#"
.ui-kbd {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  box-sizing: border-box;

  font-family: inherit;
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 500;
}

.ui-kbd--size-sm,
.ui-kbd[data-size="sm"] {
  padding: 0 var(--ui-space-xs, var(--ui-fallback-space-xs));
  min-height: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.625
  );
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
}

.ui-kbd--size-md,
.ui-kbd[data-size="md"] {
  padding: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2)
    var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-height: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.75
  );
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
}

.ui-kbd--state-with-keys,
.ui-kbd[data-state="with-keys"],
.ui-kbd[data-keys="true"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
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
