pub const CSS: &str = r#"
.ui-description {
  margin: 0;
  min-width: 0;
  font-size: 0.75rem;
  line-height: 1.35;
  color: var(--ui-fg-muted);
}

.ui-description--tone-default,
.ui-description[data-tone="default"] {
  color: var(--ui-fg-muted);
}

.ui-description--tone-muted,
.ui-description[data-tone="muted"] {
  color: color-mix(in oklab, var(--ui-fg-muted) 76%, var(--ui-bg) 24%);
}

.ui-description--tone-negative,
.ui-description[data-tone="negative"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-description--disabled,
.ui-description[data-disabled="true"] {
  opacity: 0.68;
}

.ui-description--truncate,
.ui-description[data-truncate="true"] {
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ui-description--custom-class,
.ui-description[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
