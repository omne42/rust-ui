pub const CSS: &str = r#"
.ui-alert {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
  width: 100%;
  padding: var(--ui-space-md, var(--ui-fallback-space-md));

  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  border: 1px solid var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));

  opacity: var(--ui-alert-opacity, 1);
  transform: translateY(
      var(--ui-alert-translate-y, var(--ui-fallback-alert-translate-y))
    )
    scale(var(--ui-alert-scale, var(--ui-fallback-alert-scale)));
  will-change: transform, opacity;
}

.ui-alert[data-motion-source="custom"],
.ui-alert[data-custom-motion="true"] {
  --ui-alert-custom-motion: 1;
}

.ui-alert--layout-banner,
.ui-alert[data-layout="banner"] {
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}

.ui-alert--layout-inline,
.ui-alert[data-layout="inline"] {
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
}

.ui-alert--custom-class,
.ui-alert[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-alert__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-alert-icon-size, var(--ui-fallback-alert-icon-size));
  height: var(--ui-alert-icon-size, var(--ui-fallback-alert-icon-size));
  flex: 0 0 auto;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-alert--layout-inline .ui-alert__icon,
.ui-alert[data-layout="inline"] .ui-alert__icon {
  width: var(
    --ui-alert-icon-size-inline,
    var(--ui-fallback-alert-icon-size-inline)
  );
  height: var(
    --ui-alert-icon-size-inline,
    var(--ui-fallback-alert-icon-size-inline)
  );
  margin-top: var(
    --ui-alert-icon-margin-top-inline,
    var(--ui-fallback-alert-icon-margin-top-inline)
  );
}

.ui-alert__icon svg {
  width: var(--ui-alert-icon-size, var(--ui-fallback-alert-icon-size));
  height: var(--ui-alert-icon-size, var(--ui-fallback-alert-icon-size));
}

.ui-alert--layout-inline .ui-alert__icon svg,
.ui-alert[data-layout="inline"] .ui-alert__icon svg {
  width: var(
    --ui-alert-icon-size-inline,
    var(--ui-fallback-alert-icon-size-inline)
  );
  height: var(
    --ui-alert-icon-size-inline,
    var(--ui-fallback-alert-icon-size-inline)
  );
}

.ui-alert__body {
  min-width: 0;
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  gap: var(--ui-alert-body-gap, var(--ui-fallback-alert-body-gap));
}

.ui-alert__title {
  font-weight: 650;
  font-size: var(
    --ui-heading-h6-font-size,
    var(--ui-fallback-heading-h6-font-size)
  );
  line-height: var(
    --ui-heading-h6-line-height,
    var(--ui-fallback-heading-h6-line-height)
  );
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-alert--layout-inline .ui-alert__title,
.ui-alert[data-layout="inline"] .ui-alert__title {
  line-height: var(
    --ui-heading-h6-line-height-inline,
    var(--ui-fallback-heading-h6-line-height-inline)
  );
}

.ui-alert__description,
.ui-alert__content {
  font-size: var(
    --ui-alert-body-font-size,
    var(--ui-fallback-alert-body-font-size)
  );
  line-height: var(
    --ui-alert-body-line-height,
    var(--ui-fallback-alert-body-line-height)
  );
  color: color-mix(
    in oklch,
    var(--ui-fg, var(--ui-fallback-fg)) 84%,
    transparent
  );
}

.ui-alert--layout-inline .ui-alert__description,
.ui-alert--layout-inline .ui-alert__content,
.ui-alert[data-layout="inline"] .ui-alert__description,
.ui-alert[data-layout="inline"] .ui-alert__content {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-alert__start,
.ui-alert__end {
  flex: 0 0 auto;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-alert__sr-only {
  position: absolute;
  width: var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size));
  height: var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size));
  padding: 0;
  margin: calc(
    var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size)) * -1
  );
  overflow: hidden;
  clip: rect(
    0,
    var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size)),
    var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size)),
    0
  );
  white-space: nowrap;
  border: 0;
}

.ui-alert[data-icon="hidden"] .ui-alert__icon {
  display: none;
}

.ui-alert--tone-info .ui-alert__icon,
.ui-alert--tone-positive .ui-alert__icon,
.ui-alert--tone-notice .ui-alert__icon,
.ui-alert[data-tone="info"] .ui-alert__icon,
.ui-alert[data-tone="positive"] .ui-alert__icon,
.ui-alert[data-tone="notice"] .ui-alert__icon {
  color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-alert--tone-negative .ui-alert__icon,
.ui-alert[data-tone="negative"] .ui-alert__icon {
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-alert--fill-border.ui-alert--tone-info,
.ui-alert--fill-border.ui-alert--tone-positive,
.ui-alert--fill-border.ui-alert--tone-notice,
.ui-alert[data-fill="border"][data-tone="info"],
.ui-alert[data-fill="border"][data-tone="positive"],
.ui-alert[data-fill="border"][data-tone="notice"] {
  background: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 10%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
  );
  border-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 26%,
    var(--ui-border, var(--ui-fallback-border))
  );
}

.ui-alert--fill-border.ui-alert--tone-negative,
.ui-alert[data-fill="border"][data-tone="negative"] {
  background: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 10%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
  );
  border-color: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 30%,
    var(--ui-border, var(--ui-fallback-border))
  );
}

.ui-alert--fill-subtle.ui-alert--tone-info,
.ui-alert--fill-subtle.ui-alert--tone-positive,
.ui-alert--fill-subtle.ui-alert--tone-notice,
.ui-alert[data-fill="subtle"][data-tone="info"],
.ui-alert[data-fill="subtle"][data-tone="positive"],
.ui-alert[data-fill="subtle"][data-tone="notice"] {
  background: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 7%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
  );
  border-color: transparent;
}

.ui-alert--fill-subtle.ui-alert--tone-negative,
.ui-alert[data-fill="subtle"][data-tone="negative"] {
  background: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 7%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
  );
  border-color: transparent;
}

.ui-alert--fill-bold.ui-alert--tone-info,
.ui-alert--fill-bold.ui-alert--tone-positive,
.ui-alert--fill-bold.ui-alert--tone-notice,
.ui-alert[data-fill="bold"][data-tone="info"],
.ui-alert[data-fill="bold"][data-tone="positive"],
.ui-alert[data-fill="bold"][data-tone="notice"] {
  background: var(--ui-accent, var(--ui-fallback-accent));
  color: var(--ui-accent-fg, var(--ui-fallback-accent-fg));
  border-color: transparent;
}

.ui-alert--fill-bold.ui-alert--tone-negative,
.ui-alert[data-fill="bold"][data-tone="negative"] {
  background: var(--ui-danger, var(--ui-fallback-danger));
  color: var(--ui-danger-fg, var(--ui-fallback-danger-fg));
  border-color: transparent;
}

.ui-alert--fill-bold .ui-alert__title,
.ui-alert--fill-bold .ui-alert__description,
.ui-alert--fill-bold .ui-alert__content,
.ui-alert--fill-bold .ui-alert__icon,
.ui-alert[data-fill="bold"] .ui-alert__title,
.ui-alert[data-fill="bold"] .ui-alert__description,
.ui-alert[data-fill="bold"] .ui-alert__content,
.ui-alert[data-fill="bold"] .ui-alert__icon {
  color: inherit;
}
"#;
