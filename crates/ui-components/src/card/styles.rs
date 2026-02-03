pub const CSS: &str = r#"
.ui-card {
  display: block;
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  box-shadow: var(--ui-shadow-sm);

  padding: var(--ui-space-lg);
  color: var(--ui-fg);
}

.ui-card--no-padding {
  padding: 0;
}

.ui-card--variant-default {
  background: var(--ui-bg-muted);
}

.ui-card--variant-muted {
  background: var(--ui-bg);
}

.ui-card--variant-outline {
  background: transparent;
}
"#;
