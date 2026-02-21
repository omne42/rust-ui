pub const CSS: &str = r#"
.ui-meter {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-meter-indicator-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-meter--variant-default,
.ui-meter[data-variant="default"] {
  --ui-meter-indicator-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-meter--variant-danger,
.ui-meter[data-variant="danger"] {
  --ui-meter-indicator-color: var(--ui-meter-indicator-color-danger, var(--ui-fallback-meter-indicator-color-danger));
}

.ui-meter__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-meter__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 600;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-meter__value-label {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-meter--label-custom .ui-meter__label,
.ui-meter[data-label-source="custom"] .ui-meter__label {
  color: color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)), var(--ui-accent, var(--ui-fallback-accent)) 20%);
}

.ui-meter--value-label-custom .ui-meter__value-label,
.ui-meter[data-value-label-source="custom"] .ui-meter__value-label {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-meter__track {
  position: relative;
  height: var(--ui-meter-track-height, var(--ui-fallback-meter-track-height));
  border-radius: var(--ui-meter-track-radius, var(--ui-fallback-meter-track-radius));
  background: var(--ui-bg, var(--ui-fallback-bg));
  border: var(--ui-meter-track-border-width, var(--ui-fallback-meter-track-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  overflow: hidden;
}

.ui-meter--size-sm .ui-meter__track,
.ui-meter[data-size="sm"] .ui-meter__track {
  height: var(--ui-meter-track-height-sm, var(--ui-fallback-meter-track-height-sm));
}

.ui-meter--size-lg .ui-meter__track,
.ui-meter[data-size="lg"] .ui-meter__track {
  height: var(--ui-meter-track-height-lg, var(--ui-fallback-meter-track-height-lg));
}

.ui-meter__indicator {
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

.ui-meter--motion-custom,
.ui-meter[data-motion-source="custom"] {
  transition: box-shadow var(--ui-meter-shadow-transition-duration, var(--ui-fallback-meter-shadow-transition-duration))
    var(--ui-meter-shadow-transition-easing, var(--ui-fallback-meter-shadow-transition-easing));
}

.ui-meter--custom-class,
.ui-meter[data-custom-class="true"] {
  isolation: isolate;
}

.ui-meter--indeterminate .ui-meter__indicator,
.ui-meter--state-indeterminate .ui-meter__indicator,
.ui-meter[data-state="indeterminate"] .ui-meter__indicator {
  width: var(--ui-meter-indeterminate-width, var(--ui-fallback-meter-indeterminate-width));
  transform: translateX(var(--ui-meter-indeterminate-start, var(--ui-fallback-meter-indeterminate-start)));
  animation: ui-meter-indeterminate
    var(--ui-meter-indeterminate-duration, var(--ui-fallback-meter-indeterminate-duration))
    var(--ui-meter-indeterminate-easing, var(--ui-fallback-meter-indeterminate-easing)) infinite;
}

.ui-meter--state-determinate .ui-meter__indicator,
.ui-meter[data-state="determinate"] .ui-meter__indicator {
  width: var(--ui-meter-determinate-width, var(--ui-fallback-meter-determinate-width));
  transform: scaleX(var(--ui-meter-progress, var(--ui-fallback-meter-progress)));
  animation: none;
}

@media (prefers-reduced-motion: reduce) {
  .ui-meter--indeterminate .ui-meter__indicator,
  .ui-meter--state-indeterminate .ui-meter__indicator,
  .ui-meter[data-state="indeterminate"] .ui-meter__indicator {
    animation: none;
  }
}

@keyframes ui-meter-indeterminate {
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
