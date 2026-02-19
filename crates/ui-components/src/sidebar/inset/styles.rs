pub const CSS: &str = r#"
.ui-sidebar-inset {
  display: grid;
  gap: 0.5rem;
  min-width: 0;
  border-radius: var(--ui-radius-md, 0.75rem);
  transition: background 180ms ease, border-color 180ms ease;
}

.ui-sidebar-inset--padded,
.ui-sidebar-inset[data-padded="true"] {
  padding: 0.75rem;
}

.ui-sidebar-inset--recessed,
.ui-sidebar-inset[data-recessed="true"] {
  background: color-mix(in oklab, var(--ui-bg-canvas, white) 90%, transparent);
  border: 1px solid var(--ui-border-subtle, color-mix(in oklab, currentColor 18%, transparent));
}

.ui-sidebar-inset--left,
.ui-sidebar-inset[data-side="left"] {
  border-inline-start-width: 2px;
}

.ui-sidebar-inset--right,
.ui-sidebar-inset[data-side="right"] {
  border-inline-end-width: 2px;
}

.ui-sidebar-inset--disabled,
.ui-sidebar-inset[data-disabled="true"] {
  opacity: 0.62;
}

.ui-sidebar-inset--custom-class,
.ui-sidebar-inset[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
