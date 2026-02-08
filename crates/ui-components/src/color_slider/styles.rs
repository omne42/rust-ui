pub const CSS: &str = r#"
.ui-color-slider {
  --ui-color-slider-percent: 0;
  --ui-slider-visual-percent: var(--ui-color-slider-percent);
  --ui-color-slider-track-height: 0.625rem;
  --ui-color-slider-thumb-size: 1rem;
  --ui-color-slider-track-start: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 16%);
  --ui-color-slider-track-end: var(--ui-accent);
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    var(--ui-color-slider-track-start) 0%,
    var(--ui-color-slider-track-end) 100%
  );
  --ui-color-slider-thumb-border: color-mix(in oklch, var(--ui-accent), var(--ui-border) 22%);

  display: inline-grid;
  gap: var(--ui-space-xs);
  width: min(100%, 22rem);
  color: var(--ui-fg);
}

.ui-color-slider__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-xs);
}

.ui-color-slider__label {
  font-size: 0.875rem;
  font-weight: 600;
}

.ui-color-slider__value {
  font-variant-numeric: tabular-nums;
  color: var(--ui-fg-muted);
  font-size: 0.8rem;
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
  border-radius: 999px;
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
    0 0 / 0.45rem 0.45rem;
}

.ui-color-slider__fill {
  position: absolute;
  inset-inline-start: 0;
  inset-block: 0;
  inline-size: calc(var(--ui-slider-visual-percent) * 1%);
  border-radius: inherit;
  background: color-mix(in oklch, white 22%, transparent);
  pointer-events: none;
}

.ui-color-slider__thumb {
  position: absolute;
  inset-block-start: 50%;
  inset-inline-start: calc(var(--ui-slider-visual-percent) * 1%);
  inline-size: var(--ui-color-slider-thumb-size);
  block-size: var(--ui-color-slider-thumb-size);
  border-radius: 999px;
  border: 2px solid var(--ui-color-slider-thumb-border);
  background: var(--ui-bg);
  transform: translate(-50%, -50%);
  box-shadow: var(--ui-shadow-sm);
  pointer-events: none;
}

.ui-color-slider__input:active + .ui-color-slider__track .ui-color-slider__thumb {
  transform: translate(-50%, -50%) scale(0.94);
}

.ui-color-slider__input:focus-visible + .ui-color-slider__track {
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent), transparent 66%);
}

.ui-color-slider--channel-hue,
.ui-color-slider[data-channel="hue"] {
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    #ff0000 0%,
    #ffff00 16.66%,
    #00ff00 33.33%,
    #00ffff 50%,
    #0000ff 66.66%,
    #ff00ff 83.33%,
    #ff0000 100%
  );
}

.ui-color-slider--channel-saturation,
.ui-color-slider[data-channel="saturation"] {
  --ui-color-slider-track-start: hsl(0 0% 50% / 1);
  --ui-color-slider-track-end: hsl(0 100% 50% / 1);
}

.ui-color-slider--channel-lightness,
.ui-color-slider[data-channel="lightness"] {
  --ui-color-slider-track-gradient: linear-gradient(
    90deg,
    hsl(0 100% 0% / 1) 0%,
    hsl(0 100% 50% / 1) 50%,
    hsl(0 0% 100% / 1) 100%
  );
}

.ui-color-slider--channel-alpha,
.ui-color-slider[data-channel="alpha"] {
  --ui-color-slider-track-start: transparent;
  --ui-color-slider-track-end: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 10%);
}

.ui-color-slider--channel-alpha .ui-color-slider__track::before,
.ui-color-slider[data-channel="alpha"] .ui-color-slider__track::before {
  opacity: 1;
}

.ui-color-slider--channel-red,
.ui-color-slider[data-channel="red"] {
  --ui-color-slider-track-start: rgb(0 0 0 / 0.85);
  --ui-color-slider-track-end: rgb(255 0 0 / 1);
}

.ui-color-slider--channel-green,
.ui-color-slider[data-channel="green"] {
  --ui-color-slider-track-start: rgb(0 0 0 / 0.85);
  --ui-color-slider-track-end: rgb(0 255 0 / 1);
}

.ui-color-slider--channel-blue,
.ui-color-slider[data-channel="blue"] {
  --ui-color-slider-track-start: rgb(0 0 0 / 0.85);
  --ui-color-slider-track-end: rgb(0 120 255 / 1);
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
  --ui-color-slider-thumb-border: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 16%);
}

.ui-color-slider--label-custom,
.ui-color-slider[data-label-source="custom"] {
  color: color-mix(in oklch, var(--ui-fg), var(--ui-accent) 10%);
}

.ui-color-slider--custom-class,
.ui-color-slider[data-custom-class="true"] {
  isolation: isolate;
}
"#;
