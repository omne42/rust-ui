pub const CSS: &str = r#"
.ui-iconset {
  display: inline-flex;
}

.ui-iconset[data-state="disabled"] {
  opacity: 0.64;
}

.ui-iconset[data-state="decorative"] {
  --ui-iconset-decorative: 1;
}

.ui-iconset[data-state="fallback"] {
  --ui-iconset-fallback-state: 1;
}

.ui-iconset[data-icon-source="registry"] {
  --ui-iconset-icon-source: registry;
}

.ui-iconset[data-icon-source="fallback"] {
  --ui-iconset-icon-source: fallback;
}

.ui-iconset[data-iconset-source="prop"] {
  --ui-iconset-source: prop;
}

.ui-iconset[data-iconset-source="icon"] {
  --ui-iconset-source: icon;
}

.ui-iconset[data-iconset-source="default"] {
  --ui-iconset-source: default;
}

.ui-iconset[data-label-source="custom"] {
  --ui-iconset-label-source: custom;
}

.ui-iconset[data-label-source="registry"] {
  --ui-iconset-label-source: registry;
}

.ui-iconset[data-label-source="fallback"] {
  --ui-iconset-label-source: fallback;
}

.ui-iconset[data-class-source="custom"] {
  --ui-iconset-class-source: custom;
}

.ui-iconset[data-size-source="custom"] {
  --ui-iconset-size-source: custom;
}

.ui-iconset[data-tone-source="custom"] {
  --ui-iconset-tone-source: custom;
}

.ui-iconset--custom-class,
.ui-iconset[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
