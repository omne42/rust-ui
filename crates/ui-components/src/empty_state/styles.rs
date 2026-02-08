pub const CSS: &str = r#"
.ui-empty-state {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-xl);
  border-radius: var(--ui-radius-lg);
  background: color-mix(in oklab, var(--ui-bg-muted) 86%, var(--ui-bg) 14%);
  border: 1px solid transparent;
  color: var(--ui-fg);
}

.ui-empty-state--tone-default,
.ui-empty-state[data-tone="default"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 88%, var(--ui-bg) 12%);
}

.ui-empty-state--tone-muted,
.ui-empty-state[data-tone="muted"] {
  background: var(--ui-bg);
  color: var(--ui-fg-muted);
}

.ui-empty-state--tone-accent,
.ui-empty-state[data-tone="accent"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 34%, var(--ui-bg-muted) 66%);
  border-color: color-mix(in oklab, var(--ui-accent) 36%, transparent);
}

.ui-empty-state--align-start,
.ui-empty-state[data-align="start"] {
  align-items: flex-start;
  text-align: left;
}

.ui-empty-state--align-center,
.ui-empty-state[data-align="center"] {
  align-items: center;
  text-align: center;
}

.ui-empty-state--compact,
.ui-empty-state[data-compact="true"] {
  padding: var(--ui-space-md) var(--ui-space-lg);
}

.ui-empty-state--bordered,
.ui-empty-state[data-bordered="true"] {
  border: 1px dashed color-mix(in oklab, var(--ui-border) 78%, var(--ui-accent) 22%);
}

.ui-empty-state__icon {
  width: 44px;
  height: 44px;
  border-radius: 14px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: color-mix(in oklab, var(--ui-accent-soft) 56%, transparent);
  color: var(--ui-accent);
}

.ui-empty-state__title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-empty-state__description {
  margin: 0;
  max-width: 46ch;
  font-size: 0.875rem;
  line-height: 1.45;
  color: var(--ui-fg-muted);
}

.ui-empty-state__actions {
  margin-top: var(--ui-space-xs);
  display: inline-flex;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-empty-state--custom-class,
.ui-empty-state[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 26%, transparent);
  outline-offset: 2px;
}
"#;
