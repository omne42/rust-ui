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
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-contextual-help--variant-info .ui-contextual-help__trigger,
.ui-contextual-help[data-variant="info"] .ui-contextual-help__trigger {
  color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-contextual-help[data-class-source="custom"],
.ui-contextual-help--custom-class,
.ui-contextual-help[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-contextual-help__trigger svg {
  width: var(--ui-icon-size-200, var(--ui-fallback-icon-size-200));
  height: var(--ui-icon-size-200, var(--ui-fallback-icon-size-200));
}

.ui-contextual-help__panel {
  display: grid;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-width: var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width));
  max-width: calc(var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 1.5);
}

.ui-contextual-help--no-heading .ui-contextual-help__panel,
.ui-contextual-help[data-heading="absent"] .ui-contextual-help__panel,
.ui-contextual-help__panel[data-heading="absent"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-contextual-help__panel[data-footer="present"] .ui-contextual-help__footer,
.ui-contextual-help--with-footer .ui-contextual-help__footer,
.ui-contextual-help[data-footer="present"] .ui-contextual-help__footer {
  padding-top: var(--ui-space-xs, var(--ui-fallback-space-xs));
  border-top: 1px solid var(--ui-border, var(--ui-fallback-border));
}

.ui-contextual-help__heading {
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));
  line-height: var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height));
  font-weight: 650;
  color: var(--ui-fg, var(--ui-fallback-fg));
  margin: 0;
}

.ui-contextual-help__content {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-contextual-help__footer {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}
"#;
