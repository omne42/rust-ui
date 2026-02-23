pub const CSS: &str = r#"
.ui-sidebar-content {
  --ui-sidebar-content-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-content > * {
  min-width: 0;
}

.ui-sidebar-content--padded,
.ui-sidebar-content[data-padded="true"] {
  padding: 0.25rem;
}

.ui-sidebar-content--scrollable,
.ui-sidebar-content[data-scrollable="true"] {
  min-height: 0;
  overflow: auto;
}

.ui-sidebar-content--disabled,
.ui-sidebar-content[data-disabled="true"] {
  opacity: var(--ui-sidebar-content-disabled-opacity);
  pointer-events: none;
}

.ui-sidebar-content--custom-class,
.ui-sidebar-content[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
