pub const CSS: &str = r#"
.ui-help-text {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-xs);
  min-width: 0;
  font-size: 0.75rem;
  line-height: 1.35;
  color: var(--ui-fg-muted);
}

.ui-help-text--tone-auto,
.ui-help-text--tone-neutral,
.ui-help-text[data-tone="auto"],
.ui-help-text[data-tone="neutral"] {
  color: var(--ui-fg-muted);
}

.ui-help-text--tone-negative,
.ui-help-text[data-tone="negative"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-help-text--invalid,
.ui-help-text[data-invalid="true"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-help-text--disabled,
.ui-help-text[data-disabled="true"] {
  opacity: 0.68;
}

.ui-help-text__icon {
  margin-top: 0.0625rem;
  font-size: 0.875rem;
  line-height: 1;
}

.ui-help-text__text {
  margin: 0;
  min-width: 0;
}

.ui-help-text--custom-class,
.ui-help-text[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
