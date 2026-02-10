pub const CSS: &str = r#"
.ui-dropzone {
  display: block;
}

.ui-dropzone[data-state="disabled"] {
  opacity: 0.72;
}

.ui-dropzone[data-label-source="custom"] {
  --ui-dropzone-label-source: custom;
}

.ui-dropzone[data-aria-source="custom"] {
  --ui-dropzone-aria-source: custom;
}

.ui-dropzone[data-drop-handler-source="custom"] {
  --ui-dropzone-drop-handler-source: custom;
}

.ui-dropzone[data-motion-source="custom"],
.ui-dropzone[data-custom-motion="true"] {
  --ui-dropzone-custom-motion: 1;
}

.ui-dropzone--custom-class,
.ui-dropzone[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
