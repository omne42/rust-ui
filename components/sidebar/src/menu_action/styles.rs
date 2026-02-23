pub const CSS: &str = r#"
.ui-sidebar-menu-action {
  --ui-sidebar-menu-action-radius-xs: var(
    --ui-radius-xs,
    var(--ui-radius-sm, var(--ui-fallback-radius-sm))
  );
  --ui-sidebar-menu-action-accent-solid: var(
    --ui-accent-solid,
    var(--ui-accent, var(--ui-fallback-accent))
  );
  --ui-sidebar-menu-action-hover-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 44%,
    transparent
  );
  --ui-sidebar-menu-action-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  border: 0;
  background: transparent;
  color: inherit;
  font: inherit;
  border-radius: var(--ui-sidebar-menu-action-radius-xs);
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
  opacity: var(--ui-sidebar-menu-action-disabled-opacity);
}

.ui-sidebar-menu-action:focus-visible {
  outline: 2px solid var(--ui-sidebar-menu-action-accent-solid);
  outline-offset: 1px;
}

.ui-sidebar-menu-action:hover {
  background: var(--ui-sidebar-menu-action-hover-bg);
}

.ui-sidebar-menu-action--custom-class,
.ui-sidebar-menu-action[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
