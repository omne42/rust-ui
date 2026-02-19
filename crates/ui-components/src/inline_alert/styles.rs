pub const CSS: &str = r#"
.ui-inline-alert {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-md);

  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);

  opacity: var(--ui-inline-alert-opacity, 1);
  transform: translateY(var(--ui-inline-alert-translate-y, 0px))
    scale(var(--ui-inline-alert-scale, 1));
  will-change: transform, opacity;
}

.ui-inline-alert[data-motion-source="custom"],
.ui-inline-alert[data-custom-motion="true"] {
  --ui-inline-alert-custom-motion: 1;
}

.ui-inline-alert__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  flex: 0 0 auto;
  margin-top: 1px;
  color: var(--ui-fg-muted);
}

.ui-inline-alert__icon svg {
  width: 18px;
  height: 18px;
}

.ui-inline-alert__body {
  min-width: 0;
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ui-inline-alert__title {
  font-weight: 650;
  font-size: var(--ui-heading-h6-font-size, 13px);
  line-height: var(--ui-heading-h6-line-height, 1.2);
  color: var(--ui-fg);
}

.ui-inline-alert__description,
.ui-inline-alert__content {
  font-size: 13px;
  line-height: 1.45;
  color: color-mix(in oklch, var(--ui-fg) 84%, transparent);
}

.ui-inline-alert__start,
.ui-inline-alert__end {
  flex: 0 0 auto;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-inline-alert__sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.ui-inline-alert--tone-info {
  --ui-inline-alert-tone: var(--ui-accent);
}

.ui-inline-alert--tone-positive {
  --ui-inline-alert-tone: var(--ui-accent);
}

.ui-inline-alert--tone-notice {
  --ui-inline-alert-tone: var(--ui-accent);
}

.ui-inline-alert--tone-negative {
  --ui-inline-alert-tone: var(--ui-danger);
}

.ui-inline-alert--tone-info .ui-inline-alert__icon,
.ui-inline-alert--tone-positive .ui-inline-alert__icon,
.ui-inline-alert--tone-notice .ui-inline-alert__icon {
  color: var(--ui-accent);
}

.ui-inline-alert--tone-negative .ui-inline-alert__icon {
  color: var(--ui-danger);
}

.ui-inline-alert--fill-border.ui-inline-alert--tone-info,
.ui-inline-alert--fill-border.ui-inline-alert--tone-positive,
.ui-inline-alert--fill-border.ui-inline-alert--tone-notice {
  background: color-mix(in oklch, var(--ui-accent) 10%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-accent) 26%, var(--ui-border));
}

.ui-inline-alert--fill-border.ui-inline-alert--tone-negative {
  background: color-mix(in oklch, var(--ui-danger) 10%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 30%, var(--ui-border));
}

.ui-inline-alert--fill-subtle.ui-inline-alert--tone-info,
.ui-inline-alert--fill-subtle.ui-inline-alert--tone-positive,
.ui-inline-alert--fill-subtle.ui-inline-alert--tone-notice {
  background: color-mix(in oklch, var(--ui-accent) 7%, var(--ui-bg-muted));
  border-color: transparent;
}

.ui-inline-alert--fill-subtle.ui-inline-alert--tone-negative {
  background: color-mix(in oklch, var(--ui-danger) 7%, var(--ui-bg-muted));
  border-color: transparent;
}

.ui-inline-alert--fill-bold.ui-inline-alert--tone-info,
.ui-inline-alert--fill-bold.ui-inline-alert--tone-positive,
.ui-inline-alert--fill-bold.ui-inline-alert--tone-notice {
  background: var(--ui-accent);
  color: var(--ui-accent-fg);
  border-color: transparent;
}

.ui-inline-alert--fill-bold.ui-inline-alert--tone-negative {
  background: var(--ui-danger);
  color: var(--ui-danger-fg);
  border-color: transparent;
}

.ui-inline-alert--fill-bold .ui-inline-alert__title,
.ui-inline-alert--fill-bold .ui-inline-alert__description,
.ui-inline-alert--fill-bold .ui-inline-alert__content {
  color: inherit;
}

.ui-inline-alert--fill-bold .ui-inline-alert__icon {
  color: inherit;
}
"#;
