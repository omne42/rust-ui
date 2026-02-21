pub const CSS: &str = r#"
.ui-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  padding: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2)
    var(--ui-space-sm, var(--ui-fallback-space-sm));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;
  box-sizing: border-box;

  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 500;
}

.ui-badge svg {
  flex-shrink: 0;
  pointer-events: none;
}

.ui-badge--variant-default,
.ui-badge[data-variant="default"] {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-badge--variant-accent,
.ui-badge[data-variant="accent"] {
  background: var(--ui-accent, var(--ui-fallback-accent));
  border-color: var(--ui-accent, var(--ui-fallback-accent));
  color: var(--ui-accent-fg, var(--ui-fallback-accent-fg));
}

.ui-badge--variant-danger,
.ui-badge[data-variant="danger"] {
  background: var(--ui-danger, var(--ui-fallback-danger));
  border-color: var(--ui-danger, var(--ui-fallback-danger));
  color: var(--ui-danger-fg, var(--ui-fallback-danger-fg));
}

.ui-badge--variant-outline,
.ui-badge[data-variant="outline"] {
  background: var(--ui-bg, var(--ui-fallback-bg-muted));
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));
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
.ui-badge[data-custom-class="true"],
.ui-badge[data-class-source="custom"] {
  --ui-badge-custom-class: 1;
}
"#;
