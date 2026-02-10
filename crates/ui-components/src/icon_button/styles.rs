pub const CSS: &str = r#"
.ui-icon-button {
  display: inline-flex;
}

.ui-icon-button[data-state="disabled"] {
  opacity: 0.62;
}

.ui-icon-button[data-size-mode="icon"] {
  --ui-icon-button-size-mode: icon;
}

.ui-icon-button[data-size-mode="custom"] {
  --ui-icon-button-size-mode: custom;
}

.ui-icon-button[data-handler-source="custom"] {
  --ui-icon-button-handler-source: custom;
}

.ui-icon-button[data-label-source="custom"] {
  --ui-icon-button-label-source: custom;
}

.ui-icon-button[data-class-source="custom"] {
  --ui-icon-button-class-source: custom;
}

.ui-icon-button[data-motion-source="custom"] {
  --ui-icon-button-motion-source: custom;
}

.ui-icon-button[data-custom-class="true"],
.ui-icon-button--custom-class {
  border-radius: inherit;
}
"#;
