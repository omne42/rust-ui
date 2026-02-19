pub const CSS: &str = r#"
.ui-picker-button {
  display: inline-flex;
}

.ui-picker-button[data-state="disabled"] {
  opacity: 0.56;
}

.ui-picker-button[data-state="invalid"] {
  --ui-picker-button-invalid: 1;
}

.ui-picker-button[data-state="active"] {
  --ui-picker-button-active: 1;
}

.ui-picker-button[data-quiet="true"] {
  --ui-picker-button-quiet: 1;
}

.ui-picker-button[data-invalid="true"] {
  --ui-picker-button-invalid-bool: 1;
}

.ui-picker-button[data-disabled="true"] {
  --ui-picker-button-disabled-bool: 1;
}

.ui-picker-button[data-active="true"] {
  --ui-picker-button-active-bool: 1;
}

.ui-picker-button[data-has-handler="true"] {
  --ui-picker-button-handler-source: custom;
}

.ui-picker-button[data-aria-source="custom"] {
  --ui-picker-button-aria-source: custom;
}

.ui-picker-button[data-class-source="custom"] {
  --ui-picker-button-class-source: custom;
}

.ui-picker-button[data-handler-source="custom"] {
  --ui-picker-button-handler-marker: custom;
}

.ui-picker-button--custom-class,
.ui-picker-button[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
