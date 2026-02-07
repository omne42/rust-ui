pub const CSS: &str = r#"
.ui-progress-circle {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  --ui-progress-circle-indicator-color: var(--ui-accent);
}

.ui-progress-circle--label-custom,
.ui-progress-circle[data-label-source="custom"] {
  --ui-progress-circle-indicator-color: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-progress-circle__svg {
  display: block;
  transform: rotate(-90deg);
  transform-origin: 50% 50%;
}

.ui-progress-circle__track {
  color: var(--ui-border);
}

.ui-progress-circle__indicator {
  color: var(--ui-progress-circle-indicator-color);
}

.ui-progress-circle--size-custom,
.ui-progress-circle[data-size-source="custom"] {
  border-radius: 9999px;
}

.ui-progress-circle--stroke-custom .ui-progress-circle__track,
.ui-progress-circle[data-stroke-source="custom"] .ui-progress-circle__track {
  opacity: 0.9;
}

.ui-progress-circle--motion-custom,
.ui-progress-circle[data-motion-source="custom"] {
  transition: box-shadow 160ms ease;
}

.ui-progress-circle--custom-class,
.ui-progress-circle[data-custom-class="true"] {
  isolation: isolate;
}

.ui-progress-circle--state-indeterminate .ui-progress-circle__svg,
.ui-progress-circle[data-state="indeterminate"] .ui-progress-circle__svg {
  animation: ui-progress-circle-spin 1s linear infinite;
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
