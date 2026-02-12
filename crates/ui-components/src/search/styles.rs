pub const CSS: &str = r#"
.ui-search {
  display: block;
}

.ui-search[data-state="disabled"] {
  opacity: 0.72;
}

.ui-search[data-state="invalid"] {
  --ui-search-invalid: 1;
}

.ui-search[data-state="readonly"] {
  --ui-search-readonly: 1;
}

.ui-search[data-value="filled"] {
  --ui-search-has-value: 1;
}

.ui-search[data-requirement="required"] {
  --ui-search-required: 1;
}

.ui-search[data-label-source="custom"],
.ui-search[data-custom-label="true"],
.ui-search--custom-label {
  --ui-search-label-source: custom;
}

.ui-search[data-description-source="custom"],
.ui-search[data-custom-description="true"],
.ui-search--custom-description {
  --ui-search-description-source: custom;
}

.ui-search[data-error-source="custom"],
.ui-search[data-custom-error="true"],
.ui-search--custom-error {
  --ui-search-error-source: custom;
}

.ui-search[data-placeholder-source="custom"],
.ui-search[data-custom-placeholder="true"],
.ui-search--custom-placeholder {
  --ui-search-placeholder-source: custom;
}

.ui-search[data-submit-handler-source="custom"],
.ui-search[data-custom-submit-handler="true"],
.ui-search--custom-submit-handler {
  --ui-search-submit-handler-source: custom;
}

.ui-search[data-clear-handler-source="custom"],
.ui-search[data-custom-clear-handler="true"],
.ui-search--custom-clear-handler {
  --ui-search-clear-handler-source: custom;
}

.ui-search[data-motion-source="custom"],
.ui-search[data-custom-motion="true"],
.ui-search--custom-motion {
  --ui-search-custom-motion: 1;
}

.ui-search[data-class-source="custom"],
.ui-search[data-custom-class="true"],
.ui-search--custom-class {
  border-radius: inherit;
}
"#;
