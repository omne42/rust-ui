pub const CSS: &str = r#"
.ui-circular-progress {
  display: inline-block;
  width: var(--ui-cp-size, var(--ui-button-spinner-size, 16px));
  height: var(--ui-cp-size, var(--ui-button-spinner-size, 16px));
  box-sizing: border-box;
  border-radius: 9999px;
  border: var(--ui-cp-thickness, var(--ui-button-spinner-border, 2px)) solid var(--ui-border);
  border-top-color: var(--ui-accent);
  animation-duration: var(
    --ui-cp-rotation-duration,
    var(--ui-button-spinner-duration, 800ms)
  );
}

.ui-circular-progress--state-indeterminate,
.ui-circular-progress[data-state="indeterminate"],
.ui-circular-progress[data-motion="spin"] {
  animation: ui-circular-progress-spin 0.9s linear infinite;
}

.ui-circular-progress--size-custom,
.ui-circular-progress[data-size-source="custom"] {
  min-width: var(--ui-cp-size, var(--ui-button-spinner-size, 16px));
}

.ui-circular-progress--thickness-custom,
.ui-circular-progress[data-thickness-source="custom"] {
  border-width: var(--ui-cp-thickness, var(--ui-button-spinner-border, 2px));
}

.ui-circular-progress--label-custom,
.ui-circular-progress[data-label-source="custom"] {
  border-top-color: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-circular-progress--custom-class,
.ui-circular-progress[data-custom-class="true"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklch, var(--ui-border), transparent 45%);
}

@keyframes ui-circular-progress-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-circular-progress--state-indeterminate,
  .ui-circular-progress[data-state="indeterminate"],
  .ui-circular-progress[data-motion="spin"] {
    animation-duration: 1ms;
    animation-iteration-count: 1;
  }
}
"#;
