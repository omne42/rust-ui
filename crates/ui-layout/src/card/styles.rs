pub const CSS: &str = r#"
.ui-card {
  display: block;
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  box-shadow: var(--ui-shadow-sm);
  color: var(--ui-fg);
}

.ui-card--padded,
.ui-card[data-padded="true"] {
  padding: var(--ui-space-lg);
}

.ui-card--no-padding,
.ui-card[data-flush="true"] {
  padding: 0;
}

.ui-card--variant-default,
.ui-card[data-variant="default"] {
  background: var(--ui-bg-muted);
}

.ui-card--variant-muted,
.ui-card[data-variant="muted"] {
  background: var(--ui-bg);
}

.ui-card--variant-outline,
.ui-card[data-variant="outline"] {
  background: transparent;
}
"#;
