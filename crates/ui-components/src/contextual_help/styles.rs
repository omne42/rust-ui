pub const CSS: &str = r#"
.ui-contextual-help {
  display: inline-flex;
  align-items: center;
}

.ui-contextual-help__trigger {
  --ui-contextual-help-icon-size: 18px;
}

.ui-contextual-help__trigger svg {
  width: var(--ui-contextual-help-icon-size);
  height: var(--ui-contextual-help-icon-size);
}

.ui-contextual-help__panel {
  display: grid;
  gap: var(--ui-space-sm);
  min-width: 240px;
  max-width: 360px;
}

.ui-contextual-help__heading {
  font-size: 13px;
  font-weight: 650;
  color: var(--ui-fg);
}

.ui-contextual-help__content {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
}

.ui-contextual-help__footer {
  font-size: 12px;
  color: var(--ui-fg-muted);
}

.ui-contextual-help--variant-info .ui-contextual-help__trigger {
  color: var(--ui-accent);
}
"#;
