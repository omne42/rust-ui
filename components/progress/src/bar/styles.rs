pub const CSS: &str = r#"
.ui-progress-bar {
  --ui-progress-bar-track: color-mix(
    in oklch,
    var(--ui-border, var(--ui-fallback-border)) 55%,
    var(--ui-bg, var(--ui-fallback-bg))
  );
  --ui-progress-bar-fill: var(--ui-meter-indicator-color, var(--ui-fallback-meter-indicator-color));

  display: block;
  width: 100%;
  height: var(--ui-meter-track-height, var(--ui-fallback-meter-track-height));
  box-sizing: border-box;
  border: none;
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
  background: var(--ui-progress-bar-track);
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  overflow: hidden;

  appearance: none;
  -webkit-appearance: none;
}

.ui-progress-bar--variant-default,
.ui-progress-bar[data-variant="default"] {
  --ui-progress-bar-fill: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-progress-bar--variant-accent,
.ui-progress-bar[data-variant="accent"] {
  --ui-progress-bar-fill: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-progress-bar--variant-danger,
.ui-progress-bar[data-variant="danger"] {
  --ui-progress-bar-fill: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-progress-bar--size-sm,
.ui-progress-bar[data-size="sm"] {
  height: var(--ui-meter-track-height-sm, var(--ui-fallback-meter-track-height-sm));
}

.ui-progress-bar--size-md,
.ui-progress-bar[data-size="md"] {
  height: var(--ui-meter-track-height, var(--ui-fallback-meter-track-height));
}

.ui-progress-bar--size-lg,
.ui-progress-bar[data-size="lg"] {
  height: var(--ui-meter-track-height-lg, var(--ui-fallback-meter-track-height-lg));
}

.ui-progress-bar--label-custom,
.ui-progress-bar[data-label-source="custom"] {
  --ui-progress-bar-track: color-mix(
    in oklch,
    var(--ui-border, var(--ui-fallback-border)) 45%,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft))
  );
}

.ui-progress-bar--custom-class,
.ui-progress-bar[data-custom-class="true"] {
  isolation: isolate;
}

.ui-progress-bar::-webkit-progress-bar {
  background: var(--ui-progress-bar-track);
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
}

.ui-progress-bar::-webkit-progress-value {
  background: var(--ui-progress-bar-fill);
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
}

.ui-progress-bar::-moz-progress-bar {
  background: var(--ui-progress-bar-fill);
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
}

.ui-progress-bar--state-indeterminate::-webkit-progress-value,
.ui-progress-bar[data-state="indeterminate"]::-webkit-progress-value,
.ui-progress-bar--state-indeterminate::-moz-progress-bar,
.ui-progress-bar[data-state="indeterminate"]::-moz-progress-bar,
.ui-progress-bar--indeterminate::-webkit-progress-value,
.ui-progress-bar:indeterminate::-webkit-progress-value,
.ui-progress-bar--indeterminate::-moz-progress-bar,
.ui-progress-bar:indeterminate::-moz-progress-bar {
  background-image: linear-gradient(
    90deg,
    var(--ui-progress-bar-fill) 0%,
    color-mix(in oklch, var(--ui-progress-bar-fill) 65%, var(--ui-bg, var(--ui-fallback-bg))) 50%,
    var(--ui-progress-bar-fill) 100%
  );
  background-size: 200% 100%;
  animation: ui-progress-bar-indeterminate
    var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))
    var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing)) infinite;
}

.ui-progress-bar--state-determinate::-webkit-progress-value,
.ui-progress-bar[data-state="determinate"]::-webkit-progress-value,
.ui-progress-bar--state-determinate::-moz-progress-bar,
.ui-progress-bar[data-state="determinate"]::-moz-progress-bar {
  background-image: none;
  animation: none;
}

@keyframes ui-progress-bar-indeterminate {
  from {
    background-position-x: 0%;
  }
  to {
    background-position-x: -200%;
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-progress-bar--state-indeterminate::-webkit-progress-value,
  .ui-progress-bar[data-state="indeterminate"]::-webkit-progress-value,
  .ui-progress-bar--state-indeterminate::-moz-progress-bar,
  .ui-progress-bar[data-state="indeterminate"]::-moz-progress-bar,
  .ui-progress-bar--indeterminate::-webkit-progress-value,
  .ui-progress-bar:indeterminate::-webkit-progress-value,
  .ui-progress-bar--indeterminate::-moz-progress-bar,
  .ui-progress-bar:indeterminate::-moz-progress-bar {
    animation: none;
  }
}
"#;
