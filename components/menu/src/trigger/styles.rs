pub const CSS: &str = r#"
.ui-menu-trigger {
  display: inline-block;
}

.ui-menu-trigger--persistent .ui-button {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-menu-trigger--disabled {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-menu-trigger[data-motion-source="custom"],
.ui-menu-trigger[data-custom-motion="true"] {
  --ui-menu-trigger-custom-motion: 1;
}
"#;
