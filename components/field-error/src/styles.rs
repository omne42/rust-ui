pub const CSS: &str = r#"
.ui-field-error {
  margin: 0;
  min-width: 0;
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-xs);
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}

.ui-field-error--tone-auto,
.ui-field-error--tone-neutral,
.ui-field-error[data-tone="auto"],
.ui-field-error[data-tone="neutral"] {
  color: var(--ui-fg-muted);
}

.ui-field-error--tone-negative,
.ui-field-error[data-tone="negative"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-field-error[data-state="hidden"] {
  display: none;
}

.ui-field-error--disabled,
.ui-field-error[data-disabled="true"] {
  opacity: 0.68;
}

.ui-field-error__icon {
  margin-top: 0.0625rem;
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-field-error__text {
  margin: 0;
  min-width: 0;
}

.ui-field-error--custom-class,
.ui-field-error[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
