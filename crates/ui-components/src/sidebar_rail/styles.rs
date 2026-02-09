pub const CSS: &str = r#"
.ui-sidebar-rail {
  align-self: stretch;
  width: 0.625rem;
  min-height: 2.5rem;
  border: 0;
  border-radius: 999px;
  background: color-mix(in oklab, currentColor 12%, transparent);
  cursor: pointer;
  transition: background 160ms ease, transform 160ms ease;
}

.ui-sidebar-rail:hover {
  background: color-mix(in oklab, var(--ui-accent-solid, currentColor) 30%, transparent);
}

.ui-sidebar-rail:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  outline-offset: 1px;
}

.ui-sidebar-rail--right,
.ui-sidebar-rail[data-side="right"] {
  justify-self: end;
}

.ui-sidebar-rail--closed,
.ui-sidebar-rail[data-closed="true"] {
  transform: scaleY(0.92);
}

.ui-sidebar-rail--disabled,
.ui-sidebar-rail[data-disabled="true"] {
  opacity: 0.62;
  transform: none;
  cursor: default;
}

.ui-sidebar-rail--custom-class,
.ui-sidebar-rail[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
