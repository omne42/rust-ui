pub const CSS: &str = r#"
.ui-dropdown-menu {
  display: inline-flex;
  align-items: center;
}

.ui-dropdown-menu--persistent .ui-button {
  box-shadow: var(--ui-shadow-sm);
}

.ui-dropdown-menu--disabled {
  opacity: 0.72;
}

.ui-dropdown-menu[data-motion-source="custom"],
.ui-dropdown-menu[data-custom-motion="true"] {
  --ui-dropdown-menu-custom-motion: 1;
}
"#;
