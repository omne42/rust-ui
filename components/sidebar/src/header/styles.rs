pub const CSS: &str = r#"
.ui-sidebar-header {
  --ui-sidebar-header-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-header > * {
  min-width: 0;
}

.ui-sidebar-header--disabled,
.ui-sidebar-header[data-disabled="true"] {
  opacity: var(--ui-sidebar-header-disabled-opacity);
  pointer-events: none;
}

.ui-sidebar-header--custom-class,
.ui-sidebar-header[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
