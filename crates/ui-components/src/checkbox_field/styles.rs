pub const CSS: &str = r#"
.ui-checkbox-field {
  display: grid;
  gap: var(--ui-space-2xs);
  min-width: 0;
}

.ui-checkbox-field__checkbox {
  width: 100%;
  justify-content: flex-start;
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
  font-size: 0.75rem;
  line-height: 1.35;
  color: var(--ui-fg-muted);
  padding-inline-start: calc(20px + 10px);
}

.ui-checkbox-field--indicator-end .ui-checkbox-field__description,
.ui-checkbox-field[data-indicator-placement="end"] .ui-checkbox-field__description {
  padding-inline-start: 0;
  padding-inline-end: calc(20px + 10px);
  text-align: right;
}

.ui-checkbox-field--tone-default,
.ui-checkbox-field[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-checkbox-field--tone-quiet,
.ui-checkbox-field[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-checkbox-field--invalid .ui-checkbox-field__description,
.ui-checkbox-field[data-invalid="true"] .ui-checkbox-field__description {
  color: color-mix(in oklab, var(--ui-danger) 70%, var(--ui-fg) 30%);
}

.ui-checkbox-field--disabled,
.ui-checkbox-field[data-disabled="true"] {
  opacity: 0.7;
}

.ui-checkbox-field--custom-class,
.ui-checkbox-field[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
