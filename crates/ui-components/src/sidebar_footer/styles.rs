pub const CSS: &str = r#"
.ui-sidebar-footer {
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-footer > * {
  min-width: 0;
}

.ui-sidebar-footer--bordered,
.ui-sidebar-footer[data-bordered="true"] {
  border-block-start: 1px solid
    var(--ui-border-subtle, color-mix(in oklab, currentColor 20%, transparent));
  padding-block-start: 0.4rem;
}

.ui-sidebar-footer--disabled,
.ui-sidebar-footer[data-disabled="true"] {
  opacity: 0.62;
  pointer-events: none;
}

.ui-sidebar-footer--custom-class,
.ui-sidebar-footer[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
