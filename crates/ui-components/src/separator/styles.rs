pub const CSS: &str = r#"
.ui-separator {
  flex-shrink: 0;
  background: var(--ui-border);
  opacity: var(--ui-separator-opacity, 1);
  transform: scaleX(var(--ui-separator-scale-x, 1)) scaleY(var(--ui-separator-scale-y, 1));
  will-change: transform, opacity;
}

.ui-separator--horizontal {
  width: 100%;
  height: 1px;
  transform-origin: left center;
}

.ui-separator--vertical {
  width: 1px;
  height: 100%;
  transform-origin: center top;
}
"#;
