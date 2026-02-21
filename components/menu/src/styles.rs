pub const CSS: &str = r#"
.ui-menu {
  display: flex;
  flex-direction: column;
  outline: none;
}

.ui-menu[data-motion-source="custom"],
.ui-menu[data-custom-motion="true"] {
  --ui-menu-custom-motion: 1;
}

.ui-menu__items {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-menu__item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-xs, var(--ui-fallback-space-xs))
    var(--ui-space-sm, var(--ui-fallback-space-sm));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-menu__item[data-disabled=\"true\"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-menu__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-icon-size-100, var(--ui-fallback-icon-size-100));
  flex-shrink: 0;
}
"#;
