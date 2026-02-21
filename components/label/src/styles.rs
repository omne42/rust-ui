pub const CSS: &str = r#"
.ui-label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;
  transition:
    color var(--ui-label-motion-color-duration, var(--ui-fallback-label-motion-color-duration)) var(--ui-label-motion-easing, var(--ui-fallback-label-motion-easing)),
    font-weight var(--ui-label-motion-weight-duration, var(--ui-fallback-label-motion-weight-duration)) var(--ui-label-motion-easing, var(--ui-fallback-label-motion-easing));
}

.ui-label--emphasis-default,
.ui-label[data-emphasis="default"] {
  font-weight: 500;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-label--emphasis-subtle,
.ui-label[data-emphasis="subtle"] {
  font-weight: 450;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-label--emphasis-strong,
.ui-label[data-emphasis="strong"] {
  font-weight: 600;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-label--required,
.ui-label[data-required="true"] {
  letter-spacing: normal;
}

.ui-label--disabled,
.ui-label[data-disabled="true"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-label--for,
.ui-label[data-has-for="true"] {
  cursor: pointer;
}

.ui-label--for:not(.ui-label--disabled):hover,
.ui-label[data-has-for="true"]:not([data-disabled="true"]):hover {
  color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-label--for:not(.ui-label--disabled):active,
.ui-label[data-has-for="true"]:not([data-disabled="true"]):active {
  color: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
}

.ui-label--text-custom,
.ui-label[data-label-source="custom"] {
  text-decoration: underline;
  text-underline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}

.ui-label--indicator-custom,
.ui-label[data-indicator-source="custom"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-label--custom-class,
.ui-label[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed var(--ui-accent-soft, var(--ui-fallback-accent-soft));
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-label__text {
  display: inline-flex;
  align-items: center;
}

.ui-label__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger, var(--ui-fallback-danger));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
}
"#;
