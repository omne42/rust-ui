pub const CSS: &str = r#"
.ui-labeled-value {
  display: flex;
  min-width: 0;
  color: var(--ui-fg);
}

.ui-labeled-value--orientation-stacked,
.ui-labeled-value[data-orientation="stacked"] {
  flex-direction: column;
  align-items: flex-start;
  gap: var(--ui-space-2xs);
}

.ui-labeled-value--orientation-inline,
.ui-labeled-value[data-orientation="inline"] {
  flex-direction: row;
  align-items: baseline;
  gap: var(--ui-space-xs);
}

.ui-labeled-value--tone-default,
.ui-labeled-value[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-labeled-value--tone-subtle,
.ui-labeled-value[data-tone="subtle"] {
  color: var(--ui-fg-muted);
}

.ui-labeled-value--tone-strong,
.ui-labeled-value[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 90%, black 10%);
}

.ui-labeled-value--with-description,
.ui-labeled-value[data-has-description="true"] {
  row-gap: var(--ui-space-2xs);
}

.ui-labeled-value--label-custom,
.ui-labeled-value[data-label-source="custom"] {
  text-decoration: underline;
  text-underline-offset: 0.12em;
}

.ui-labeled-value--value-custom,
.ui-labeled-value[data-value-source="custom"] {
  font-feature-settings: "tnum";
}

.ui-labeled-value--aria-custom,
.ui-labeled-value[data-aria-source="custom"] {
  outline: 1px dashed color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}

.ui-labeled-value--custom-class,
.ui-labeled-value[data-custom-class="true"] {
  border-radius: var(--ui-radius-sm);
}

.ui-labeled-value__label {
  display: inline-flex;
  align-items: center;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 500;
  color: var(--ui-fg-muted);
}

.ui-labeled-value__value {
  display: inline-flex;
  align-items: center;
  min-width: 0;
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-labeled-value__description {
  display: inline-flex;
  align-items: center;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}
"#;
