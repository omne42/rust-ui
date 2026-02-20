pub const CSS: &str = r#"
.ui-dropdown {
  display: inline-flex;
}

.ui-dropdown--disabled,
.ui-dropdown[data-disabled="true"] {
  opacity: 0.72;
}

.ui-dropdown--persistent,
.ui-dropdown[data-keep-open-on-action="true"] {
  outline: 1px dashed color-mix(in oklab, var(--ui-border) 64%, var(--ui-accent) 36%);
  outline-offset: 2px;
}

.ui-dropdown--custom-class,
.ui-dropdown[data-custom-class="true"] {
  border-radius: var(--ui-radius-sm);
}

.ui-dropdown__trigger {
  min-width: 0;
}

.ui-dropdown[data-motion-source="custom"],
.ui-dropdown[data-custom-motion="true"] {
  --ui-dropdown-custom-motion: 1;
}
"#;
