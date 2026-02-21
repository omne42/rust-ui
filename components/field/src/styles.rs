pub const CSS: &str = r#"
.ui-field {
  --ui-field-min-inline-size-none: var(
    --ui-min-inline-size-none,
    var(--ui-fallback-min-inline-size-none)
  );
  --ui-field-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-field-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-field-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-field-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-field-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-field-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-field-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-field-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-field-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-field-border: var(--ui-border, var(--ui-fallback-border));
  --ui-field-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-field-danger: var(--ui-danger, var(--ui-fallback-danger));
  --ui-field-horizontal-label-min-inline-size: var(
    --ui-space-xl,
    var(--ui-fallback-space-xl)
  );
  --ui-field-horizontal-label-max-inline-size: calc(
    var(--ui-space-xl, var(--ui-fallback-space-xl)) +
      var(--ui-space-lg, var(--ui-fallback-space-lg))
  );
  --ui-field-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-field-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );

  display: grid;
  min-width: var(--ui-field-min-inline-size-none);
  gap: var(--ui-field-space-xs);
  color: var(--ui-field-fg);
  transition:
    color var(--ui-field-motion-duration) var(--ui-field-motion-easing),
    opacity var(--ui-field-motion-duration) var(--ui-field-motion-easing);
}

.ui-field--orientation-vertical,
.ui-field[data-orientation="vertical"] {
  grid-template-columns: minmax(var(--ui-field-min-inline-size-none), 1fr);
  align-items: start;
}

.ui-field--orientation-horizontal,
.ui-field[data-orientation="horizontal"] {
  grid-template-columns:
    minmax(
      var(--ui-field-horizontal-label-min-inline-size),
      var(--ui-field-horizontal-label-max-inline-size)
    )
    minmax(var(--ui-field-min-inline-size-none), 1fr);
  align-items: center;
  column-gap: var(--ui-field-space-md);
}

.ui-field--tone-default,
.ui-field[data-tone="default"] {
  color: var(--ui-field-fg);
}

.ui-field--tone-muted,
.ui-field[data-tone="muted"] {
  color: var(--ui-field-fg-muted);
}

.ui-field--required .ui-field__label,
.ui-field[data-required="true"] .ui-field__label {
  font-weight: 600;
}

.ui-field--disabled,
.ui-field[data-disabled="true"] {
  opacity: 0.72;
}

.ui-field--invalid .ui-field__control,
.ui-field[data-invalid="true"] .ui-field__control {
  outline: var(--ui-field-border-width) solid
    color-mix(in oklab, var(--ui-field-danger) 44%, transparent);
  border-radius: var(--ui-field-radius-sm);
}

.ui-field__label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-field-space-2xs);
  min-width: var(--ui-field-min-inline-size-none);
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  transition: color var(--ui-field-motion-duration) var(--ui-field-motion-easing);
}

.ui-field--orientation-horizontal .ui-field__label,
.ui-field[data-orientation="horizontal"] .ui-field__label {
  justify-content: flex-end;
}

.ui-field__required-indicator {
  color: color-mix(in oklab, var(--ui-field-danger) 78%, var(--ui-field-fg) 22%);
}

.ui-field__control {
  min-width: var(--ui-field-min-inline-size-none);
  border-radius: var(--ui-field-radius-sm);
  transition:
    background-color var(--ui-field-motion-duration) var(--ui-field-motion-easing),
    outline-color var(--ui-field-motion-duration) var(--ui-field-motion-easing);
}

.ui-field[data-disabled="false"][data-invalid="false"]:hover .ui-field__label {
  color: color-mix(in oklab, var(--ui-field-fg) 84%, var(--ui-field-accent) 16%);
}

.ui-field[data-disabled="false"][data-invalid="false"]:hover .ui-field__control {
  background: color-mix(in oklab, var(--ui-field-bg) 90%, var(--ui-field-accent) 10%);
}

.ui-field[data-disabled="false"]:focus-within .ui-field__label {
  color: color-mix(in oklab, var(--ui-field-fg) 76%, var(--ui-field-accent) 24%);
}

.ui-field[data-disabled="false"][data-invalid="false"]:focus-within .ui-field__control {
  outline: var(--ui-field-border-width) solid
    color-mix(in oklab, var(--ui-field-accent) 52%, var(--ui-field-border) 48%);
  outline-offset: var(--ui-field-space-2xs);
}

.ui-field__description,
.ui-field__error {
  margin: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-field--orientation-horizontal .ui-field__description,
.ui-field--orientation-horizontal .ui-field__error,
.ui-field[data-orientation="horizontal"] .ui-field__description,
.ui-field[data-orientation="horizontal"] .ui-field__error {
  grid-column: 2;
}

.ui-field__description {
  color: var(--ui-field-fg-muted);
}

.ui-field__error {
  color: color-mix(in oklab, var(--ui-field-danger) 74%, var(--ui-field-fg) 26%);
}

.ui-field[data-message-kind="description"] .ui-field__description {
  color: var(--ui-field-fg-muted);
}

.ui-field[data-message-kind="error"] .ui-field__error {
  color: color-mix(in oklab, var(--ui-field-danger) 74%, var(--ui-field-fg) 26%);
}

.ui-field--custom-class,
.ui-field[data-custom-class="true"] {
  outline: var(--ui-field-border-width) solid
    color-mix(in oklab, var(--ui-field-accent) 24%, transparent);
  outline-offset: var(--ui-field-space-2xs);
}

@media (prefers-reduced-motion: reduce) {
  .ui-field {
    --ui-field-motion-duration: 1ms;
  }
}
"#;
