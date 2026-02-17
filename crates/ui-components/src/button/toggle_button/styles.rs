pub const CSS: &str = r#"
.ui-toggle-button {
  --ui-toggle-button-scale: var(--ui-button-scale, 1);

  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  white-space: nowrap;
  position: relative;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-toggle-button-border, transparent);
  box-sizing: border-box;
  line-height: 1;
  font-weight: 500;
  font-size: 14px;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  text-decoration: none;

  background: var(--ui-toggle-button-bg, transparent);
  color: var(--ui-toggle-button-fg, var(--ui-fg));
  box-shadow: var(--ui-toggle-button-shadow, none);

  transform: scale(var(--ui-toggle-button-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-toggle-button[data-motion-source="custom"],
.ui-toggle-button[data-custom-motion="true"] {
  --ui-toggle-button-custom-motion: 1;
}

.ui-toggle-button:not(:disabled) {
  cursor: pointer;
}

.ui-toggle-button:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-toggle-button--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-toggle-button--size-xs {
  height: 28px;
  padding: 0 10px;
  gap: 4px;
}

.ui-toggle-button--size-s,
.ui-toggle-button--size-sm {
  height: 32px;
  padding: 0 12px;
  gap: 6px;
}

.ui-toggle-button--size-m,
.ui-toggle-button--size-default {
  height: 36px;
  padding: 8px 16px;
}

.ui-toggle-button--size-l,
.ui-toggle-button--size-lg {
  height: 40px;
  padding: 0 24px;
}

.ui-toggle-button--size-xl {
  height: 44px;
  padding: 0 28px;
}

.ui-toggle-button--size-icon-xs {
  width: 28px;
  height: 28px;
  padding: 0;
}

.ui-toggle-button--size-icon-s,
.ui-toggle-button--size-icon-sm {
  width: 32px;
  height: 32px;
  padding: 0;
}

.ui-toggle-button--size-icon-m,
.ui-toggle-button--size-icon {
  width: 36px;
  height: 36px;
  padding: 0;
}

.ui-toggle-button--size-icon-l,
.ui-toggle-button--size-icon-lg {
  width: 40px;
  height: 40px;
  padding: 0;
}

.ui-toggle-button--size-icon-xl {
  width: 44px;
  height: 44px;
  padding: 0;
}

.ui-toggle-button--variant-default {
  --ui-toggle-button-bg: var(--ui-bg-muted);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: var(--ui-border);
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button--variant-accent {
  --ui-toggle-button-bg: color-mix(in oklch, var(--ui-accent) 15%, transparent);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: color-mix(in oklch, var(--ui-accent) 35%, transparent);
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button--variant-secondary {
  --ui-toggle-button-bg: var(--ui-bg-muted);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: var(--ui-border);
  --ui-toggle-button-shadow: none;
}

.ui-toggle-button--variant-outline {
  --ui-toggle-button-bg: var(--ui-bg);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: var(--ui-border);
  --ui-toggle-button-shadow: none;
}

.ui-toggle-button--variant-ghost {
  --ui-toggle-button-bg: transparent;
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: transparent;
  --ui-toggle-button-shadow: none;
}

.ui-toggle-button--variant-destructive {
  --ui-toggle-button-bg: color-mix(in oklch, var(--ui-danger) 16%, transparent);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: color-mix(in oklch, var(--ui-danger) 35%, transparent);
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-default {
  --ui-toggle-button-bg: var(--ui-accent);
  --ui-toggle-button-fg: var(--ui-accent-fg);
  --ui-toggle-button-border: transparent;
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-accent {
  --ui-toggle-button-bg: var(--ui-accent);
  --ui-toggle-button-fg: var(--ui-accent-fg);
  --ui-toggle-button-border: transparent;
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-secondary {
  --ui-toggle-button-bg: var(--ui-accent);
  --ui-toggle-button-fg: var(--ui-accent-fg);
  --ui-toggle-button-border: transparent;
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-outline {
  --ui-toggle-button-bg: var(--ui-bg-muted);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: var(--ui-border);
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-ghost {
  --ui-toggle-button-bg: var(--ui-bg-muted);
  --ui-toggle-button-fg: var(--ui-fg);
  --ui-toggle-button-border: var(--ui-border);
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-selected="true"].ui-toggle-button--variant-destructive {
  --ui-toggle-button-bg: var(--ui-danger);
  --ui-toggle-button-fg: var(--ui-danger-fg);
  --ui-toggle-button-border: transparent;
  --ui-toggle-button-shadow: var(--ui-shadow-sm);
}

.ui-toggle-button[data-hovered="true"]:not(:disabled).ui-toggle-button--variant-ghost {
  background: var(--ui-bg-muted);
}

.ui-toggle-button[data-hovered="true"]:not(:disabled).ui-toggle-button--variant-outline {
  background: var(--ui-bg-muted);
}
"#;
