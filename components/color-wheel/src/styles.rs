pub const CSS: &str = r#"
.ui-color-wheel {
  --ui-slider-visual-percent: 0;
  --ui-color-wheel-thumb-border: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-border, var(--ui-fallback-border)) 22%
  );
  --ui-color-wheel-motion-duration: var(
    --ui-checkbox-group-motion-duration,
    var(--ui-fallback-checkbox-group-motion-duration)
  );
  --ui-color-wheel-motion-easing: var(
    --ui-checkbox-group-motion-easing,
    var(--ui-fallback-checkbox-group-motion-easing)
  );

  display: inline-grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-color-wheel__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-color-wheel__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 600;
}

.ui-color-wheel__value {
  font-variant-numeric: tabular-nums;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  line-height: var(
    --ui-button-size-s-line-height,
    var(--ui-fallback-button-size-s-line-height)
  );
}

.ui-color-wheel__track {
  position: relative;
  inline-size: var(--ui-color-wheel-size, var(--ui-fallback-color-wheel-size));
  block-size: var(--ui-color-wheel-size, var(--ui-fallback-color-wheel-size));
  border-radius: 50%;
  touch-action: none;
}

.ui-color-wheel__ring {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: conic-gradient(
    var(--ui-color-wheel-hue-red, var(--ui-fallback-color-wheel-hue-red)) 0%,
    var(--ui-color-wheel-hue-yellow, var(--ui-fallback-color-wheel-hue-yellow)) 16.66%,
    var(--ui-color-wheel-hue-green, var(--ui-fallback-color-wheel-hue-green)) 33.33%,
    var(--ui-color-wheel-hue-cyan, var(--ui-fallback-color-wheel-hue-cyan)) 50%,
    var(--ui-color-wheel-hue-blue, var(--ui-fallback-color-wheel-hue-blue)) 66.66%,
    var(--ui-color-wheel-hue-magenta, var(--ui-fallback-color-wheel-hue-magenta)) 83.33%,
    var(--ui-color-wheel-hue-red, var(--ui-fallback-color-wheel-hue-red)) 100%
  );
  -webkit-mask: radial-gradient(
    farthest-side,
    transparent calc(100% - var(--ui-color-wheel-track-thickness, var(--ui-fallback-color-wheel-track-thickness))),
    var(--ui-common-black, var(--ui-fallback-common-black)) calc(
      100% - var(--ui-color-wheel-track-thickness, var(--ui-fallback-color-wheel-track-thickness)) +
        var(--ui-border-width, var(--ui-fallback-border-width))
    )
  );
  mask: radial-gradient(
    farthest-side,
    transparent calc(100% - var(--ui-color-wheel-track-thickness, var(--ui-fallback-color-wheel-track-thickness))),
    var(--ui-common-black, var(--ui-fallback-common-black)) calc(
      100% - var(--ui-color-wheel-track-thickness, var(--ui-fallback-color-wheel-track-thickness)) +
        var(--ui-border-width, var(--ui-fallback-border-width))
    )
  );
}

.ui-color-wheel__orbit {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  pointer-events: none;
  transform: rotate(calc(var(--ui-slider-visual-percent) * 3.6deg));
}

.ui-color-wheel__thumb {
  position: absolute;
  inset-inline-start: 50%;
  inset-block-start: calc(var(--ui-color-wheel-track-thickness, var(--ui-fallback-color-wheel-track-thickness)) / 2);
  inline-size: var(--ui-color-wheel-thumb-size, var(--ui-fallback-color-wheel-thumb-size));
  block-size: var(--ui-color-wheel-thumb-size, var(--ui-fallback-color-wheel-thumb-size));
  border-radius: var(--ui-button-radius-full, var(--ui-fallback-button-radius-full));
  border: var(--ui-slider-thumb-border-width, var(--ui-fallback-slider-thumb-border-width)) solid
    var(--ui-color-wheel-thumb-border);
  background: var(--ui-bg, var(--ui-fallback-bg));
  transform: translate(-50%, -50%);
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  transition:
    transform var(--ui-color-wheel-motion-duration) var(--ui-color-wheel-motion-easing),
    box-shadow var(--ui-color-wheel-motion-duration) var(--ui-color-wheel-motion-easing);
}

.ui-color-wheel__input {
  position: absolute;
  inset: 0;
  margin: 0;
  opacity: 0;
  cursor: grab;
}

.ui-color-wheel__track:hover .ui-color-wheel__thumb {
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}

.ui-color-wheel__track[data-dragging="true"] .ui-color-wheel__input {
  cursor: grabbing;
}

.ui-color-wheel__track[data-dragging="true"] .ui-color-wheel__thumb {
  transform: translate(-50%, -50%) scale(1.04);
}

.ui-color-wheel__track:focus-within .ui-color-wheel__ring {
  outline: var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width)) solid
    color-mix(
      in oklch,
      var(--ui-accent, var(--ui-fallback-accent)),
      var(--ui-bg, var(--ui-fallback-bg)) 64%
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-color-wheel--disabled,
.ui-color-wheel[data-disabled="true"] {
  opacity: var(--ui-checkbox-group-disabled-opacity, var(--ui-fallback-checkbox-group-disabled-opacity));
}

.ui-color-wheel--disabled .ui-color-wheel__input,
.ui-color-wheel[data-disabled="true"] .ui-color-wheel__input {
  cursor: not-allowed;
}

.ui-color-wheel--motion-custom,
.ui-color-wheel[data-motion-source="custom"] {
  --ui-color-wheel-thumb-border: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-fg, var(--ui-fallback-fg)) 16%
  );
}

.ui-color-wheel--label-custom,
.ui-color-wheel[data-label-source="custom"] {
  color: color-mix(
    in oklch,
    var(--ui-fg, var(--ui-fallback-fg)),
    var(--ui-accent, var(--ui-fallback-accent)) 10%
  );
}

.ui-color-wheel--custom-class,
.ui-color-wheel[data-custom-class="true"] {
  isolation: isolate;
}

@media (prefers-reduced-motion: reduce) {
  .ui-color-wheel {
    --ui-color-wheel-motion-duration: 1ms;
  }
}
"#;
