pub const CSS: &str = r#"
.ui-header {
  --ui-header-motion-duration: 180ms;
  display: block;
  min-width: 0;
  color: var(--ui-fg);
  font-weight: 600;
  font-size: 1rem;
  line-height: 1.35;
  transition:
    color var(--ui-header-motion-duration) ease,
    border-color var(--ui-header-motion-duration) ease;
}

.ui-header--tone-default,
.ui-header[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-header--tone-strong,
.ui-header[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 78%, var(--ui-accent) 22%);
}

.ui-header--bordered,
.ui-header[data-bordered="true"] {
  border-bottom: 1px solid var(--ui-border);
  padding-bottom: var(--ui-space-sm);
}

.ui-header--custom-class,
.ui-header[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
