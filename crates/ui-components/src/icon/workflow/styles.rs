pub const CSS: &str = r#"
.ui-icons-workflow {
  display: inline-flex;
}

.ui-icons-workflow[data-state="disabled"] {
  opacity: 0.64;
}

.ui-icons-workflow[data-state="decorative"] {
  --ui-icons-workflow-decorative: 1;
}

.ui-icons-workflow[data-icon-reference-source="default"] {
  --ui-icons-workflow-icon-ref-source: default;
}

.ui-icons-workflow[data-icon-reference-source="explicit"] {
  --ui-icons-workflow-icon-ref-source: explicit;
}

.ui-icons-workflow[data-icon-reference-source="prefixed"] {
  --ui-icons-workflow-icon-ref-source: prefixed;
}

.ui-icons-workflow[data-aria-source="custom"] {
  --ui-icons-workflow-aria-source: custom;
}

.ui-icons-workflow[data-class-source="custom"] {
  --ui-icons-workflow-class-source: custom;
}

.ui-icons-workflow[data-glyph-source="custom"] {
  --ui-icons-workflow-glyph-source: custom;
}

.ui-icons-workflow[data-size-source="custom"] {
  --ui-icons-workflow-size-source: custom;
}

.ui-icons-workflow[data-tone-source="custom"] {
  --ui-icons-workflow-tone-source: custom;
}

.ui-icons-workflow--custom-class,
.ui-icons-workflow[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
