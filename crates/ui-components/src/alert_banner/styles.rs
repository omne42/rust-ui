pub const CSS: &str = r#"
.ui-alert-banner {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-md);
  width: 100%;
  padding: var(--ui-space-md);

  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);

  opacity: var(--ui-alert-banner-opacity, 1);
  transform: translateY(var(--ui-alert-banner-translate-y, 0px))
    scale(var(--ui-alert-banner-scale, 1));
  will-change: transform, opacity;
}

.ui-alert-banner__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  flex: 0 0 auto;
  color: var(--ui-fg-muted);
}

.ui-alert-banner__icon svg {
  width: 20px;
  height: 20px;
}

.ui-alert-banner__body {
  min-width: 0;
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ui-alert-banner__title {
  font-weight: 650;
  font-size: 14px;
  line-height: 1.2;
  color: var(--ui-fg);
}

.ui-alert-banner__description,
.ui-alert-banner__content {
  font-size: 13px;
  line-height: 1.45;
  color: color-mix(in oklch, var(--ui-fg) 84%, transparent);
}

.ui-alert-banner__start,
.ui-alert-banner__end {
  flex: 0 0 auto;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-alert-banner__sr-only {
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

.ui-alert-banner--tone-info .ui-alert-banner__icon,
.ui-alert-banner--tone-positive .ui-alert-banner__icon,
.ui-alert-banner--tone-notice .ui-alert-banner__icon {
  color: var(--ui-accent);
}

.ui-alert-banner--tone-negative .ui-alert-banner__icon {
  color: var(--ui-danger);
}

.ui-alert-banner--fill-border.ui-alert-banner--tone-info,
.ui-alert-banner--fill-border.ui-alert-banner--tone-positive,
.ui-alert-banner--fill-border.ui-alert-banner--tone-notice {
  background: color-mix(in oklch, var(--ui-accent) 10%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-accent) 26%, var(--ui-border));
}

.ui-alert-banner--fill-border.ui-alert-banner--tone-negative {
  background: color-mix(in oklch, var(--ui-danger) 10%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 30%, var(--ui-border));
}

.ui-alert-banner--fill-subtle.ui-alert-banner--tone-info,
.ui-alert-banner--fill-subtle.ui-alert-banner--tone-positive,
.ui-alert-banner--fill-subtle.ui-alert-banner--tone-notice {
  background: color-mix(in oklch, var(--ui-accent) 7%, var(--ui-bg-muted));
  border-color: transparent;
}

.ui-alert-banner--fill-subtle.ui-alert-banner--tone-negative {
  background: color-mix(in oklch, var(--ui-danger) 7%, var(--ui-bg-muted));
  border-color: transparent;
}

.ui-alert-banner--fill-bold.ui-alert-banner--tone-info,
.ui-alert-banner--fill-bold.ui-alert-banner--tone-positive,
.ui-alert-banner--fill-bold.ui-alert-banner--tone-notice {
  background: var(--ui-accent);
  color: var(--ui-accent-fg);
  border-color: transparent;
}

.ui-alert-banner--fill-bold.ui-alert-banner--tone-negative {
  background: var(--ui-danger);
  color: var(--ui-danger-fg);
  border-color: transparent;
}

.ui-alert-banner--fill-bold .ui-alert-banner__title,
.ui-alert-banner--fill-bold .ui-alert-banner__description,
.ui-alert-banner--fill-bold .ui-alert-banner__content,
.ui-alert-banner--fill-bold .ui-alert-banner__icon {
  color: inherit;
}
"#;
