pub const CSS: &str = r#"
.ui-legend {
  --ui-legend-motion-duration: 140ms;

  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  margin: 0;
  padding: 0;
  color: var(--ui-fg);
  font-size: var(--ui-button-size-l-font-size, 15px);
  line-height: var(--ui-button-size-l-line-height, 22px);
  font-weight: 600;
  transition:
    color var(--ui-legend-motion-duration) ease,
    opacity var(--ui-legend-motion-duration) ease;
}

.ui-legend--tone-default,
.ui-legend[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-legend--tone-muted,
.ui-legend[data-tone="muted"] {
  color: var(--ui-fg-muted);
  font-weight: 500;
}

.ui-legend--tone-strong,
.ui-legend[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 92%, black 8%);
  letter-spacing: 0.01em;
}

.ui-legend--required,
.ui-legend[data-required="true"] {
  letter-spacing: 0.01em;
}

.ui-legend--disabled,
.ui-legend[data-disabled="true"] {
  color: color-mix(in oklab, var(--ui-fg-muted) 76%, var(--ui-bg) 24%);
}

.ui-legend--text-custom,
.ui-legend[data-text-source="custom"] {
  text-decoration: underline;
  text-underline-offset: 0.12em;
}

.ui-legend--indicator-custom,
.ui-legend[data-indicator-source="custom"] {
  gap: var(--ui-space-xs);
}

.ui-legend--custom-class,
.ui-legend[data-custom-class="true"] {
  outline: 1px dashed color-mix(in oklab, var(--ui-accent) 30%, transparent);
  outline-offset: 2px;
}

.ui-legend__text {
  display: inline-flex;
  align-items: center;
}

.ui-legend__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger);
  font-size: 0.85em;
}

@media (prefers-reduced-motion: reduce) {
  .ui-legend {
    --ui-legend-motion-duration: 1ms;
  }
}
"#;
