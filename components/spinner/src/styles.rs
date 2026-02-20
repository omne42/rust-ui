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
    var(--ui-button-spinner-duration, 800ms)
  );
}

.ui-spinner--size-sm,
.ui-spinner[data-size="sm"] {
  --ui-cp-size: var(--ui-button-spinner-size, 16px);
  --ui-cp-thickness: var(--ui-button-spinner-border, 2px);
}

.ui-spinner--size-md,
.ui-spinner[data-size="md"] {
  --ui-cp-size: calc(var(--ui-button-spinner-size, 16px) + var(--ui-space-2xs, 4px));
  --ui-cp-thickness: var(--ui-button-spinner-border, 2px);
}

.ui-spinner--size-lg,
.ui-spinner[data-size="lg"] {
  --ui-cp-size: calc(
    var(--ui-button-spinner-size, 16px) + var(--ui-space-sm, 8px) + var(--ui-space-2xs, 4px)
  );
  --ui-cp-thickness: calc(
    var(--ui-button-spinner-border, 2px) + (var(--ui-space-3xs, 2px) / 2)
  );
}

.ui-spinner--label-custom .ui-spinner__progress,
.ui-spinner[data-label-source="custom"] .ui-spinner__progress {
  border-top-color: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-spinner--custom-class,
.ui-spinner[data-custom-class="true"] {
  isolation: isolate;
}

.ui-spinner[data-class-source="custom"] .ui-spinner__progress {
  box-shadow: inset 0 0 0 1px color-mix(in oklch, var(--ui-border), transparent 45%);
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
    animation-duration: 1ms;
    animation-iteration-count: 1;
  }
}
"#;
