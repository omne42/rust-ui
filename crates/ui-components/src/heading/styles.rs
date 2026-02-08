pub const CSS: &str = r#"
.ui-heading {
  display: block;
  margin: 0;
  min-width: 0;
  color: var(--ui-fg);
  line-height: 1.25;
  font-weight: 600;
}

.ui-heading--tone-default,
.ui-heading[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-heading--tone-strong,
.ui-heading[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 78%, var(--ui-accent) 22%);
}

.ui-heading--tone-muted,
.ui-heading[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-heading--level-1,
.ui-heading[data-level="1"] {
  font-size: 2rem;
}

.ui-heading--level-2,
.ui-heading[data-level="2"] {
  font-size: 1.5rem;
}

.ui-heading--level-3,
.ui-heading[data-level="3"] {
  font-size: 1.25rem;
}

.ui-heading--level-4,
.ui-heading[data-level="4"] {
  font-size: 1.125rem;
}

.ui-heading--level-5,
.ui-heading[data-level="5"] {
  font-size: 1rem;
}

.ui-heading--level-6,
.ui-heading[data-level="6"] {
  font-size: 0.875rem;
}

.ui-heading--truncate,
.ui-heading[data-truncate="true"] {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ui-heading--custom-class,
.ui-heading[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
