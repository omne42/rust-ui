pub const CSS: &str = r#"
.ui-content {
  display: block;
  min-width: 0;
  color: inherit;
}

.ui-content--tone-default,
.ui-content[data-tone="default"] {
  color: inherit;
}

.ui-content--tone-muted,
.ui-content[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-content--padded,
.ui-content[data-padded="true"] {
  padding: var(--ui-space-md);
}

.ui-content--custom-class,
.ui-content[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
