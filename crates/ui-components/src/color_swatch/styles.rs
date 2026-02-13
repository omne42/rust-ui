pub const CSS: &str = r#"
.ui-color-swatch {
  --ui-color-swatch-size: 1.25rem;
  --ui-color-swatch-inline-size: var(--ui-color-swatch-size);
  --ui-color-swatch-radius: var(--ui-radius-sm);
  --ui-color-swatch-color: transparent;

  position: relative;
  display: inline-flex;
  inline-size: var(--ui-color-swatch-inline-size);
  block-size: var(--ui-color-swatch-size);
  border-radius: var(--ui-color-swatch-radius);
  overflow: hidden;
  box-sizing: border-box;
  background: var(--ui-bg-muted);
  opacity: var(--ui-im-opacity, 1);
  transform: translateY(var(--ui-im-y, 0px));
  will-change: transform, opacity;
}

.ui-color-swatch[data-motion-source="custom"],
.ui-color-swatch[data-custom-motion="true"] {
  --ui-color-swatch-custom-motion: 1;
}

.ui-color-swatch__checker,
.ui-color-swatch__sample,
.ui-color-swatch__slash {
  position: absolute;
  inset: 0;
}

.ui-color-swatch__checker {
  background: repeating-conic-gradient(
      color-mix(in oklab, var(--ui-fg) 8%, transparent) 0 25%,
      transparent 0 50%
    )
    0 0 / 0.5rem 0.5rem;
  opacity: 0;
}

.ui-color-swatch__sample {
  background: var(--ui-color-swatch-color);
}

.ui-color-swatch__slash {
  opacity: 0;
  background: linear-gradient(
    135deg,
    transparent calc(50% - 1px),
    color-mix(in oklab, var(--ui-danger) 82%, black 18%) calc(50% - 1px),
    color-mix(in oklab, var(--ui-danger) 82%, black 18%) calc(50% + 1px),
    transparent calc(50% + 1px)
  );
}

.ui-color-swatch--size-xs,
.ui-color-swatch[data-size="xs"] {
  --ui-color-swatch-size: 0.875rem;
}

.ui-color-swatch--size-sm,
.ui-color-swatch[data-size="sm"] {
  --ui-color-swatch-size: 1rem;
}

.ui-color-swatch--size-md,
.ui-color-swatch[data-size="md"] {
  --ui-color-swatch-size: 1.25rem;
}

.ui-color-swatch--size-lg,
.ui-color-swatch[data-size="lg"] {
  --ui-color-swatch-size: 1.5rem;
}

.ui-color-swatch--rounding-default,
.ui-color-swatch[data-rounding="default"] {
  --ui-color-swatch-radius: var(--ui-radius-sm);
}

.ui-color-swatch--rounding-none,
.ui-color-swatch[data-rounding="none"] {
  --ui-color-swatch-radius: 0;
}

.ui-color-swatch--rounding-full,
.ui-color-swatch[data-rounding="full"] {
  --ui-color-swatch-radius: 999px;
}

.ui-color-swatch--shape-square,
.ui-color-swatch[data-shape="square"] {
  --ui-color-swatch-inline-size: var(--ui-color-swatch-size);
}

.ui-color-swatch--shape-wide,
.ui-color-swatch[data-shape="wide"] {
  --ui-color-swatch-inline-size: calc(var(--ui-color-swatch-size) * 2.5);
}

.ui-color-swatch--bordered,
.ui-color-swatch[data-bordered="true"] {
  border: 1px solid color-mix(in oklab, var(--ui-fg) 24%, transparent);
}

.ui-color-swatch--alpha-opaque .ui-color-swatch__sample,
.ui-color-swatch[data-alpha="opaque"] .ui-color-swatch__sample {
  opacity: 1;
}

.ui-color-swatch--alpha-translucent .ui-color-swatch__checker,
.ui-color-swatch[data-alpha="translucent"] .ui-color-swatch__checker,
.ui-color-swatch--alpha-transparent .ui-color-swatch__checker,
.ui-color-swatch[data-alpha="transparent"] .ui-color-swatch__checker {
  opacity: 1;
}

.ui-color-swatch--alpha-transparent .ui-color-swatch__sample,
.ui-color-swatch[data-alpha="transparent"] .ui-color-swatch__sample,
.ui-color-swatch--alpha-none .ui-color-swatch__sample,
.ui-color-swatch[data-alpha="none"] .ui-color-swatch__sample {
  opacity: 0;
}

.ui-color-swatch--alpha-transparent .ui-color-swatch__slash,
.ui-color-swatch[data-alpha="transparent"] .ui-color-swatch__slash,
.ui-color-swatch--alpha-none .ui-color-swatch__slash,
.ui-color-swatch[data-alpha="none"] .ui-color-swatch__slash {
  opacity: 1;
}

.ui-color-swatch--custom-class,
.ui-color-swatch[data-custom-class="true"] {
  --ui-color-swatch-has-custom-class: 1;
}
"#;
