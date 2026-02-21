pub const CSS: &str = r#"
.ui-field-label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;
}

.ui-field-label--tone-default,
.ui-field-label[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-field-label--tone-muted,
.ui-field-label[data-tone="muted"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-weight: 450;
}

.ui-field-label--tone-strong,
.ui-field-label[data-tone="strong"] {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 92%,
    var(--ui-bg, var(--ui-fallback-bg)) 8%
  );
  font-weight: 600;
  letter-spacing: 0.01em;
}

.ui-field-label--required,
.ui-field-label[data-required="true"] {
  letter-spacing: 0.01em;
}

.ui-field-label--disabled,
.ui-field-label[data-disabled="true"] {
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 76%,
    var(--ui-bg, var(--ui-fallback-bg)) 24%
  );
}

.ui-field-label--for,
.ui-field-label[data-has-for="true"] {
  cursor: pointer;
}

.ui-field-label--for:not(.ui-field-label--disabled):hover,
.ui-field-label[data-has-for="true"][data-disabled="false"]:hover {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 88%,
    var(--ui-accent, var(--ui-fallback-accent)) 12%
  );
}

.ui-field-label--for:not(.ui-field-label--disabled):active,
.ui-field-label[data-has-for="true"][data-disabled="false"]:active {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 78%,
    var(--ui-accent, var(--ui-fallback-accent)) 22%
  );
}

.ui-field-label--for:not(.ui-field-label--disabled):focus-visible,
.ui-field-label[data-has-for="true"][data-disabled="false"]:focus-visible {
  outline: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2) solid
    color-mix(
      in oklab,
      var(--ui-accent, var(--ui-fallback-accent)) 56%,
      var(--ui-bg, var(--ui-fallback-bg)) 44%
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-field-label--text-custom,
.ui-field-label[data-text-source="custom"] {
  text-decoration: underline;
  text-underline-offset: 0.12em;
}

.ui-field-label--indicator-custom,
.ui-field-label[data-indicator-source="custom"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-field-label--aria-custom,
.ui-field-label[data-aria-source="custom"] {
  outline-offset: var(--ui-border-width, var(--ui-fallback-border-width));
}

.ui-field-label--custom-class,
.ui-field-label[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(
      in oklab,
      var(--ui-accent, var(--ui-fallback-accent)) 30%,
      var(--ui-bg, var(--ui-fallback-bg)) 70%
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-field-label__text {
  display: inline-flex;
  align-items: center;
}

.ui-field-label__required {
  display: inline-flex;
  align-items: center;
  color: var(--ui-danger, var(--ui-fallback-danger));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
}
"#;
