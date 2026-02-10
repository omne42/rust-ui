pub const CSS: &str = r#"
.ui-icons-ui {
  display: inline-flex;
}

.ui-icons-ui[data-state="disabled"] {
  opacity: 0.64;
}

.ui-icons-ui[data-state="decorative"] {
  --ui-icons-ui-decorative: 1;
}

.ui-icons-ui[data-icon-reference-source="default"] {
  --ui-icons-ui-icon-ref-source: default;
}

.ui-icons-ui[data-icon-reference-source="explicit"] {
  --ui-icons-ui-icon-ref-source: explicit;
}

.ui-icons-ui[data-icon-reference-source="prefixed"] {
  --ui-icons-ui-icon-ref-source: prefixed;
}

.ui-icons-ui[data-aria-source="custom"] {
  --ui-icons-ui-aria-source: custom;
}

.ui-icons-ui[data-class-source="custom"] {
  --ui-icons-ui-class-source: custom;
}

.ui-icons-ui[data-glyph-source="custom"] {
  --ui-icons-ui-glyph-source: custom;
}

.ui-icons-ui[data-size-source="custom"] {
  --ui-icons-ui-size-source: custom;
}

.ui-icons-ui[data-tone-source="custom"] {
  --ui-icons-ui-tone-source: custom;
}

.ui-icons-ui--custom-class,
.ui-icons-ui[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
