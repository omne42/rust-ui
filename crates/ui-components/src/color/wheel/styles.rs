pub const CSS: &str = r#"
.ui-color-wheel {
  --ui-slider-visual-percent: 0;
  --ui-color-wheel-size: 11rem;
  --ui-color-wheel-track-thickness: 1rem;
  --ui-color-wheel-thumb-size: 1rem;
  --ui-color-wheel-thumb-border: color-mix(in oklch, var(--ui-accent), var(--ui-border) 22%);

  display: inline-grid;
  gap: var(--ui-space-xs);
  color: var(--ui-fg);
}

.ui-color-wheel__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-xs);
}

.ui-color-wheel__label {
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 600;
}

.ui-color-wheel__value {
  font-variant-numeric: tabular-nums;
  color: var(--ui-fg-muted);
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
}

.ui-color-wheel__track {
  position: relative;
  inline-size: var(--ui-color-wheel-size);
  block-size: var(--ui-color-wheel-size);
  border-radius: 50%;
  touch-action: none;
}

.ui-color-wheel__ring {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: conic-gradient(
    #ff0000 0%,
    #ffff00 16.66%,
    #00ff00 33.33%,
    #00ffff 50%,
    #0000ff 66.66%,
    #ff00ff 83.33%,
    #ff0000 100%
  );
  -webkit-mask: radial-gradient(
    farthest-side,
    transparent calc(100% - var(--ui-color-wheel-track-thickness)),
    #000 calc(100% - var(--ui-color-wheel-track-thickness) + 1px)
  );
  mask: radial-gradient(
    farthest-side,
    transparent calc(100% - var(--ui-color-wheel-track-thickness)),
    #000 calc(100% - var(--ui-color-wheel-track-thickness) + 1px)
  );
}

.ui-color-wheel__orbit {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  pointer-events: none;
  transform: rotate(calc(var(--ui-slider-visual-percent) * 3.6deg));
}

.ui-color-wheel__thumb {
  position: absolute;
  inset-inline-start: 50%;
  inset-block-start: calc(var(--ui-color-wheel-track-thickness) / 2);
  inline-size: var(--ui-color-wheel-thumb-size);
  block-size: var(--ui-color-wheel-thumb-size);
  border-radius: 999px;
  border: 2px solid var(--ui-color-wheel-thumb-border);
  background: var(--ui-bg);
  transform: translate(-50%, -50%);
  box-shadow: var(--ui-shadow-sm);
}

.ui-color-wheel__input {
  position: absolute;
  inset: 0;
  margin: 0;
  opacity: 0;
  cursor: grab;
}

.ui-color-wheel__track[data-dragging="true"] .ui-color-wheel__input,
.ui-color-wheel__input:active {
  cursor: grabbing;
}

.ui-color-wheel__track:focus-within .ui-color-wheel__ring {
  outline: 2px solid color-mix(in oklch, var(--ui-accent), transparent 64%);
  outline-offset: 2px;
}

.ui-color-wheel--disabled,
.ui-color-wheel[data-disabled="true"] {
  opacity: 0.62;
}

.ui-color-wheel--disabled .ui-color-wheel__input,
.ui-color-wheel[data-disabled="true"] .ui-color-wheel__input {
  cursor: not-allowed;
}

.ui-color-wheel--motion-custom,
.ui-color-wheel[data-motion-source="custom"] {
  --ui-color-wheel-thumb-border: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 16%);
}

.ui-color-wheel--label-custom,
.ui-color-wheel[data-label-source="custom"] {
  color: color-mix(in oklch, var(--ui-fg), var(--ui-accent) 10%);
}

.ui-color-wheel--custom-class,
.ui-color-wheel[data-custom-class="true"] {
  isolation: isolate;
}
"#;
