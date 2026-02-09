pub const CSS: &str = r#"
.ui-sidebar-menu-action {
  border: 0;
  background: transparent;
  color: inherit;
  font: inherit;
  border-radius: var(--ui-radius-xs, 0.375rem);
  padding: 0.25rem 0.35rem;
  line-height: 1;
}

.ui-sidebar-menu-action--hover-only,
.ui-sidebar-menu-action[data-hover-only="true"],
.ui-sidebar-menu-action[data-visibility="hover"] {
  opacity: 0.72;
}

.ui-sidebar-menu-action--disabled,
.ui-sidebar-menu-action[data-disabled="true"] {
  opacity: 0.52;
}

.ui-sidebar-menu-action:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  outline-offset: 1px;
}

.ui-sidebar-menu-action:hover {
  background: color-mix(in oklab, currentColor 10%, transparent);
}

.ui-sidebar-menu-action--custom-class,
.ui-sidebar-menu-action[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
