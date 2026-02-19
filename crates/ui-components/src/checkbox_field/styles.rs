pub const CSS: &str = r#"
.ui-checkbox-field {
  display: grid;
  gap: var(--ui-space-2xs);
  min-width: 0;
  --ui-checkbox-field-transition-ms: 160ms;
  --ui-checkbox-field-indicator-scale: 1;
}

.ui-checkbox-field__checkbox {
  width: 100%;
  justify-content: flex-start;
  transform: scale(var(--ui-checkbox-field-indicator-scale));
  transform-origin: center;
  transition:
    transform var(--ui-checkbox-field-transition-ms) ease,
    opacity var(--ui-checkbox-field-transition-ms) ease;
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
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
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

.ui-checkbox-field[data-motion-source="custom"],
.ui-checkbox-field[data-custom-motion="true"],
.ui-checkbox-field--custom-motion {
  --ui-checkbox-field-motion-source: custom;
}
"#;
