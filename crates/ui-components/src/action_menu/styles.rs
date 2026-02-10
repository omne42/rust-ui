pub const CSS: &str = r#"
.ui-action-menu {
  display: inline-flex;
}

.ui-action-menu--persistent .ui-action-button {
  box-shadow: var(--ui-shadow-sm);
}

.ui-action-menu--disabled {
  opacity: 0.72;
}

.ui-action-menu[data-motion-source="custom"],
.ui-action-menu[data-custom-motion="true"] {
  --ui-action-menu-custom-motion: 1;
}
"#;
