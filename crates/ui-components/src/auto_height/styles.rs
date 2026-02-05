pub const CSS: &str = r#"
.ui-auto-height {
  overflow: hidden;
  height: var(--ui-auto-height-height, auto);
  will-change: height;
}

.ui-auto-height__content {
  width: 100%;
}
"#;
