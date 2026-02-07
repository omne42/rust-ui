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

.ui-badge--variant-default,
.ui-badge[data-variant="default"] {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-badge--variant-accent,
.ui-badge[data-variant="accent"] {
  background: var(--ui-accent);
  border-color: var(--ui-accent);
  color: var(--ui-accent-fg);
}

.ui-badge--variant-danger,
.ui-badge[data-variant="danger"] {
  background: var(--ui-danger);
  border-color: var(--ui-danger);
  color: var(--ui-danger-fg);
}

.ui-badge--variant-outline,
.ui-badge[data-variant="outline"] {
  background: var(--ui-bg);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-badge--fill-solid,
.ui-badge[data-fill="solid"],
.ui-badge[data-solid="true"],
.ui-badge[data-state="solid"] {
  --ui-badge-fill-outline: 0;
}

.ui-badge--fill-outline,
.ui-badge[data-fill="outline"],
.ui-badge[data-outline="true"],
.ui-badge[data-state="outline"] {
  --ui-badge-fill-outline: 1;
}

.ui-badge--custom-class,
.ui-badge[data-custom-class="true"] {
  --ui-badge-custom-class: 1;
}
"#;
