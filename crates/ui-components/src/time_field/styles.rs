pub const CSS: &str = r#"
.ui-time-field {
  --ui-time-field-clear-opacity: 1;
  --ui-time-field-clear-scale: 1;
  display: grid;
  gap: var(--ui-space-2xs);
  width: min(100%, 16rem);
}

.ui-time-field--tone-default,
.ui-time-field[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-time-field--tone-quiet,
.ui-time-field[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-time-field--tone-strong,
.ui-time-field[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 82%, var(--ui-accent) 18%);
}

.ui-time-field--disabled,
.ui-time-field[data-disabled="true"] {
  opacity: 0.7;
}

.ui-time-field--has-value,
.ui-time-field[data-has-value="true"] .ui-time-field__control {
  border-color: color-mix(in oklab, var(--ui-accent) 38%, var(--ui-border) 62%);
}

.ui-time-field--custom-class,
.ui-time-field[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 20%, transparent);
}

.ui-time-field[data-motion-source="custom"] .ui-time-field__clear {
  will-change: opacity, transform;
}

.ui-time-field[data-custom-motion="true"] .ui-time-field__clear {
  will-change: opacity, transform;
}

.ui-time-field__label {
  font-size: var(--ui-font-size-100);
  font-weight: 600;
}

.ui-time-field__control {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  width: fit-content;
  padding: var(--ui-space-3xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
}

.ui-time-field__input {
  width: calc(var(--ui-component-height-100) * 1.7);
  padding: var(--ui-space-3xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-xs);
  background: var(--ui-bg);
  color: var(--ui-fg);
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.ui-time-field__input:disabled {
  color: var(--ui-fg-muted);
}

.ui-time-field__separator {
  color: var(--ui-fg-muted);
  font-weight: 600;
}

.ui-time-field__clear {
  margin-inline-start: var(--ui-space-2xs);
  padding: var(--ui-space-3xs) var(--ui-space-2xs);
  border: 1px solid transparent;
  border-radius: var(--ui-radius-xs);
  background: transparent;
  color: var(--ui-fg-muted);
  opacity: var(--ui-time-field-clear-opacity);
  transform: scale(var(--ui-time-field-clear-scale));
  cursor: pointer;
}

.ui-time-field__clear[data-visible="true"] {
  visibility: visible;
  pointer-events: auto;
}

.ui-time-field__clear:not([data-visible="true"]) {
  visibility: hidden;
  pointer-events: none;
}

.ui-time-field__clear:hover,
.ui-time-field__clear:focus-visible {
  border-color: color-mix(in oklab, var(--ui-accent) 45%, var(--ui-border) 55%);
  color: var(--ui-fg);
  outline: none;
}

.ui-time-field__clear:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}
"#;
