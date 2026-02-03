pub const CSS: &str = r#"
.ui-menu {
  display: flex;
  flex-direction: column;
  outline: none;
}

.ui-menu__items {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ui-menu__item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 10px;
  border-radius: 10px;
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-menu__item[data-disabled=\"true\"] {
  opacity: 0.5;
}

.ui-menu__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  flex-shrink: 0;
}
"#;
