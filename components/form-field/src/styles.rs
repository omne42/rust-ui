pub const CSS: &str = r#"
.ui-form-field {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-width: 0;
}

.ui-form-field__content {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  flex: 1;
  min-width: 0;
}

.ui-form-field__indicator {
  display: inline-flex;
  align-items: flex-start;
  padding-top: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}

.ui-form-field--placement-end,
.ui-form-field[data-indicator-placement="end"] {
  justify-content: space-between;
}

.ui-form-field--placement-start,
.ui-form-field[data-indicator-placement="start"] {
  justify-content: flex-start;
}

.ui-form-field--placement-end .ui-form-field__content,
.ui-form-field[data-indicator-placement="end"] .ui-form-field__content {
  order: 1;
}

.ui-form-field--placement-end .ui-form-field__indicator,
.ui-form-field[data-indicator-placement="end"] .ui-form-field__indicator {
  order: 2;
}

.ui-form-field--placement-start .ui-form-field__indicator,
.ui-form-field[data-indicator-placement="start"] .ui-form-field__indicator {
  order: 1;
}

.ui-form-field--placement-start .ui-form-field__content,
.ui-form-field[data-indicator-placement="start"] .ui-form-field__content {
  order: 2;
}

.ui-form-field--tone-default,
.ui-form-field[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-form-field--tone-quiet,
.ui-form-field[data-tone="quiet"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-form-field__label {
  margin: 0;
  font-size: var(--ui-button-size-l-font-size, var(--ui-fallback-button-size-l-font-size));
  line-height: var(--ui-button-size-l-line-height, var(--ui-fallback-button-size-l-line-height));
  font-weight: 600;
}

.ui-form-field__description,
.ui-form-field__error {
  margin: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-form-field__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-form-field__error {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 72%,
    var(--ui-fg, var(--ui-fallback-fg)) 28%
  );
}

.ui-form-field--invalid .ui-form-field__label,
.ui-form-field[data-invalid="true"] .ui-form-field__label {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 64%,
    var(--ui-fg, var(--ui-fallback-fg)) 36%
  );
}

.ui-form-field--disabled,
.ui-form-field[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-form-field__control.ui-switch .ui-switch__label,
.ui-form-field__control.ui-checkbox .ui-checkbox__label {
  display: none;
}

.ui-form-field__control.ui-switch,
.ui-form-field__control.ui-checkbox {
  gap: 0;
}

.ui-form-field--custom-class,
.ui-form-field[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width))
    solid color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
