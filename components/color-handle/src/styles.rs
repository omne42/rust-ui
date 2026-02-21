pub const CSS: &str = r#"
.ui-color-handle {
  --ui-color-handle-space: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-color-handle-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-handle-border: var(--ui-border, var(--ui-fallback-border));
  --ui-color-handle-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-handle-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-handle-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-handle-disabled-opacity: var(
    --ui-checkbox-disabled-opacity,
    var(--ui-fallback-checkbox-disabled-opacity, 0.62)
  );
  --ui-color-handle-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-color-handle-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  display: grid;
  gap: var(--ui-color-handle-space);
}

.ui-color-handle__surface {
  position: relative;
  min-inline-size: 12rem;
  min-block-size: 7rem;
  border-radius: var(--ui-color-handle-radius);
  border: 1px solid color-mix(in oklch, var(--ui-color-handle-border), transparent 18%);
  background:
    linear-gradient(135deg, color-mix(in oklch, var(--ui-color-handle-accent), transparent 82%), transparent),
    color-mix(in oklch, var(--ui-color-handle-bg), var(--ui-color-handle-fg) 2%);
  overflow: hidden;
  transition:
    border-color var(--ui-color-handle-motion-duration) var(--ui-color-handle-motion-easing),
    box-shadow var(--ui-color-handle-motion-duration) var(--ui-color-handle-motion-easing);
}

.ui-color-handle__thumb.ui-color-thumb {
  position: absolute;
}

.ui-color-handle--focused .ui-color-handle__surface,
.ui-color-handle[data-focused="true"] .ui-color-handle__surface {
  border-color: color-mix(in oklch, var(--ui-color-handle-accent), var(--ui-color-handle-border) 42%);
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-color-handle-accent), transparent 86%);
}

.ui-color-handle--dragging .ui-color-handle__surface,
.ui-color-handle[data-dragging="true"] .ui-color-handle__surface {
  border-color: color-mix(in oklch, var(--ui-color-handle-accent), var(--ui-color-handle-border) 24%);
}

.ui-color-handle--disabled,
.ui-color-handle[data-disabled="true"] {
  opacity: var(--ui-color-handle-disabled-opacity);
}

.ui-color-handle--disabled .ui-color-handle__surface,
.ui-color-handle[data-disabled="true"] .ui-color-handle__surface {
  cursor: not-allowed;
}

.ui-color-handle--custom-class,
.ui-color-handle[data-custom-class="true"] {
  isolation: isolate;
}

@media (prefers-reduced-motion: reduce) {
  .ui-color-handle {
    --ui-color-handle-motion-duration: 0ms;
  }
}
"#;
