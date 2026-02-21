pub const CSS: &str = r#"
.ui-dropdown-menu {
  display: inline-flex;
  align-items: center;
}

.ui-dropdown-menu--persistent .ui-button {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-dropdown-menu--disabled {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-dropdown-menu[data-motion-source="custom"],
.ui-dropdown-menu[data-custom-motion="true"] {
  --ui-dropdown-menu-custom-motion: 1;
}
"#;
