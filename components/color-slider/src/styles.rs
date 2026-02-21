pub const CSS: &str = r#"
.ui-color-slider {
  --ui-color-slider-percent: 0;
  --ui-slider-visual-percent: var(--ui-color-slider-percent);
  --ui-color-slider-track-height: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-color-slider-thumb-size: var(--ui-icon-size-100, var(--ui-fallback-icon-size-100));
  --ui-color-slider-track-start: color-mix(in oklch, var(--ui-bg, var(--ui-fallback-bg)), var(--ui-fg, var(--ui-fallback-fg)) 16%);
  --ui-color-slider-track-end: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-slider-checker-size: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    var(--ui-color-slider-track-start) 0%,
    var(--ui-color-slider-track-end) 100%
  );
  --ui-color-slider-thumb-border: color-mix(in oklch, var(--ui-accent, var(--ui-fallback-accent)), var(--ui-border, var(--ui-fallback-border)) 22%);

  display: inline-grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  width: min(100%, var(--ui-slider-max-width, var(--ui-fallback-slider-max-width)));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-color-slider__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-color-slider__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 600;
}

.ui-color-slider__value {
  font-variant-numeric: tabular-nums;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-color-slider__control {
  position: relative;
  min-block-size: var(--ui-color-slider-thumb-size);
}

.ui-color-slider__input {
  position: absolute;
  inset: 0;
  margin: 0;
  inline-size: 100%;
  block-size: 100%;
  opacity: 0;
  cursor: pointer;
  z-index: 2;
}

.ui-color-slider__track {
  position: absolute;
  inset-inline: 0;
  inset-block-start: 50%;
  transform: translateY(-50%);
  block-size: var(--ui-color-slider-track-height);
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  overflow: hidden;
  background: var(--ui-color-slider-track-gradient);
}

.ui-color-slider__track::before {
  content: "";
  position: absolute;
  inset: 0;
  opacity: 0;
  background: repeating-conic-gradient(
      color-mix(in oklab, var(--ui-fg) 8%, transparent) 0 25%,
      transparent 0 50%
    )
    0 0 / var(--ui-color-slider-checker-size) var(--ui-color-slider-checker-size);
}

.ui-color-slider__fill {
  position: absolute;
  inset-inline-start: 0;
  inset-block: 0;
  inline-size: calc(var(--ui-slider-visual-percent) * 1%);
  border-radius: inherit;
  background: color-mix(in oklch, var(--ui-common-white, var(--ui-fallback-common-white)) 22%, transparent);
  pointer-events: none;
}

.ui-color-slider__thumb {
  position: absolute;
  inset-block-start: 50%;
  inset-inline-start: calc(var(--ui-slider-visual-percent) * 1%);
  inline-size: var(--ui-color-slider-thumb-size);
  block-size: var(--ui-color-slider-thumb-size);
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  border: var(--ui-slider-thumb-border-width, var(--ui-fallback-slider-thumb-border-width)) solid var(--ui-color-slider-thumb-border);
  background: var(--ui-bg, var(--ui-fallback-bg));
  transform: translate(-50%, -50%);
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  pointer-events: none;
}

.ui-color-slider[data-pressed="true"] .ui-color-slider__thumb {
  transform: translate(-50%, -50%) scale(0.94);
}

.ui-color-slider[data-hovered="true"] .ui-color-slider__track {
  box-shadow: inset 0 0 0 1px color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)), transparent 82%);
}

.ui-color-slider[data-focus-visible="true"] .ui-color-slider__track {
  box-shadow: 0 0 0 var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width)) color-mix(in oklch, var(--ui-focus-ring, var(--ui-fallback-focus-ring)), transparent 68%);
}

.ui-color-slider--channel-hue,
.ui-color-slider[data-channel="hue"] {
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    var(--ui-common-red-500, var(--ui-fallback-common-red-500)) 0%,
    var(--ui-common-yellow-500, var(--ui-fallback-common-yellow-500)) 16.66%,
    var(--ui-common-green-500, var(--ui-fallback-common-green-500)) 33.33%,
    var(--ui-common-cyan-500, var(--ui-fallback-common-cyan-500)) 50%,
    var(--ui-common-blue-500, var(--ui-fallback-common-blue-500)) 66.66%,
    var(--ui-common-purple-500, var(--ui-fallback-common-purple-500)) 83.33%,
    var(--ui-common-red-500, var(--ui-fallback-common-red-500)) 100%
  );
}

.ui-color-slider--channel-saturation,
.ui-color-slider[data-channel="saturation"] {
  --ui-color-slider-track-start: var(--ui-common-zinc-500, var(--ui-fallback-common-zinc-500));
  --ui-color-slider-track-end: var(--ui-common-red-500, var(--ui-fallback-common-red-500));
}

.ui-color-slider--channel-lightness,
.ui-color-slider[data-channel="lightness"] {
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    var(--ui-common-black, var(--ui-fallback-common-black)) 0%,
    var(--ui-common-red-500, var(--ui-fallback-common-red-500)) 50%,
    var(--ui-common-white, var(--ui-fallback-common-white)) 100%
  );
}

.ui-color-slider--channel-alpha,
.ui-color-slider[data-channel="alpha"] {
  --ui-color-slider-track-start: transparent;
  --ui-color-slider-track-end: color-mix(in oklch, var(--ui-accent, var(--ui-fallback-accent)), var(--ui-fg, var(--ui-fallback-fg)) 10%);
}

.ui-color-slider--channel-alpha .ui-color-slider__track::before,
.ui-color-slider[data-channel="alpha"] .ui-color-slider__track::before {
  opacity: 1;
}

.ui-color-slider--channel-red,
.ui-color-slider[data-channel="red"] {
  --ui-color-slider-track-start: var(--ui-common-black, var(--ui-fallback-common-black));
  --ui-color-slider-track-end: var(--ui-common-red-600, var(--ui-fallback-common-red-600));
}

.ui-color-slider--channel-green,
.ui-color-slider[data-channel="green"] {
  --ui-color-slider-track-start: var(--ui-common-black, var(--ui-fallback-common-black));
  --ui-color-slider-track-end: var(--ui-common-green-600, var(--ui-fallback-common-green-600));
}

.ui-color-slider--channel-blue,
.ui-color-slider[data-channel="blue"] {
  --ui-color-slider-track-start: var(--ui-common-black, var(--ui-fallback-common-black));
  --ui-color-slider-track-end: var(--ui-common-blue-600, var(--ui-fallback-common-blue-600));
}

.ui-color-slider--track-custom,
.ui-color-slider[data-track-source="custom"] {
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    var(--ui-color-slider-track-start) 0%,
    var(--ui-color-slider-track-end) 100%
  );
}

.ui-color-slider--disabled,
.ui-color-slider[data-disabled="true"] {
  opacity: 0.62;
}

.ui-color-slider--disabled .ui-color-slider__input,
.ui-color-slider[data-disabled="true"] .ui-color-slider__input {
  cursor: not-allowed;
}

.ui-color-slider--motion-custom,
.ui-color-slider[data-motion-source="custom"] {
  --ui-color-slider-thumb-border: color-mix(in oklch, var(--ui-accent, var(--ui-fallback-accent)), var(--ui-fg, var(--ui-fallback-fg)) 16%);
}

.ui-color-slider--label-custom,
.ui-color-slider[data-label-source="custom"] {
  color: color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)), var(--ui-accent, var(--ui-fallback-accent)) 10%);
}

.ui-color-slider--custom-class,
.ui-color-slider[data-custom-class="true"] {
  isolation: isolate;
}
"#;
