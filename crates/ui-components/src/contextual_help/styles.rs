pub const CSS: &str = r#"
.ui-contextual-help {
  display: inline-flex;
  align-items: center;
}

.ui-contextual-help[data-motion-source="custom"],
.ui-contextual-help--custom-motion,
.ui-contextual-help[data-custom-motion="true"] {
  --ui-contextual-help-custom-motion: 1;
}

.ui-contextual-help--enabled,
.ui-contextual-help[data-state="enabled"] {
  opacity: 1;
}

.ui-contextual-help--disabled,
.ui-contextual-help[data-state="disabled"],
.ui-contextual-help[data-disabled="true"] {
  opacity: 0.72;
}

.ui-contextual-help--placement-bottom-start,
.ui-contextual-help[data-placement="bottom-start"],
.ui-contextual-help--placement-bottom-end,
.ui-contextual-help[data-placement="bottom-end"],
.ui-contextual-help--placement-top-start,
.ui-contextual-help[data-placement="top-start"],
.ui-contextual-help--placement-top-end,
.ui-contextual-help[data-placement="top-end"] {
  justify-content: flex-start;
}

.ui-contextual-help--variant-help .ui-contextual-help__trigger,
.ui-contextual-help[data-variant="help"] .ui-contextual-help__trigger {
  color: var(--ui-fg-muted);
}

.ui-contextual-help--variant-info .ui-contextual-help__trigger,
.ui-contextual-help[data-variant="info"] .ui-contextual-help__trigger {
  color: var(--ui-accent);
}

.ui-contextual-help[data-class-source="custom"],
.ui-contextual-help--custom-class,
.ui-contextual-help[data-custom-class="true"] {
  border-radius: inherit;
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

.ui-contextual-help--no-heading .ui-contextual-help__panel,
.ui-contextual-help[data-heading="absent"] .ui-contextual-help__panel,
.ui-contextual-help__panel[data-heading="absent"] {
  gap: var(--ui-space-xs);
}

.ui-contextual-help__panel[data-footer="present"] .ui-contextual-help__footer,
.ui-contextual-help--with-footer .ui-contextual-help__footer,
.ui-contextual-help[data-footer="present"] .ui-contextual-help__footer {
  padding-top: var(--ui-space-xs);
  border-top: 1px solid var(--ui-border);
}

.ui-contextual-help__heading {
  font-size: 13px;
  font-weight: 650;
  color: var(--ui-fg);
  margin: 0;
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
"#;
