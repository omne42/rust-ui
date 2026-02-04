pub const CSS: &str = r#"
.ui-ripple {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: currentColor;
  opacity: 0;
  transform: scale(0);
  pointer-events: none;
  will-change: transform, opacity;
}
"#;
