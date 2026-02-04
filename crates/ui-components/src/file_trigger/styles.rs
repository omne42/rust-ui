pub const CSS: &str = r#"
.ui-file-trigger {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
}

.ui-file-trigger__input {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}
"#;
