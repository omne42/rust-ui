pub const CSS: &str = r#"
.ui-progress-circle {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  --ui-meter-indicator-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-progress-circle--label-custom,
.ui-progress-circle[data-label-source="custom"] {
  --ui-meter-indicator-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-progress-circle__svg {
  display: block;
  transform: rotate(-90deg);
  transform-origin: 50% 50%;
}

.ui-progress-circle__track {
  color: var(--ui-border, var(--ui-fallback-border));
}

.ui-progress-circle__indicator {
  color: var(--ui-meter-indicator-color, var(--ui-fallback-meter-indicator-color));
}

.ui-progress-circle--size-custom,
.ui-progress-circle[data-size-source="custom"] {
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
}

.ui-progress-circle--stroke-custom .ui-progress-circle__track,
.ui-progress-circle[data-stroke-source="custom"] .ui-progress-circle__track {
  opacity: 0.9;
}

.ui-progress-circle--motion-custom,
.ui-progress-circle[data-motion-source="custom"] {
  transition: box-shadow var(--ui-meter-shadow-transition-duration, var(--ui-fallback-meter-shadow-transition-duration))
    var(--ui-meter-shadow-transition-easing, var(--ui-fallback-meter-shadow-transition-easing));
}

.ui-progress-circle--custom-class,
.ui-progress-circle[data-custom-class="true"] {
  isolation: isolate;
}

.ui-progress-circle--state-indeterminate .ui-progress-circle__svg,
.ui-progress-circle[data-state="indeterminate"] .ui-progress-circle__svg {
  animation: ui-progress-circle-spin
    var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))
    var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing)) infinite;
}

.ui-progress-circle--state-determinate .ui-progress-circle__svg,
.ui-progress-circle[data-state="determinate"] .ui-progress-circle__svg {
  animation: none;
}

@media (prefers-reduced-motion: reduce) {
  .ui-progress-circle--state-indeterminate .ui-progress-circle__svg,
  .ui-progress-circle[data-state="indeterminate"] .ui-progress-circle__svg {
    animation: none;
  }
}

@keyframes ui-progress-circle-spin {
  to {
    transform: rotate(270deg);
  }
}
"#;
