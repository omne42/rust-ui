pub const CSS: &str = r#"
.ui-top-nav {
  display: block;
}

.ui-top-nav[data-selection-mode="controlled"] {
  --ui-top-nav-selection-mode: controlled;
}

.ui-top-nav[data-focus-activation="manual"] {
  --ui-top-nav-focus-activation: manual;
}

.ui-top-nav--has-default-selection,
.ui-top-nav[data-has-default-selection="true"] {
  --ui-top-nav-default-selection: 1;
}

.ui-top-nav--custom-label,
.ui-top-nav[data-label-source="custom"],
.ui-top-nav[data-custom-label="true"] {
  --ui-top-nav-label-source: custom;
}

.ui-top-nav[data-motion-source="custom"],
.ui-top-nav[data-custom-motion="true"] {
  --ui-top-nav-custom-motion: 1;
}

.ui-top-nav[data-class-source="custom"],
.ui-top-nav--custom-class,
.ui-top-nav[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
