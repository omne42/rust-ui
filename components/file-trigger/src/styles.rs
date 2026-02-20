pub const CSS: &str = r#"
.ui-file-trigger {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
}

.ui-file-trigger--disabled,
.ui-file-trigger[data-disabled="true"] {
  opacity: 0.7;
}

.ui-file-trigger[data-motion-source="custom"],
.ui-file-trigger--custom-motion,
.ui-file-trigger[data-custom-motion="true"] {
  --ui-file-trigger-custom-motion: 1;
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
