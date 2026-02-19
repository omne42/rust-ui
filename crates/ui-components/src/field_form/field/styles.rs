pub const CSS: &str = r#"
.ui-field {
  --ui-field-motion-duration: 160ms;

  display: grid;
  min-width: 0;
  gap: var(--ui-space-xs);
  color: var(--ui-fg);
  transition:
    color var(--ui-field-motion-duration) ease,
    opacity var(--ui-field-motion-duration) ease;
}

.ui-field--orientation-vertical,
.ui-field[data-orientation="vertical"] {
  grid-template-columns: minmax(0, 1fr);
  align-items: start;
}

.ui-field--orientation-horizontal,
.ui-field[data-orientation="horizontal"] {
  grid-template-columns: minmax(8rem, 14rem) minmax(0, 1fr);
  align-items: center;
  column-gap: var(--ui-space-md);
}

.ui-field--tone-default,
.ui-field[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-field--tone-muted,
.ui-field[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-field--required .ui-field__label,
.ui-field[data-required="true"] .ui-field__label {
  font-weight: 600;
}

.ui-field--disabled,
.ui-field[data-disabled="true"] {
  opacity: 0.72;
}

.ui-field--invalid .ui-field__control,
.ui-field[data-invalid="true"] .ui-field__control {
  outline: 1px solid color-mix(in oklab, var(--ui-danger) 44%, transparent);
  border-radius: var(--ui-radius-sm);
}

.ui-field__label {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, 4px);
  min-width: 0;
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-field--orientation-horizontal .ui-field__label,
.ui-field[data-orientation="horizontal"] .ui-field__label {
  justify-content: flex-end;
}

.ui-field__required-indicator {
  color: color-mix(in oklab, var(--ui-danger) 78%, var(--ui-fg) 22%);
}

.ui-field__control {
  min-width: 0;
}

.ui-field__description,
.ui-field__error {
  margin: 0;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-field--orientation-horizontal .ui-field__description,
.ui-field--orientation-horizontal .ui-field__error,
.ui-field[data-orientation="horizontal"] .ui-field__description,
.ui-field[data-orientation="horizontal"] .ui-field__error {
  grid-column: 2;
}

.ui-field__description {
  color: var(--ui-fg-muted);
}

.ui-field__error {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-field--custom-class,
.ui-field[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}

@media (prefers-reduced-motion: reduce) {
  .ui-field {
    --ui-field-motion-duration: 1ms;
  }
}
"#;
