pub const CSS: &str = r#"
.ui-swatch {
  --ui-swatch-size: 1.25rem;
  --ui-swatch-inline-size: var(--ui-swatch-size);
  --ui-swatch-radius: var(--ui-radius-sm);
  --ui-swatch-color: transparent;
  --ui-swatch-scale: 1;
  --ui-swatch-ring-opacity: 0;

  position: relative;
  display: inline-flex;
  inline-size: var(--ui-swatch-inline-size);
  block-size: var(--ui-swatch-size);
  border-radius: var(--ui-swatch-radius);
  overflow: hidden;
  box-sizing: border-box;
  cursor: pointer;
  outline: none;
  border: 1px solid color-mix(in oklch, var(--ui-fg) 20%, transparent);
  background: var(--ui-bg-muted);
  transform: scale(var(--ui-swatch-scale));
  will-change: transform;
}

.ui-swatch::after {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  box-shadow: inset 0 0 0 2px color-mix(in oklch, var(--ui-accent) 82%, transparent);
  opacity: var(--ui-swatch-ring-opacity);
  pointer-events: none;
}

.ui-swatch__checker,
.ui-swatch__sample,
.ui-swatch__slash,
.ui-swatch__mixed-mark,
.ui-swatch__disabled-mark {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.ui-swatch__checker {
  background: repeating-conic-gradient(
      color-mix(in oklab, var(--ui-fg) 8%, transparent) 0 25%,
      transparent 0 50%
    )
    0 0 / 0.5rem 0.5rem;
  opacity: 0;
}

.ui-swatch__sample {
  background: var(--ui-swatch-color);
}

.ui-swatch__slash {
  opacity: 0;
  background: linear-gradient(
    135deg,
    transparent calc(50% - 1px),
    color-mix(in oklab, var(--ui-danger) 82%, var(--ui-bg) 18%) calc(50% - 1px),
    color-mix(in oklab, var(--ui-danger) 82%, var(--ui-bg) 18%) calc(50% + 1px),
    transparent calc(50% + 1px)
  );
}

.ui-swatch__mixed-mark {
  inset: auto 24% auto 24%;
  top: 50%;
  height: 2px;
  transform: translateY(-50%);
  border-radius: 999px;
  background: color-mix(in oklab, var(--ui-fg) 82%, transparent);
  opacity: 0;
}

.ui-swatch__disabled-mark {
  opacity: 0;
  background: radial-gradient(
      circle at center,
      color-mix(in oklab, var(--ui-bg) 70%, transparent) 0 38%,
      transparent 39%
    ),
    linear-gradient(
      135deg,
      transparent calc(50% - 1px),
      color-mix(in oklab, var(--ui-fg-muted) 72%, transparent) calc(50% - 1px),
      color-mix(in oklab, var(--ui-fg-muted) 72%, transparent) calc(50% + 1px),
      transparent calc(50% + 1px)
    );
}

.ui-swatch--size-xs,
.ui-swatch[data-size="xs"] {
  --ui-swatch-size: 0.875rem;
}

.ui-swatch--size-s,
.ui-swatch[data-size="s"] {
  --ui-swatch-size: 1rem;
}

.ui-swatch--size-m,
.ui-swatch[data-size="m"] {
  --ui-swatch-size: 1.25rem;
}

.ui-swatch--size-l,
.ui-swatch[data-size="l"] {
  --ui-swatch-size: 1.5rem;
}

.ui-swatch--rounding-default,
.ui-swatch[data-rounding="default"] {
  --ui-swatch-radius: var(--ui-radius-sm);
}

.ui-swatch--rounding-none,
.ui-swatch[data-rounding="none"] {
  --ui-swatch-radius: 0;
}

.ui-swatch--rounding-full,
.ui-swatch[data-rounding="full"] {
  --ui-swatch-radius: 999px;
}

.ui-swatch--shape-square,
.ui-swatch[data-shape="square"] {
  --ui-swatch-inline-size: var(--ui-swatch-size);
}

.ui-swatch--shape-rectangle,
.ui-swatch[data-shape="rectangle"] {
  --ui-swatch-inline-size: calc(var(--ui-swatch-size) * 2.5);
}

.ui-swatch--border-default,
.ui-swatch[data-border="default"] {
  border-color: color-mix(in oklch, var(--ui-fg) 20%, transparent);
}

.ui-swatch--border-light,
.ui-swatch[data-border="light"] {
  border-color: color-mix(in oklch, var(--ui-fg) 10%, transparent);
}

.ui-swatch--border-none,
.ui-swatch[data-border="none"] {
  border-color: transparent;
}

.ui-swatch--nothing .ui-swatch__checker,
.ui-swatch[data-nothing="true"] .ui-swatch__checker {
  opacity: 1;
}

.ui-swatch--nothing .ui-swatch__sample,
.ui-swatch[data-nothing="true"] .ui-swatch__sample {
  opacity: 0;
}

.ui-swatch--nothing .ui-swatch__slash,
.ui-swatch[data-nothing="true"] .ui-swatch__slash {
  opacity: 1;
}

.ui-swatch--mixed .ui-swatch__checker,
.ui-swatch[data-mixed-value="true"] .ui-swatch__checker {
  opacity: 1;
}

.ui-swatch--mixed .ui-swatch__sample,
.ui-swatch[data-mixed-value="true"] .ui-swatch__sample {
  opacity: 0;
}

.ui-swatch--mixed .ui-swatch__mixed-mark,
.ui-swatch[data-mixed-value="true"] .ui-swatch__mixed-mark {
  opacity: 1;
}

.ui-swatch--disabled,
.ui-swatch[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.75;
}

.ui-swatch--disabled .ui-swatch__disabled-mark,
.ui-swatch[data-disabled="true"] .ui-swatch__disabled-mark {
  opacity: 1;
}

.ui-swatch[data-selected="true"] {
  --ui-swatch-ring-opacity: 1;
}

.ui-swatch:focus-visible {
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent) 24%, transparent);
}

.ui-swatch--static,
.ui-swatch[data-decorative="true"] {
  cursor: default;
}
"#;
