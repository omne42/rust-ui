pub const CSS: &str = r#"
.ui-sidebar-rail {
  --ui-sidebar-rail-accent-solid: var(
    --ui-accent-solid,
    var(--ui-accent, var(--ui-fallback-accent))
  );
  --ui-sidebar-rail-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 52%,
    transparent
  );
  --ui-sidebar-rail-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-rail-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-rail-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  align-self: stretch;
  width: 0.625rem;
  min-height: 2.5rem;
  border: 0;
  border-radius: 999px;
  background: var(--ui-sidebar-rail-bg);
  cursor: pointer;
  transition:
    background var(--ui-sidebar-rail-motion-duration) var(--ui-sidebar-rail-motion-easing),
    transform var(--ui-sidebar-rail-motion-duration) var(--ui-sidebar-rail-motion-easing);
}

.ui-sidebar-rail:hover {
  background: color-mix(in oklab, var(--ui-sidebar-rail-accent-solid) 30%, transparent);
}

.ui-sidebar-rail:focus-visible {
  outline: 2px solid var(--ui-sidebar-rail-accent-solid);
  outline-offset: 1px;
}

.ui-sidebar-rail--right,
.ui-sidebar-rail[data-side="right"] {
  justify-self: end;
}

.ui-sidebar-rail--closed,
.ui-sidebar-rail[data-closed="true"] {
  transform: scaleY(0.92);
}

.ui-sidebar-rail--disabled,
.ui-sidebar-rail[data-disabled="true"] {
  opacity: var(--ui-sidebar-rail-disabled-opacity);
  transform: none;
  cursor: default;
}

.ui-sidebar-rail--custom-class,
.ui-sidebar-rail[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
