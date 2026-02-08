pub const CSS: &str = r#"
.ui-footer {
  display: block;
  min-width: 0;
  color: var(--ui-fg-muted);
  font-size: 0.875rem;
  line-height: 1.4;
}

.ui-footer--tone-default,
.ui-footer[data-tone="default"] {
  color: var(--ui-fg-muted);
}

.ui-footer--tone-muted,
.ui-footer[data-tone="muted"] {
  color: color-mix(in oklab, var(--ui-fg-muted) 88%, var(--ui-fg) 12%);
}

.ui-footer--bordered,
.ui-footer[data-bordered="true"] {
  border-top: 1px solid var(--ui-border);
  padding-top: var(--ui-space-sm);
}

.ui-footer--custom-class,
.ui-footer[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
