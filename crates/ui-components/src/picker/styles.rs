pub const CSS: &str = r#"
.ui-picker {
  display: block;
}

.ui-picker[data-state="disabled"] {
  opacity: 0.62;
}

.ui-picker[data-state="empty"] {
  --ui-picker-empty: 1;
}

.ui-picker[data-selection="selected"] {
  --ui-picker-has-selection: 1;
}

.ui-picker[data-disabled-options="present"] {
  --ui-picker-disabled-options: 1;
}

.ui-picker[data-open-mode="controlled"] {
  --ui-picker-open-mode: controlled;
}

.ui-picker[data-open-mode="uncontrolled"] {
  --ui-picker-open-mode: uncontrolled;
}

.ui-picker[data-initial-open="open"] {
  --ui-picker-initial-open: open;
}

.ui-picker[data-placeholder-source="custom"] {
  --ui-picker-placeholder-source: custom;
}

.ui-picker[data-handler-source="custom"] {
  --ui-picker-handler-source: custom;
}

.ui-picker[data-class-source="custom"] {
  --ui-picker-class-source: custom;
}

.ui-picker[data-placement-source="custom"] {
  --ui-picker-placement-source: custom;
}

.ui-picker[data-motion-source="custom"] {
  --ui-picker-motion-source: custom;
}

.ui-picker--custom-class,
.ui-picker[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
