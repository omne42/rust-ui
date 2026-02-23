pub const CSS: &str = r#"
.ui-date-field {
  display: grid;
  gap: var(--ui-space-2xs);
  width: min(100%, 18rem);
}

.ui-date-field--tone-default,
.ui-date-field[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-date-field--tone-quiet,
.ui-date-field[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-date-field--tone-strong,
.ui-date-field[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 82%, var(--ui-accent) 18%);
}

.ui-date-field--disabled,
.ui-date-field[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity);
}

.ui-date-field--has-value,
.ui-date-field[data-has-value="true"] .ui-date-field__control {
  border-color: color-mix(in oklab, var(--ui-accent) 38%, var(--ui-border) 62%);
}

.ui-date-field--custom-class,
.ui-date-field[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 22%, transparent);
}

.ui-date-field__label {
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
  font-weight: 600;
}

.ui-date-field__control {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  width: fit-content;
  padding: var(--ui-space-3xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
}

.ui-date-field__input {
  padding: var(--ui-space-3xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-xs);
  background: var(--ui-bg);
  color: var(--ui-fg);
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.ui-date-field__input--year {
  width: 4.8rem;
}

.ui-date-field__input--month,
.ui-date-field__input--day {
  width: 3.2rem;
}

.ui-date-field__input:disabled {
  color: var(--ui-fg-muted);
}

.ui-date-field__separator {
  color: var(--ui-fg-muted);
  font-weight: 600;
}

.ui-date-field__clear {
  margin-inline-start: var(--ui-space-2xs);
  padding: var(--ui-space-3xs) var(--ui-space-2xs);
  border: 1px solid transparent;
  border-radius: var(--ui-radius-xs);
  background: transparent;
  color: var(--ui-fg-muted);
  cursor: pointer;
}

.ui-date-field__clear:hover,
.ui-date-field__clear:focus-visible {
  border-color: color-mix(in oklab, var(--ui-accent) 45%, var(--ui-border) 55%);
  color: var(--ui-fg);
  outline: none;
}

.ui-date-field__clear:disabled {
  cursor: not-allowed;
  opacity: var(--ui-disabled-opacity);
}
"#;

#[cfg(test)]
#[path = "../../test/date_field/styles.rs"]
mod tests;
