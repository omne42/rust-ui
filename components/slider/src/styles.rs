pub const CSS: &str = r#"
.ui-slider {
  --ui-slider-percent: 0;
  --ui-slider-visual-percent: var(--ui-slider-percent);
  --ui-slider-track-height: var(--ui-space-2xs, 4px);
  --ui-slider-thumb-size: var(--ui-icon-size-100, 20px);
  --ui-slider-track-bg: color-mix(in oklch, var(--ui-border), var(--ui-bg) 28%);
  --ui-slider-fill-bg: var(--ui-accent);
  --ui-slider-thumb-bg: var(--ui-bg);
  --ui-slider-thumb-border: color-mix(in oklch, var(--ui-accent), var(--ui-border) 24%);
  display: grid;
  gap: var(--ui-space-xs, 6px);
  width: min(100%, var(--ui-slider-max-width, 352px));
  color: var(--ui-fg);
}

.ui-slider__label {
  font-size: var(--ui-font-size-100, 12px);
  font-weight: 600;
}

.ui-slider__control {
  position: relative;
  display: block;
  min-height: var(--ui-slider-thumb-size);
}

.ui-slider__input {
  position: absolute;
  inset: 0;
  margin: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
  z-index: 2;
}

.ui-slider__track {
  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  height: var(--ui-slider-track-height);
  border-radius: 999px;
  background: var(--ui-slider-track-bg);
  overflow: visible;
}

.ui-slider__fill {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: calc(var(--ui-slider-visual-percent) * 1%);
  border-radius: inherit;
  background: var(--ui-slider-fill-bg);
}

.ui-slider__thumb {
  position: absolute;
  top: 50%;
  left: calc(var(--ui-slider-visual-percent) * 1%);
  width: var(--ui-slider-thumb-size);
  height: var(--ui-slider-thumb-size);
  border-radius: 999px;
  border: var(--ui-slider-thumb-border-width, 2px) solid var(--ui-slider-thumb-border);
  background: var(--ui-slider-thumb-bg);
  transform: translate(-50%, -50%);
  box-shadow: var(--ui-shadow-sm);
  pointer-events: none;
}

.ui-slider[data-pressed="true"] .ui-slider__thumb {
  transform: translate(-50%, -50%) scale(0.94);
}

.ui-slider[data-focus-visible="true"] .ui-slider__track {
  box-shadow: 0 0 0 var(--ui-slider-focus-ring-width, 2px) color-mix(in oklch, var(--ui-focus-ring), transparent 68%);
}

.ui-slider--state-disabled,
.ui-slider[data-state="disabled"],
.ui-slider[data-disabled="true"] {
  opacity: 0.62;
}

.ui-slider--state-disabled .ui-slider__input,
.ui-slider[data-disabled="true"] .ui-slider__input {
  cursor: not-allowed;
}

.ui-slider--motion-custom,
.ui-slider[data-motion-source="custom"] {
  --ui-slider-fill-bg: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-slider--label-custom,
.ui-slider[data-label-source="custom"] {
  color: color-mix(in oklch, var(--ui-fg), var(--ui-accent) 10%);
}

.ui-slider--custom-class,
.ui-slider[data-custom-class="true"] {
  isolation: isolate;
}
"#;
