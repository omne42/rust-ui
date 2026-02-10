pub const CSS: &str = r#"
.ui-theme-toggle-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-theme-toggle-button__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;

  --ui-theme-toggle-rotate: 0deg;
  --ui-theme-toggle-scale: 1;

  transform: rotate(var(--ui-theme-toggle-rotate)) scale(var(--ui-theme-toggle-scale));
  transform-origin: center;
  will-change: transform;
}

.ui-theme-toggle-button__icon[data-motion-source="custom"],
.ui-theme-toggle-button__icon[data-custom-motion="true"] {
  --ui-theme-toggle-custom-motion: 1;
}

.ui-theme-toggle-button svg {
  width: 18px;
  height: 18px;
}
"#;
