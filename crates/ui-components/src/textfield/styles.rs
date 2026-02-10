pub const CSS: &str = r#"
.ui-textfield {
  display: block;
}

.ui-textfield[data-state="disabled"] {
  opacity: 0.72;
}

.ui-textfield[data-state="invalid"] {
  --ui-textfield-invalid: 1;
}

.ui-textfield[data-state="readonly"] {
  --ui-textfield-readonly: 1;
}

.ui-textfield[data-value="filled"] {
  --ui-textfield-has-value: 1;
}

.ui-textfield[data-requirement="required"] {
  --ui-textfield-required: 1;
}

.ui-textfield[data-label-source="custom"] {
  --ui-textfield-label-source: custom;
}

.ui-textfield[data-description-source="custom"] {
  --ui-textfield-description-source: custom;
}

.ui-textfield[data-error-source="custom"] {
  --ui-textfield-error-source: custom;
}

.ui-textfield[data-placeholder-source="custom"] {
  --ui-textfield-placeholder-source: custom;
}

.ui-textfield[data-type-source="custom"] {
  --ui-textfield-type-source: custom;
}

.ui-textfield--custom-class,
.ui-textfield[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
