pub const CSS: &str = r#"
.ui-form-field {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-sm);
  min-width: 0;
}

.ui-form-field__content {
  display: grid;
  gap: var(--ui-space-2xs);
  flex: 1;
  min-width: 0;
}

.ui-form-field__indicator {
  display: inline-flex;
  align-items: flex-start;
  padding-top: 1px;
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
  color: var(--ui-fg);
}

.ui-form-field--tone-quiet,
.ui-form-field[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-form-field__label {
  margin: 0;
  font-size: 0.9375rem;
  line-height: 1.3;
  font-weight: 600;
}

.ui-form-field__description,
.ui-form-field__error {
  margin: 0;
  font-size: 0.75rem;
  line-height: 1.35;
}

.ui-form-field__description {
  color: var(--ui-fg-muted);
}

.ui-form-field__error {
  color: color-mix(in oklab, var(--ui-danger) 72%, var(--ui-fg) 28%);
}

.ui-form-field--invalid .ui-form-field__label,
.ui-form-field[data-invalid="true"] .ui-form-field__label {
  color: color-mix(in oklab, var(--ui-danger) 64%, var(--ui-fg) 36%);
}

.ui-form-field--disabled,
.ui-form-field[data-disabled="true"] {
  opacity: 0.72;
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
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
