pub const CSS: &str = r#"
.ui-sidebar-trigger {
  --ui-sidebar-trigger-motion-duration: 160ms;
  transition:
    transform var(--ui-sidebar-trigger-motion-duration) ease,
    opacity var(--ui-sidebar-trigger-motion-duration) ease;
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

@media (prefers-reduced-motion: reduce) {
  .ui-sidebar-trigger {
    --ui-sidebar-trigger-motion-duration: 1ms;
  }
}
"#;
