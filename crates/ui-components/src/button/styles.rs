pub const CSS: &str = r#"
.ui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  white-space: nowrap;
  border-radius: var(--ui-radius-md);
  border: 1px solid transparent;
  box-sizing: border-box;
  line-height: 1;
  font-weight: 500;
  font-size: 14px;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  text-decoration: none;

  transform: scale(var(--ui-button-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-button:not(:disabled) {
  cursor: pointer;
}

.ui-button:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-button svg {
  pointer-events: none;
  flex-shrink: 0;
}

.ui-button--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-button--size-default {
  height: 36px;
  padding: 8px 16px;
}

.ui-button--size-sm {
  height: 32px;
  padding: 0 12px;
  gap: 6px;
}

.ui-button--size-lg {
  height: 40px;
  padding: 0 24px;
}

.ui-button--size-icon {
  width: 36px;
  height: 36px;
  padding: 0;
}

.ui-button--size-icon-sm {
  width: 32px;
  height: 32px;
  padding: 0;
}

.ui-button--size-icon-lg {
  width: 40px;
  height: 40px;
  padding: 0;
}

.ui-button--variant-default {
  background: var(--ui-accent);
  color: var(--ui-accent-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-accent {
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-destructive {
  background: var(--ui-danger);
  color: var(--ui-danger-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-outline {
  background: var(--ui-bg);
  border-color: var(--ui-border);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-secondary {
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-ghost {
  background: transparent;
  color: var(--ui-fg);
  box-shadow: none;
}

.ui-button--variant-link {
  background: transparent;
  color: var(--ui-accent);
  box-shadow: none;
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-default {
  filter: brightness(0.95);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-accent {
  filter: brightness(0.97);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-secondary {
  filter: brightness(0.97);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-destructive {
  filter: brightness(0.95);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-outline {
  background: var(--ui-bg-muted);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-ghost {
  background: var(--ui-bg-muted);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-link {
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 4px;
}
"#;
