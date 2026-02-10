pub const CSS: &str = r#"
.ui-split-view {
  display: block;
}

.ui-split-view[data-orientation="horizontal"] {
  --ui-split-view-axis: horizontal;
}

.ui-split-view[data-orientation="vertical"] {
  --ui-split-view-axis: vertical;
}

.ui-split-view[data-state="disabled"] {
  opacity: 0.72;
}

.ui-split-view[data-split-mode="controlled"] {
  --ui-split-view-mode: controlled;
}

.ui-split-view[data-split-mode="uncontrolled"] {
  --ui-split-view-mode: uncontrolled;
}

.ui-split-view[data-handle="with-handle"] {
  --ui-split-view-handle: with-handle;
}

.ui-split-view[data-handle="plain"] {
  --ui-split-view-handle: plain;
}

.ui-split-view[data-default-split-source="custom"] {
  --ui-split-view-default-source: custom;
}

.ui-split-view[data-bounds-source="custom"] {
  --ui-split-view-bounds-source: custom;
}

.ui-split-view[data-label-source="custom"] {
  --ui-split-view-label-source: custom;
}

.ui-split-view[data-class-source="custom"] {
  --ui-split-view-class-source: custom;
}

.ui-split-view[data-handler-source="custom"] {
  --ui-split-view-handler-source: custom;
}

.ui-split-view--custom-class,
.ui-split-view[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
