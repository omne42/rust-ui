pub const CSS: &str = r#"
.ui-drag-and-drop {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
}

.ui-drag-and-drop[data-state="disabled"] {
  opacity: 0.64;
}

.ui-drag-and-drop[data-supports-drop="true"] {
  --ui-drag-and-drop-drop-enabled: 1;
}

.ui-drag-and-drop[data-supports-pick="true"] {
  --ui-drag-and-drop-pick-enabled: 1;
}
"#;
