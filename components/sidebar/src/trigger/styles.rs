pub const CSS: &str = r#"
.ui-sidebar-trigger {
  --ui-sidebar-trigger-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-trigger-motion-reduced-duration: 1ms;
  --ui-sidebar-trigger-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-trigger-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  transition:
    transform var(--ui-sidebar-trigger-motion-duration) var(--ui-sidebar-trigger-motion-easing),
    opacity var(--ui-sidebar-trigger-motion-duration) var(--ui-sidebar-trigger-motion-easing);
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
  opacity: var(--ui-sidebar-trigger-disabled-opacity);
  transform: none;
}

.ui-sidebar-trigger--custom-class,
.ui-sidebar-trigger[data-custom-class="true"] {
  border-radius: inherit;
}

@media (prefers-reduced-motion: reduce) {
  .ui-sidebar-trigger {
    --ui-sidebar-trigger-motion-duration: var(--ui-sidebar-trigger-motion-reduced-duration);
  }
}
"#;
