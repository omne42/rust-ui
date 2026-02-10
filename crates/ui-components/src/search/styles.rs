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

.ui-search[data-label-source="custom"] {
  --ui-search-label-source: custom;
}

.ui-search[data-description-source="custom"] {
  --ui-search-description-source: custom;
}

.ui-search[data-error-source="custom"] {
  --ui-search-error-source: custom;
}

.ui-search[data-placeholder-source="custom"] {
  --ui-search-placeholder-source: custom;
}

.ui-search[data-submit-handler-source="custom"] {
  --ui-search-submit-handler-source: custom;
}

.ui-search[data-clear-handler-source="custom"] {
  --ui-search-clear-handler-source: custom;
}

.ui-search[data-motion-source="custom"],
.ui-search[data-custom-motion="true"] {
  --ui-search-custom-motion: 1;
}

.ui-search--custom-class,
.ui-search[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
