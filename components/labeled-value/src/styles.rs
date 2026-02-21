pub const CSS: &str = r#"
.ui-labeled-value {
  --ui-labeled-value-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-labeled-value-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-labeled-value-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-labeled-value-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-labeled-value-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-labeled-value-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-labeled-value-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-labeled-value-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-labeled-value-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  --ui-labeled-value-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-labeled-value-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  --ui-labeled-value-outline-width: var(
    --ui-button-focus-outline-width,
    var(--ui-fallback-button-focus-outline-width)
  );
  --ui-labeled-value-outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
  --ui-labeled-value-underline-offset: var(
    --ui-action-bar-clear-underline-offset,
    var(--ui-fallback-action-bar-clear-underline-offset)
  );
  display: flex;
  min-width: 0;
  color: var(--ui-labeled-value-fg);
}

.ui-labeled-value--orientation-stacked,
.ui-labeled-value[data-orientation="stacked"] {
  flex-direction: column;
  align-items: flex-start;
  gap: var(--ui-labeled-value-space-2xs);
}

.ui-labeled-value--orientation-inline,
.ui-labeled-value[data-orientation="inline"] {
  flex-direction: row;
  align-items: baseline;
  gap: var(--ui-labeled-value-space-xs);
}

.ui-labeled-value--tone-default,
.ui-labeled-value[data-tone="default"] {
  color: var(--ui-labeled-value-fg);
}

.ui-labeled-value--tone-subtle,
.ui-labeled-value[data-tone="subtle"] {
  color: var(--ui-labeled-value-fg-muted);
}

.ui-labeled-value--tone-strong,
.ui-labeled-value[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-labeled-value-fg) 90%, var(--ui-labeled-value-bg) 10%);
}

.ui-labeled-value--with-description,
.ui-labeled-value[data-has-description="true"] {
  row-gap: var(--ui-labeled-value-space-2xs);
}

.ui-labeled-value--label-custom,
.ui-labeled-value[data-label-source="custom"] {
  text-decoration: underline;
  text-underline-offset: var(--ui-labeled-value-underline-offset);
}

.ui-labeled-value--value-custom,
.ui-labeled-value[data-value-source="custom"] {
  font-feature-settings: "tnum";
}

.ui-labeled-value--aria-custom,
.ui-labeled-value[data-aria-source="custom"] {
  outline: var(--ui-labeled-value-outline-width) dashed color-mix(in oklab, var(--ui-labeled-value-accent) 24%, transparent);
  outline-offset: var(--ui-labeled-value-outline-offset);
}

.ui-labeled-value--custom-class,
.ui-labeled-value[data-custom-class="true"] {
  border-radius: var(--ui-labeled-value-radius-sm);
}

.ui-labeled-value__label {
  display: inline-flex;
  align-items: center;
  font-size: var(--ui-labeled-value-font-size-100);
  line-height: var(--ui-labeled-value-line-height-100);
  font-weight: 500;
  color: var(--ui-labeled-value-fg-muted);
}

.ui-labeled-value__value {
  display: inline-flex;
  align-items: center;
  min-width: 0;
  font-size: var(--ui-labeled-value-font-size-150);
  line-height: var(--ui-labeled-value-line-height-150);
  font-weight: 500;
  color: var(--ui-labeled-value-fg);
}

.ui-labeled-value__description {
  display: inline-flex;
  align-items: center;
  font-size: var(--ui-labeled-value-font-size-100);
  line-height: var(--ui-labeled-value-line-height-100);
  color: var(--ui-labeled-value-fg-muted);
}
"#;
