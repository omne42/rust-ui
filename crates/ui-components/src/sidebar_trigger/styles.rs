pub const CSS: &str = r#"
.ui-sidebar-trigger {
  transition: transform 160ms ease, opacity 160ms ease;
}

.ui-sidebar-trigger:hover {
  transform: translateY(-1px);
}

.ui-sidebar-trigger--open,
.ui-sidebar-trigger[data-open="true"] {
  opacity: 1;
}

.ui-sidebar-trigger--closed,
.ui-sidebar-trigger[data-closed="true"] {
  opacity: 0.92;
}

.ui-sidebar-trigger--disabled,
.ui-sidebar-trigger[data-disabled="true"] {
  opacity: 0.62;
  transform: none;
}

.ui-sidebar-trigger--custom-class,
.ui-sidebar-trigger[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
