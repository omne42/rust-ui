pub const CSS: &str = r#"
.ui-circular-progress {
  display: inline-block;
  width: var(--ui-cp-size, var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size)));
  height: var(--ui-cp-size, var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size)));
  box-sizing: border-box;
  border-radius: var(--ui-button-radius-full, var(--ui-fallback-button-radius-full));
  border:
    var(--ui-cp-thickness, var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border)))
    solid var(--ui-border, var(--ui-fallback-border));
  border-top-color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-circular-progress--state-indeterminate,
.ui-circular-progress[data-state="indeterminate"],
.ui-circular-progress[data-motion="spin"] {
  animation: ui-circular-progress-spin
    var(
      --ui-cp-rotation-duration,
      var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))
    ) linear infinite;
}

.ui-circular-progress--size-custom,
.ui-circular-progress[data-size-source="custom"] {
  min-width: var(--ui-cp-size, var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size)));
}

.ui-circular-progress--thickness-custom,
.ui-circular-progress[data-thickness-source="custom"] {
  border-width:
    var(--ui-cp-thickness, var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border)));
}

.ui-circular-progress--label-custom,
.ui-circular-progress[data-label-source="custom"] {
  border-top-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)),
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-circular-progress--custom-class,
.ui-circular-progress[data-custom-class="true"] {
  box-shadow: inset 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(in oklch, var(--ui-border, var(--ui-fallback-border)), transparent 45%);
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
