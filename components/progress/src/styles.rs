pub const CSS: &str = r#"
.ui-progress {
  display: inline-flex;
  width: 100%;
  max-width: var(--ui-slider-max-width, var(--ui-fallback-slider-max-width));
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-meter-indicator-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-progress--label-custom,
.ui-progress[data-label-source="custom"] {
  --ui-meter-indicator-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-progress__track {
  position: relative;
  height: var(--ui-meter-track-height, var(--ui-fallback-meter-track-height));
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
  background: var(--ui-bg, var(--ui-fallback-bg));
  border: var(--ui-meter-track-border-width, var(--ui-fallback-meter-track-border-width))
    solid var(--ui-border, var(--ui-fallback-border));
  overflow: hidden;
}

.ui-progress--value-label-custom .ui-progress__track,
.ui-progress[data-value-label-source="custom"] .ui-progress__track {
  border-color: color-mix(
    in oklch,
    var(--ui-border, var(--ui-fallback-border)),
    var(--ui-accent, var(--ui-fallback-accent)) 20%
  );
}

.ui-progress__indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 100%;
  transform-origin: left center;
  transform: scaleX(var(--ui-meter-progress, var(--ui-fallback-meter-progress)));
  background: var(--ui-meter-indicator-color, var(--ui-fallback-meter-indicator-color));
  border-radius: inherit;
  will-change: transform;
}

.ui-progress--motion-custom,
.ui-progress[data-motion-source="custom"] {
  transition: box-shadow var(--ui-meter-shadow-transition-duration, var(--ui-fallback-meter-shadow-transition-duration))
    var(--ui-meter-shadow-transition-easing, var(--ui-fallback-meter-shadow-transition-easing));
}

.ui-progress--custom-class,
.ui-progress[data-custom-class="true"] {
  isolation: isolate;
}

.ui-progress--indeterminate .ui-progress__indicator,
.ui-progress--state-indeterminate .ui-progress__indicator,
.ui-progress[data-state="indeterminate"] .ui-progress__indicator {
  width: var(--ui-meter-indeterminate-width, var(--ui-fallback-meter-indeterminate-width));
  transform: translateX(var(--ui-meter-indeterminate-start, var(--ui-fallback-meter-indeterminate-start)));
  animation: ui-progress-indeterminate
    var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))
    var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing)) infinite;
}

.ui-progress--state-determinate .ui-progress__indicator,
.ui-progress[data-state="determinate"] .ui-progress__indicator {
  width: var(--ui-meter-determinate-width, var(--ui-fallback-meter-determinate-width));
  transform: scaleX(var(--ui-meter-progress, var(--ui-fallback-meter-progress)));
  animation: none;
}

@media (prefers-reduced-motion: reduce) {
  .ui-progress--indeterminate .ui-progress__indicator,
  .ui-progress--state-indeterminate .ui-progress__indicator,
  .ui-progress[data-state="indeterminate"] .ui-progress__indicator {
    animation: none;
  }
}

@keyframes ui-progress-indeterminate {
  0% {
    transform: translateX(var(--ui-meter-indeterminate-start, var(--ui-fallback-meter-indeterminate-start)));
  }
  50% {
    transform: translateX(var(--ui-meter-indeterminate-mid, var(--ui-fallback-meter-indeterminate-mid)));
  }
  100% {
    transform: translateX(var(--ui-meter-indeterminate-end, var(--ui-fallback-meter-indeterminate-end)));
  }
}
"#;
