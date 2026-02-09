pub const CSS: &str = r#"
.ui-toggle {
  --ui-toggle-selected: 0;
}

.ui-toggle[data-selected="true"] {
  --ui-toggle-selected: 1;
}

.ui-toggle[data-unselected="true"] {
  --ui-toggle-selected: 0;
}

.ui-toggle[data-selected="true"] .ui-toggle-button__label {
  font-weight: 600;
}
"#;
