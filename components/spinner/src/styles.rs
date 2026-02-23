pub const CSS: &str = r#"
.ui-spinner {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-spinner__progress {
  display: inline-flex;
  animation-duration: var(
    --ui-spinner-rotation-duration,
    var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))
  );
}

.ui-spinner--size-sm,
.ui-spinner[data-size="sm"] {
  --ui-cp-size: var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size));
  --ui-cp-thickness: var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border));
}

.ui-spinner--size-md,
.ui-spinner[data-size="md"] {
  --ui-cp-size: calc(
    var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size))
      + var(--ui-space-2xs, var(--ui-fallback-space-2xs))
  );
  --ui-cp-thickness: var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border));
}

.ui-spinner--size-lg,
.ui-spinner[data-size="lg"] {
  --ui-cp-size: calc(
    var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size))
      + var(--ui-space-sm, var(--ui-fallback-space-sm))
      + var(--ui-space-2xs, var(--ui-fallback-space-2xs))
  );
  --ui-cp-thickness: calc(
    var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border))
      + (var(--ui-space-3xs, var(--ui-fallback-space-3xs)) / 2)
  );
}

.ui-spinner--label-custom .ui-spinner__progress,
.ui-spinner[data-label-source="custom"] .ui-spinner__progress {
  border-top-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-spinner--custom-class,
.ui-spinner[data-custom-class="true"] {
  isolation: isolate;
}

.ui-spinner[data-class-source="custom"] .ui-spinner__progress {
  box-shadow: inset 0 0 0
    var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(in oklch, var(--ui-border, var(--ui-fallback-border)), transparent 45%);
}

.ui-spinner[data-motion-source="custom"],
.ui-spinner[data-custom-motion="true"] {
  --ui-spinner-motion-custom: 1;
}

.ui-spinner[data-state="indeterminate"] .ui-spinner__progress,
.ui-spinner[data-indeterminate="true"] .ui-spinner__progress {
  animation-play-state: running;
}

@media (prefers-reduced-motion: reduce) {
  .ui-spinner__progress {
    animation: none;
  }
}
"#;
