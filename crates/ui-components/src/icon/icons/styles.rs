pub const CSS: &str = r#"
.ui-icons {
  display: inline-flex;
}

.ui-icons[data-state="disabled"] {
  opacity: 0.64;
}

.ui-icons[data-state="decorative"] {
  --ui-icons-decorative: 1;
}

.ui-icons[data-set="ui"] {
  --ui-icons-set: ui;
}

.ui-icons[data-set="workflow"] {
  --ui-icons-set: workflow;
}

.ui-icons[data-scale="medium"] {
  --ui-icons-scale: medium;
}

.ui-icons[data-scale="large"] {
  --ui-icons-scale: large;
}

.ui-icons[data-set-source="name"] {
  --ui-icons-set-source: name;
}

.ui-icons[data-set-source="prop"] {
  --ui-icons-set-source: prop;
}

.ui-icons[data-set-source="default"] {
  --ui-icons-set-source: default;
}

.ui-icons[data-aria-source="custom"] {
  --ui-icons-aria-source: custom;
}

.ui-icons[data-class-source="custom"] {
  --ui-icons-class-source: custom;
}

.ui-icons[data-glyph-source="custom"] {
  --ui-icons-glyph-source: custom;
}

.ui-icons[data-tone-source="custom"] {
  --ui-icons-tone-source: custom;
}

.ui-icons--custom-class,
.ui-icons[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
