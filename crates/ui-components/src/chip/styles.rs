pub const CSS: &str = r#"
.ui-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  border-radius: 9999px;
  border: 1px solid transparent;
  box-sizing: border-box;
  font-size: 13px;
  font-weight: 500;
  line-height: 1;

  color: var(--ui-fg);
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
}

.ui-chip--size-sm {
  height: 24px;
  padding: 0 10px;
  font-size: 12px;
}

.ui-chip--size-md {
  height: 28px;
  padding: 0 12px;
}

.ui-chip--size-lg {
  height: 32px;
  padding: 0 14px;
  font-size: 14px;
}

.ui-chip--variant-default {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-chip--variant-accent {
  background: var(--ui-accent-soft);
  border-color: color-mix(in oklch, var(--ui-accent) 24%, var(--ui-border));
  color: var(--ui-fg);
}

.ui-chip--variant-danger {
  background: color-mix(in oklch, var(--ui-danger) 12%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 35%, var(--ui-border));
  color: var(--ui-fg);
}

.ui-chip--variant-outline {
  background: transparent;
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-chip__dismiss {
  width: 18px;
  height: 18px;
  border-radius: 9999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;

  border: 0;
  padding: 0;
  margin: 0;
  background: transparent;
  color: inherit;
  cursor: pointer;
  opacity: 0.7;
}

.ui-chip__dismiss:hover {
  opacity: 1;
  background: color-mix(in oklch, var(--ui-fg) 8%, transparent);
}

.ui-chip__dismiss:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}
"#;
