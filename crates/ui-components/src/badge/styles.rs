pub const CSS: &str = r#"
.ui-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-xs);
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  padding: calc(var(--ui-space-xs) / 2) var(--ui-space-sm);
  border-radius: var(--ui-radius-lg);
  border: 1px solid transparent;
  box-sizing: border-box;

  font-size: 12px;
  line-height: 1;
  font-weight: 500;
}

.ui-badge svg {
  flex-shrink: 0;
  pointer-events: none;
}

.ui-badge--variant-default {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-badge--variant-accent {
  background: var(--ui-accent);
  border-color: var(--ui-accent);
  color: var(--ui-accent-fg);
}

.ui-badge--variant-danger {
  background: var(--ui-danger);
  border-color: var(--ui-danger);
  color: var(--ui-danger-fg);
}

.ui-badge--variant-outline {
  background: var(--ui-bg);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}
"#;
