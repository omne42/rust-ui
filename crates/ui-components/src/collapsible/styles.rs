pub const CSS: &str = r#"
.ui-collapsible {
  --ui-collapsible-open: 0;
  display: flex;
  flex-direction: column;
}

.ui-collapsible[data-open="true"] {
  --ui-collapsible-open: 1;
}

.ui-collapsible[data-closed="true"] {
  --ui-collapsible-open: 0;
}

.ui-collapsible .ui-disclosure__trigger {
  transition:
    border-color 200ms ease,
    background-color 200ms ease,
    box-shadow 200ms ease;
}

.ui-collapsible[data-open="true"] .ui-disclosure__trigger {
  border-color: color-mix(in oklch, var(--ui-accent) 35%, var(--ui-border));
  background: color-mix(in oklch, var(--ui-bg) 85%, var(--ui-accent-soft));
}

.ui-collapsible[data-disabled="true"] .ui-disclosure__trigger {
  box-shadow: none;
}

.ui-collapsible .ui-disclosure__panel {
  border-color: color-mix(in oklch, var(--ui-border) 85%, var(--ui-accent-soft));
}
"#;
