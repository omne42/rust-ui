pub const CSS: &str = r#"
.ui-fieldset {
  display: grid;
  min-width: 0;
  margin: 0;
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) 0 0;
  border: none;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-fieldset[data-motion-source="custom"],
.ui-fieldset[data-custom-motion="true"] {
  --ui-fieldset-custom-motion: 1;
}

.ui-fieldset--orientation-vertical,
.ui-fieldset[data-orientation="vertical"] {
  grid-template-columns: minmax(0, 1fr);
  align-items: start;
}

.ui-fieldset--orientation-horizontal,
.ui-fieldset[data-orientation="horizontal"] {
  grid-template-columns:
    minmax(
      var(--ui-fieldset-horizontal-legend-min-inline-size, var(--ui-fallback-fieldset-horizontal-legend-min-inline-size)),
      var(--ui-fieldset-horizontal-legend-max-inline-size, var(--ui-fallback-fieldset-horizontal-legend-max-inline-size))
    )
    minmax(0, 1fr);
  align-items: start;
  column-gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-fieldset--tone-default,
.ui-fieldset[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-fieldset--tone-muted,
.ui-fieldset[data-tone="muted"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-fieldset--required .ui-fieldset__legend,
.ui-fieldset[data-required="true"] .ui-fieldset__legend {
  font-weight: 600;
}

.ui-fieldset--disabled,
.ui-fieldset[data-disabled="true"] {
  opacity: 0.72;
}

.ui-fieldset--invalid .ui-fieldset__group,
.ui-fieldset[data-invalid="true"] .ui-fieldset__group {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid color-mix(in oklab, var(--ui-danger, var(--ui-fallback-danger)) 44%, transparent);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-fieldset__legend {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  min-width: 0;
  margin: 0;
  padding: 0;
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
}

.ui-fieldset--orientation-horizontal .ui-fieldset__legend,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__legend {
  justify-content: flex-end;
}

.ui-fieldset__required-indicator {
  color: color-mix(in oklab, var(--ui-danger, var(--ui-fallback-danger)) 78%, var(--ui-fg, var(--ui-fallback-fg)) 22%);
}

.ui-fieldset__group {
  min-width: 0;
  display: grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-fieldset__actions {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-fieldset--orientation-horizontal .ui-fieldset__actions,
.ui-fieldset--orientation-horizontal .ui-fieldset__description,
.ui-fieldset--orientation-horizontal .ui-fieldset__error,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__actions,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__description,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__error {
  grid-column: 2;
}

.ui-fieldset__description,
.ui-fieldset__error {
  margin: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  transition:
    opacity var(--ui-fieldset-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))) var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    transform var(--ui-fieldset-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))) var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
  transform: translateY(var(--ui-fieldset-motion-distance, var(--ui-space-2xs, var(--ui-fallback-space-2xs))));
  opacity: 1;
}

.ui-fieldset__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-fieldset__error {
  color: color-mix(in oklab, var(--ui-danger, var(--ui-fallback-danger)) 74%, var(--ui-fg, var(--ui-fallback-fg)) 26%);
}

.ui-fieldset--custom-class,
.ui-fieldset[data-custom-class="true"],
.ui-fieldset[data-class-source="custom"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
