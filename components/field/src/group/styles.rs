pub const CSS: &str = r#"
.ui-field-group {
  --ui-field-group-min-inline-size-none: var(
    --ui-min-inline-size-none,
    var(--ui-fallback-min-inline-size-none)
  );
  --ui-field-group-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-field-group-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-field-group-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-field-group-space-lg: var(--ui-space-lg, var(--ui-fallback-space-lg));
  --ui-field-group-space-xl: var(--ui-space-xl, var(--ui-fallback-space-xl));
  --ui-field-group-radius-md: var(--ui-radius-md, var(--ui-fallback-radius-md));
  --ui-field-group-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-field-group-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-field-group-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-field-group-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-field-group-border: var(--ui-border, var(--ui-fallback-border));
  --ui-field-group-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-field-group-danger: var(--ui-danger, var(--ui-fallback-danger));
  --ui-field-group-horizontal-item-min-inline-size: calc(
    var(--ui-field-group-space-xl) + var(--ui-field-group-space-lg)
  );
  --ui-field-group-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-field-group-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );

  display: grid;
  gap: var(--ui-field-group-space-sm);
  min-width: var(--ui-field-group-min-inline-size-none);
  padding: var(--ui-field-group-space-sm);
  border-radius: var(--ui-field-group-radius-md);
  border: var(--ui-field-group-border-width) solid
    color-mix(in oklab, var(--ui-field-group-border) 80%, transparent);
  background: color-mix(in oklab, var(--ui-field-group-bg) 85%, transparent);
  transition:
    border-color var(--ui-field-group-motion-duration) var(--ui-field-group-motion-easing),
    background-color var(--ui-field-group-motion-duration) var(--ui-field-group-motion-easing),
    box-shadow var(--ui-field-group-motion-duration) var(--ui-field-group-motion-easing);
}

.ui-field-group[data-disabled="false"][data-invalid="false"]:hover {
  border-color: color-mix(
    in oklab,
    var(--ui-field-group-border) 58%,
    var(--ui-field-group-accent) 42%
  );
  background: color-mix(
    in oklab,
    var(--ui-field-group-bg) 84%,
    var(--ui-field-group-accent) 16%
  );
}

.ui-field-group[data-disabled="false"]:focus-within {
  border-color: color-mix(
    in oklab,
    var(--ui-field-group-accent) 56%,
    var(--ui-field-group-border) 44%
  );
  box-shadow: 0 0 0 var(--ui-field-group-border-width)
    color-mix(in oklab, var(--ui-field-group-accent) 26%, transparent);
}

.ui-field-group[data-disabled="false"]:focus-within .ui-field-group__label {
  color: color-mix(in oklab, var(--ui-field-group-fg) 76%, var(--ui-field-group-accent) 24%);
}

.ui-field-group--density-comfortable,
.ui-field-group[data-density="comfortable"] {
  gap: var(--ui-field-group-space-sm);
  padding: var(--ui-field-group-space-sm);
}

.ui-field-group--density-compact,
.ui-field-group[data-density="compact"] {
  gap: var(--ui-field-group-space-xs);
  padding: var(--ui-field-group-space-xs);
}

.ui-field-group__label {
  margin: 0;
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 600;
  color: var(--ui-field-group-fg);
}

.ui-field-group__content {
  display: grid;
  gap: var(--ui-field-group-space-sm);
  min-width: var(--ui-field-group-min-inline-size-none);
}

.ui-field-group--orientation-vertical .ui-field-group__content,
.ui-field-group[data-orientation="vertical"] .ui-field-group__content {
  grid-template-columns: minmax(var(--ui-field-group-min-inline-size-none), 1fr);
}

.ui-field-group--orientation-horizontal .ui-field-group__content,
.ui-field-group[data-orientation="horizontal"] .ui-field-group__content {
  grid-template-columns: repeat(
    auto-fit,
    minmax(var(--ui-field-group-horizontal-item-min-inline-size), 1fr)
  );
  align-items: start;
}

.ui-field-group__description {
  margin: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-field-group-fg-muted);
}

.ui-field-group--invalid,
.ui-field-group[data-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-field-group-danger) 64%, transparent);
}

.ui-field-group--invalid .ui-field-group__description,
.ui-field-group[data-invalid="true"] .ui-field-group__description {
  color: color-mix(in oklab, var(--ui-field-group-danger) 70%, var(--ui-field-group-fg) 30%);
}

.ui-field-group--disabled,
.ui-field-group[data-disabled="true"] {
  opacity: 0.68;
}

.ui-field-group--custom-class,
.ui-field-group[data-custom-class="true"] {
  outline: var(--ui-field-group-border-width) solid
    color-mix(in oklab, var(--ui-field-group-accent) 24%, transparent);
  outline-offset: var(--ui-field-group-space-2xs);
}
"#;
