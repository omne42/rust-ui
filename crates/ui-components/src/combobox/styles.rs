pub const CSS: &str = r#"
.ui-combobox {
  display: block;
}

.ui-combobox[data-state="disabled"] {
  opacity: 0.72;
}

.ui-combobox[data-selection="out-of-range"] {
  --ui-combobox-selection-warning: 1;
}

.ui-combobox[data-options="has-disabled"] {
  --ui-combobox-has-disabled-options: 1;
}

.ui-combobox[data-requirement="required"] {
  --ui-combobox-required: 1;
}

.ui-combobox[data-label-source="custom"],
.ui-combobox[data-custom-label="true"],
.ui-combobox--custom-label {
  --ui-combobox-label-source: custom;
}

.ui-combobox[data-description-source="custom"],
.ui-combobox[data-custom-description="true"],
.ui-combobox--custom-description {
  --ui-combobox-description-source: custom;
}

.ui-combobox[data-error-source="custom"],
.ui-combobox[data-custom-error="true"],
.ui-combobox--custom-error {
  --ui-combobox-error-source: custom;
}

.ui-combobox[data-placeholder-source="custom"],
.ui-combobox[data-custom-placeholder="true"],
.ui-combobox--custom-placeholder {
  --ui-combobox-placeholder-source: custom;
}

.ui-combobox[data-motion-source="custom"],
.ui-combobox[data-custom-motion="true"],
.ui-combobox--custom-motion {
  --ui-combobox-custom-motion: 1;
}

.ui-combobox[data-class-source="custom"],
.ui-combobox[data-custom-class="true"],
.ui-combobox--custom-class {
  border-radius: inherit;
}
"#;
