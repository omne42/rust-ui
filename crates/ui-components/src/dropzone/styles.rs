pub const CSS: &str = r#"
.ui-dropzone {
  display: block;
}

.ui-dropzone[data-state="disabled"] {
  opacity: 0.72;
}

.ui-dropzone[data-label-source="custom"],
.ui-dropzone[data-custom-label="true"],
.ui-dropzone--custom-label {
  --ui-dropzone-label-source: custom;
}

.ui-dropzone[data-aria-source="custom"],
.ui-dropzone[data-custom-aria="true"],
.ui-dropzone--custom-aria {
  --ui-dropzone-aria-source: custom;
}

.ui-dropzone[data-drop-handler-source="custom"],
.ui-dropzone[data-custom-drop-handler="true"],
.ui-dropzone--custom-drop-handler {
  --ui-dropzone-drop-handler-source: custom;
}

.ui-dropzone[data-motion-source="custom"],
.ui-dropzone[data-custom-motion="true"],
.ui-dropzone--custom-motion {
  --ui-dropzone-custom-motion: 1;
}

.ui-dropzone[data-class-source="custom"],
.ui-dropzone[data-custom-class="true"],
.ui-dropzone--custom-class {
  border-radius: inherit;
}
"#;
