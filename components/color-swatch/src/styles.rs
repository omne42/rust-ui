pub const CSS: &str = r#"
.ui-color-swatch {
  --ui-color-swatch-size: var(--ui-color-swatch-size-md, var(--ui-fallback-color-swatch-size-md));
  --ui-color-swatch-inline-size: var(--ui-color-swatch-size);
  --ui-color-swatch-radius: var(
    --ui-color-swatch-radius-default,
    var(--ui-fallback-color-swatch-radius-default)
  );
  --ui-color-swatch-color: transparent;

  position: relative;
  display: inline-flex;
  inline-size: var(--ui-color-swatch-inline-size);
  block-size: var(--ui-color-swatch-size);
  border-radius: var(--ui-color-swatch-radius);
  overflow: hidden;
  box-sizing: border-box;
  background: var(--ui-bg-muted);
  opacity: var(--ui-color-swatch-opacity, 1);
  transform: translateY(var(--ui-color-swatch-y, var(--ui-fallback-color-swatch-y)));
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
    0 0 / var(--ui-color-swatch-checker-size, var(--ui-fallback-color-swatch-checker-size))
    var(--ui-color-swatch-checker-size, var(--ui-fallback-color-swatch-checker-size));
  opacity: 0;
}

.ui-color-swatch__sample {
  background: var(--ui-color-swatch-color);
}

.ui-color-swatch__slash {
  opacity: 0;
  --ui-color-swatch-slash-half-width: calc(
    var(--ui-color-swatch-slash-width, var(--ui-fallback-color-swatch-slash-width)) / 2
  );
  background: linear-gradient(
    135deg,
    transparent calc(50% - var(--ui-color-swatch-slash-half-width)),
    color-mix(in oklab, var(--ui-danger) 82%, black 18%)
      calc(50% - var(--ui-color-swatch-slash-half-width)),
    color-mix(in oklab, var(--ui-danger) 82%, black 18%)
      calc(50% + var(--ui-color-swatch-slash-half-width)),
    transparent calc(50% + var(--ui-color-swatch-slash-half-width))
  );
}

.ui-color-swatch--size-xs,
.ui-color-swatch[data-size="xs"] {
  --ui-color-swatch-size: var(--ui-color-swatch-size-xs, var(--ui-fallback-color-swatch-size-xs));
}

.ui-color-swatch--size-sm,
.ui-color-swatch[data-size="sm"] {
  --ui-color-swatch-size: var(--ui-color-swatch-size-sm, var(--ui-fallback-color-swatch-size-sm));
}

.ui-color-swatch--size-md,
.ui-color-swatch[data-size="md"] {
  --ui-color-swatch-size: var(--ui-color-swatch-size-md, var(--ui-fallback-color-swatch-size-md));
}

.ui-color-swatch--size-lg,
.ui-color-swatch[data-size="lg"] {
  --ui-color-swatch-size: var(--ui-color-swatch-size-lg, var(--ui-fallback-color-swatch-size-lg));
}

.ui-color-swatch--rounding-default,
.ui-color-swatch[data-rounding="default"] {
  --ui-color-swatch-radius: var(
    --ui-color-swatch-radius-default,
    var(--ui-fallback-color-swatch-radius-default)
  );
}

.ui-color-swatch--rounding-none,
.ui-color-swatch[data-rounding="none"] {
  --ui-color-swatch-radius: var(
    --ui-color-swatch-radius-none,
    var(--ui-fallback-color-swatch-radius-none)
  );
}

.ui-color-swatch--rounding-full,
.ui-color-swatch[data-rounding="full"] {
  --ui-color-swatch-radius: var(
    --ui-color-swatch-radius-full,
    var(--ui-fallback-color-swatch-radius-full)
  );
}

.ui-color-swatch--shape-square,
.ui-color-swatch[data-shape="square"] {
  --ui-color-swatch-inline-size: var(--ui-color-swatch-size);
}

.ui-color-swatch--shape-wide,
.ui-color-swatch[data-shape="wide"] {
  --ui-color-swatch-inline-size: calc(
    var(--ui-color-swatch-size) *
      var(--ui-color-swatch-wide-multiplier, var(--ui-fallback-color-swatch-wide-multiplier))
  );
}

.ui-color-swatch--bordered,
.ui-color-swatch[data-bordered="true"] {
  border: var(--ui-color-swatch-border-width, var(--ui-fallback-color-swatch-border-width))
    solid color-mix(in oklab, var(--ui-fg) 24%, transparent);
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
