pub const CSS: &str = r#"
.ui-error-message {
  margin: 0;
  min-width: 0;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-error-message--tone-auto,
.ui-error-message--tone-negative,
.ui-error-message[data-tone="auto"],
.ui-error-message[data-tone="negative"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-error-message--tone-neutral,
.ui-error-message[data-tone="neutral"] {
  color: var(--ui-fg-muted);
}

.ui-error-message--disabled,
.ui-error-message[data-disabled="true"] {
  opacity: 0.68;
}

.ui-error-message--truncate,
.ui-error-message[data-truncate="true"] {
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ui-error-message--custom-class,
.ui-error-message[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
