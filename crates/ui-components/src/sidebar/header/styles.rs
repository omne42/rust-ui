pub const CSS: &str = r#"
.ui-sidebar-header {
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-header > * {
  min-width: 0;
}

.ui-sidebar-header--disabled,
.ui-sidebar-header[data-disabled="true"] {
  opacity: 0.62;
  pointer-events: none;
}

.ui-sidebar-header--custom-class,
.ui-sidebar-header[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
