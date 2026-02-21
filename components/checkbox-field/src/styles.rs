pub const CSS: &str = r#"
.ui-checkbox-field {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  min-width: 0;
  --ui-checkbox-field-transition-ms: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));
  --ui-checkbox-field-transition-easing: var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
  --ui-checkbox-field-description-offset: calc(
    var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default)) +
      var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap))
  );
  --ui-checkbox-field-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
  --ui-checkbox-field-custom-outline-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-checkbox-field-custom-outline-offset: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2);
  --ui-checkbox-field-indicator-scale: 1;
}

.ui-checkbox-field__checkbox {
  width: 100%;
  justify-content: flex-start;
  transform: scale(var(--ui-checkbox-field-indicator-scale));
  transform-origin: center;
  transition:
    transform var(--ui-checkbox-field-transition-ms) var(--ui-checkbox-field-transition-easing),
    opacity var(--ui-checkbox-field-transition-ms) var(--ui-checkbox-field-transition-easing);
}

.ui-checkbox-field__checkbox .ui-checkbox__label {
  flex: 1;
  text-align: left;
}

.ui-checkbox-field--indicator-end .ui-checkbox-field__checkbox,
.ui-checkbox-field[data-indicator-placement="end"] .ui-checkbox-field__checkbox,
.ui-checkbox-field__checkbox--indicator-end {
  flex-direction: row-reverse;
  justify-content: space-between;
}

.ui-checkbox-field__description {
  margin: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  padding-inline-start: var(--ui-checkbox-field-description-offset);
}

.ui-checkbox-field--indicator-end .ui-checkbox-field__description,
.ui-checkbox-field[data-indicator-placement="end"] .ui-checkbox-field__description {
  padding-inline-start: 0;
  padding-inline-end: var(--ui-checkbox-field-description-offset);
  text-align: right;
}

.ui-checkbox-field--tone-default,
.ui-checkbox-field[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-checkbox-field--tone-quiet,
.ui-checkbox-field[data-tone="quiet"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-checkbox-field--invalid .ui-checkbox-field__description,
.ui-checkbox-field[data-invalid="true"] .ui-checkbox-field__description {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 70%,
    var(--ui-fg, var(--ui-fallback-fg)) 30%
  );
}

.ui-checkbox-field--disabled,
.ui-checkbox-field[data-disabled="true"] {
  opacity: var(--ui-checkbox-field-disabled-opacity);
}

.ui-checkbox-field--custom-class,
.ui-checkbox-field[data-custom-class="true"] {
  outline:
    var(--ui-checkbox-field-custom-outline-width) solid
      color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-checkbox-field-custom-outline-offset);
}

.ui-checkbox-field[data-motion-source="custom"],
.ui-checkbox-field[data-custom-motion="true"],
.ui-checkbox-field--custom-motion {
  --ui-checkbox-field-motion-source: custom;
}
"#;
