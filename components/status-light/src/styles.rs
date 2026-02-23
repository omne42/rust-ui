pub const CSS: &str = r#"
.ui-status-light {
  --ui-status-light-dot: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-status-light-label: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  font-weight: 500;
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-status-light__dot {
  width: var(--ui-space-sm, var(--ui-fallback-space-sm));
  height: var(--ui-space-sm, var(--ui-fallback-space-sm));
  flex-shrink: 0;
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: var(--ui-status-light-dot);
  box-shadow: 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)) 12%, transparent);
}

.ui-status-light__label {
  color: var(--ui-status-light-label);
}

.ui-status-light--variant-default,
.ui-status-light[data-variant="default"] {
  --ui-status-light-dot: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-status-light-label: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-status-light--variant-accent,
.ui-status-light[data-variant="accent"] {
  --ui-status-light-dot: var(--ui-accent, var(--ui-fallback-accent));
  --ui-status-light-label: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-status-light--variant-danger,
.ui-status-light[data-variant="danger"] {
  --ui-status-light-dot: var(--ui-danger, var(--ui-fallback-danger));
  --ui-status-light-label: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-status-light--live,
.ui-status-light[data-state="live"],
.ui-status-light[data-live="true"] {
  font-weight: 600;
}

.ui-status-light--static,
.ui-status-light[data-state="static"],
.ui-status-light[data-static="true"] {
  --ui-status-light-static-state: 1;
}

.ui-status-light--static .ui-status-light__dot,
.ui-status-light[data-state="static"] .ui-status-light__dot,
.ui-status-light[data-static="true"] .ui-status-light__dot {
  opacity: 0.9;
}

.ui-status-light--role-none,
.ui-status-light[data-role-source="none"] {
  --ui-status-light-role-source: 0;
}

.ui-status-light--role-custom,
.ui-status-light[data-role-source="custom"] {
  --ui-status-light-role-source: 1;
}

.ui-status-light--custom-class,
.ui-status-light[data-custom-class="true"],
.ui-status-light[data-class-source="custom"] {
  --ui-status-light-custom-class: 1;
}
"#;
