pub const CSS: &str = r#"
.ui-sidebar-inset {
  --ui-sidebar-inset-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-inset-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-inset-radius-md: var(--ui-radius-md, var(--ui-fallback-radius-md));
  --ui-sidebar-inset-bg-canvas: var(--ui-bg-canvas, var(--ui-bg, var(--ui-fallback-bg)));
  --ui-sidebar-inset-border-subtle: var(
    --ui-border-subtle,
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 18%, transparent)
  );
  --ui-sidebar-inset-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  display: grid;
  gap: 0.5rem;
  min-width: 0;
  border-radius: var(--ui-sidebar-inset-radius-md);
  transition:
    background var(--ui-sidebar-inset-motion-duration) var(--ui-sidebar-inset-motion-easing),
    border-color var(--ui-sidebar-inset-motion-duration) var(--ui-sidebar-inset-motion-easing);
}

.ui-sidebar-inset--padded,
.ui-sidebar-inset[data-padded="true"] {
  padding: 0.75rem;
}

.ui-sidebar-inset--recessed,
.ui-sidebar-inset[data-recessed="true"] {
  background: color-mix(in oklab, var(--ui-sidebar-inset-bg-canvas) 90%, transparent);
  border: 1px solid var(--ui-sidebar-inset-border-subtle);
}

.ui-sidebar-inset--left,
.ui-sidebar-inset[data-side="left"] {
  border-inline-start-width: 2px;
}

.ui-sidebar-inset--right,
.ui-sidebar-inset[data-side="right"] {
  border-inline-end-width: 2px;
}

.ui-sidebar-inset--disabled,
.ui-sidebar-inset[data-disabled="true"] {
  opacity: var(--ui-sidebar-inset-disabled-opacity);
}

.ui-sidebar-inset--custom-class,
.ui-sidebar-inset[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
