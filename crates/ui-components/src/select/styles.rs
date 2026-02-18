pub const CSS: &str = r#"
.ui-select {
  display: inline-block;
}

.ui-select[data-class-source="custom"],
.ui-select[data-custom-class="true"],
.ui-select--custom-class {
  --ui-select-class-source: custom;
}

.ui-select[data-motion-source="custom"],
.ui-select[data-custom-motion="true"],
.ui-select--custom-motion {
  --ui-select-motion-source: custom;
}

.ui-select--open .ui-select__panel,
.ui-select[data-open="true"] .ui-select__panel {
  --ui-select-open: 1;
}

.ui-select--has-disabled-options .ui-select__listbox,
.ui-select[data-has-disabled-options="true"] .ui-select__listbox {
  --ui-select-has-disabled-options: 1;
}

.ui-select--empty,
.ui-select[data-empty="true"] {
  --ui-select-empty: 1;
}

.ui-select__panel {
  min-width: var(--ui-overlay-panel-min-width);
}

.ui-select__listbox {
  width: 100%;
  box-shadow: none;
  border: none;
  padding: 0;
}
"#;
