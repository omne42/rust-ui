pub const CSS: &str = r#"
.ui-alert {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--ui-space-xs);

  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  box-shadow: var(--ui-shadow-sm);
  padding: var(--ui-space-lg);
  color: var(--ui-fg);
}

.ui-alert__title {
  font-weight: 650;
  font-size: 14px;
  line-height: 1.2;
}

.ui-alert__description {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
}

.ui-alert__actions {
  display: flex;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
  margin-top: var(--ui-space-xs);
}

.ui-alert--variant-default {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
}

.ui-alert--variant-accent {
  background: var(--ui-accent-soft);
  border-color: var(--ui-border);
}

.ui-alert--variant-danger {
  background: color-mix(in oklch, var(--ui-danger) 12%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 35%, var(--ui-border));
}
"#;
