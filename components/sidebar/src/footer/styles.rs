pub const CSS: &str = r#"
.ui-sidebar-footer {
  --ui-sidebar-footer-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  --ui-sidebar-footer-border-subtle: var(
    --ui-border-subtle,
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 20%, transparent)
  );
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-footer > * {
  min-width: 0;
}

.ui-sidebar-footer--bordered,
.ui-sidebar-footer[data-bordered="true"] {
  border-block-start: 1px solid var(--ui-sidebar-footer-border-subtle);
  padding-block-start: 0.4rem;
}

.ui-sidebar-footer--disabled,
.ui-sidebar-footer[data-disabled="true"] {
  opacity: var(--ui-sidebar-footer-disabled-opacity);
  pointer-events: none;
}

.ui-sidebar-footer--custom-class,
.ui-sidebar-footer[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
