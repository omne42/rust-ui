pub const CSS: &str = r#"
.ui-legend {
  --ui-legend-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-legend-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-legend-strong-letter-spacing: var(
    --ui-command-group-heading-letter-spacing,
    var(--ui-fallback-command-group-heading-letter-spacing)
  );
  --ui-legend-required-letter-spacing: var(
    --ui-command-group-heading-letter-spacing,
    var(--ui-fallback-command-group-heading-letter-spacing)
  );
  --ui-legend-underline-offset: var(
    --ui-action-bar-clear-underline-offset,
    var(--ui-fallback-action-bar-clear-underline-offset)
  );
  --ui-legend-outline-width: var(
    --ui-button-focus-outline-width,
    var(--ui-fallback-button-focus-outline-width)
  );
  --ui-legend-outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );

  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  margin: 0;
  padding: 0;
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: var(--ui-button-size-l-font-size, var(--ui-fallback-font-size-150));
  line-height: var(--ui-button-size-l-line-height, var(--ui-fallback-line-height-150));
  font-weight: 600;
  transition:
    color var(--ui-legend-motion-duration) var(--ui-legend-motion-easing),
    opacity var(--ui-legend-motion-duration) var(--ui-legend-motion-easing);
}

.ui-legend--tone-default,
.ui-legend[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-legend--tone-muted,
.ui-legend[data-tone="muted"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-weight: 500;
}

.ui-legend--tone-strong,
.ui-legend[data-tone="strong"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
  letter-spacing: var(--ui-legend-strong-letter-spacing);
}

.ui-legend--required,
.ui-legend[data-required="true"] {
  letter-spacing: var(--ui-legend-required-letter-spacing);
}

.ui-legend--disabled,
.ui-legend[data-disabled="true"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-legend--text-custom,
.ui-legend[data-text-source="custom"] {
  text-decoration: underline;
  text-underline-offset: var(--ui-legend-underline-offset);
}

.ui-legend--indicator-custom,
.ui-legend[data-indicator-source="custom"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-legend--custom-class,
.ui-legend[data-custom-class="true"] {
  outline: var(--ui-legend-outline-width) dashed color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 30%, transparent);
  outline-offset: var(--ui-legend-outline-offset);
}

.ui-legend__text {
  display: inline-flex;
  align-items: center;
}

.ui-legend__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger, var(--ui-fallback-danger));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
}

@media (prefers-reduced-motion: reduce) {
  .ui-legend {
    --ui-legend-motion-duration: var(
      --ui-text-field-motion-duration,
      var(--ui-fallback-text-field-motion-duration)
    );
  }
}
"#;
