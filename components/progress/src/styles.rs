pub const CSS: &str = r#"
.ui-progress {
  display: inline-flex;
  width: 220px;
  flex-direction: column;
  gap: var(--ui-space-xs);
  --ui-progress-indicator-color: var(--ui-accent);
}

.ui-progress--label-custom,
.ui-progress[data-label-source="custom"] {
  --ui-progress-indicator-color: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-progress__track {
  position: relative;
  height: 10px;
  border-radius: 999px;
  background: var(--ui-bg);
  border: 1px solid var(--ui-border);
  overflow: hidden;
}

.ui-progress--value-label-custom .ui-progress__track,
.ui-progress[data-value-label-source="custom"] .ui-progress__track {
  border-color: color-mix(in oklch, var(--ui-border), var(--ui-accent) 20%);
}

.ui-progress__indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 100%;
  transform-origin: left center;
  transform: scaleX(var(--ui-progress-progress, 0));
  background: var(--ui-progress-indicator-color);
  border-radius: inherit;
  will-change: transform;
}

.ui-progress--motion-custom,
.ui-progress[data-motion-source="custom"] {
  transition: box-shadow 160ms ease;
}

.ui-progress--custom-class,
.ui-progress[data-custom-class="true"] {
  isolation: isolate;
}

.ui-progress--indeterminate .ui-progress__indicator,
.ui-progress--state-indeterminate .ui-progress__indicator,
.ui-progress[data-state="indeterminate"] .ui-progress__indicator {
  width: 40%;
  transform: translateX(-60%);
  animation: ui-progress-indeterminate 1.2s ease-in-out infinite;
}

.ui-progress--state-determinate .ui-progress__indicator,
.ui-progress[data-state="determinate"] .ui-progress__indicator {
  width: 100%;
  transform: scaleX(var(--ui-progress-progress, 0));
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
    transform: translateX(-60%);
  }
  50% {
    transform: translateX(80%);
  }
  100% {
    transform: translateX(220%);
  }
}
"#;
