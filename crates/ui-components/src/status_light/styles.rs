pub const CSS: &str = r#"
.ui-status-light {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  font-size: 14px;
  font-weight: 500;
  line-height: 1.2;
}

.ui-status-light__dot {
  width: 10px;
  height: 10px;
  flex-shrink: 0;
  border-radius: 9999px;
  background: var(--ui-status-light-dot);
  box-shadow: 0 0 0 1px color-mix(in oklch, var(--ui-fg) 12%, transparent);
}

.ui-status-light__label {
  color: var(--ui-status-light-label);
}

.ui-status-light--variant-default {
  --ui-status-light-dot: var(--ui-fg-muted);
  --ui-status-light-label: var(--ui-fg-muted);
}

.ui-status-light--variant-accent {
  --ui-status-light-dot: var(--ui-accent);
  --ui-status-light-label: var(--ui-accent);
}

.ui-status-light--variant-danger {
  --ui-status-light-dot: var(--ui-danger);
  --ui-status-light-label: var(--ui-danger);
}
"#;
