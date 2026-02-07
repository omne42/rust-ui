pub const CSS: &str = r#"
.ui-alert {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  gap: var(--ui-space-xs);

  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  border-left: 3px solid transparent;
  background: var(--ui-bg-muted);
  box-shadow: var(--ui-shadow-sm);
  padding: var(--ui-space-lg);
  color: var(--ui-fg);
}

.ui-alert--detailed,
.ui-alert[data-state="detailed"] {
  gap: var(--ui-space-xs);
}

.ui-alert--compact,
.ui-alert[data-state="compact"] {
  gap: var(--ui-space-2xs, 4px);
}

.ui-alert--custom-class,
.ui-alert[data-custom-class="true"] {
  border-radius: inherit;
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

.ui-alert--with-actions .ui-alert__actions,
.ui-alert[data-actions="present"] .ui-alert__actions {
  margin-top: var(--ui-space-xs);
}

.ui-alert--no-actions .ui-alert__actions,
.ui-alert[data-actions="absent"] .ui-alert__actions {
  display: none;
}

.ui-alert--no-description .ui-alert__title,
.ui-alert[data-description="absent"] .ui-alert__title {
  color: var(--ui-fg);
}

.ui-alert--variant-default,
.ui-alert[data-variant="default"] {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  border-left-color: color-mix(in oklch, var(--ui-border) 85%, transparent);
}

.ui-alert--variant-accent,
.ui-alert[data-variant="accent"] {
  background: var(--ui-accent-soft);
  border-color: color-mix(in oklch, var(--ui-accent) 24%, var(--ui-border));
  border-left-color: color-mix(in oklch, var(--ui-accent) 62%, var(--ui-border));
}

.ui-alert--variant-danger,
.ui-alert[data-variant="danger"] {
  background: color-mix(in oklch, var(--ui-danger) 12%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 35%, var(--ui-border));
  border-left-color: var(--ui-danger);
}
"#;
