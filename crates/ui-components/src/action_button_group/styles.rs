pub const CSS: &str = r#"
.ui-action-button-group {
  display: inline-flex;
  gap: var(--ui-space-xs);
}

.ui-action-button-group--horizontal {
  flex-direction: row;
  align-items: center;
}

.ui-action-button-group--vertical {
  flex-direction: column;
  align-items: flex-start;
}

.ui-action-button-group--density-compact {
  gap: calc(var(--ui-space-xs) / 2);
}

.ui-action-button-group--justified > .ui-action-button {
  flex: 1;
}
"#;
