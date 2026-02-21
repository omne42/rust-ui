pub const CSS: &str = r#"
.ui-color-loupe {
  --ui-color-loupe-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-color-loupe-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-loupe-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-color-loupe-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-color-loupe-space-xl: var(--ui-space-xl, var(--ui-fallback-space-xl));
  --ui-color-loupe-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-loupe-radius-lg: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  --ui-color-loupe-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-color-loupe-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-loupe-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-loupe-border: var(--ui-border, var(--ui-fallback-border));
  --ui-color-loupe-z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));
  --ui-color-loupe-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-color-loupe-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-color-loupe-rest-offset: var(--ui-color-loupe-space-xs);
  --ui-color-loupe-enter-offset: var(--ui-color-loupe-space-2xs);
  --ui-color-loupe-rest-y: -100%;
  --ui-color-loupe-enter-y: -86%;
  --ui-color-loupe-overshoot-y: -104%;
  --ui-color-loupe-initial-scale: 0.84;
  --ui-color-loupe-enter-scale: 0.76;
  --ui-color-loupe-overshoot-scale: 1.04;
  --ui-color-loupe-inline-size: calc(
    var(--ui-color-loupe-space-xl) * 2 + var(--ui-color-loupe-space-sm)
  );
  --ui-color-loupe-block-size: calc(
    var(--ui-color-loupe-inline-size) + var(--ui-color-loupe-space-md)
  );
  --ui-color-loupe-bubble-tail-gap: calc(
    var(--ui-color-loupe-space-sm) + var(--ui-color-loupe-space-2xs)
  );
  --ui-color-loupe-checker-inset: var(--ui-color-loupe-space-2xs);
  --ui-color-loupe-checker-size: var(--ui-color-loupe-space-sm);
  --ui-color-loupe-checker-shift: calc(var(--ui-color-loupe-checker-size) / 2);
  --ui-color-loupe-tail-size: var(--ui-color-loupe-space-md);
  --ui-color-loupe-tail-bottom: calc(var(--ui-color-loupe-space-2xs) / 2);
  position: absolute;
  inline-size: var(--ui-color-loupe-inline-size);
  block-size: var(--ui-color-loupe-block-size);
  transform: translate(-50%, calc(var(--ui-color-loupe-rest-y) - var(--ui-color-loupe-rest-offset))) scale(var(--ui-color-loupe-initial-scale));
  transform-origin: 50% 100%;
  opacity: 0;
  pointer-events: none;
  z-index: var(--ui-color-loupe-z-index);
  filter: drop-shadow(
    0 var(--ui-color-loupe-space-2xs) calc(var(--ui-color-loupe-space-md) + var(--ui-color-loupe-space-2xs))
      color-mix(in oklch, var(--ui-color-loupe-fg), transparent 84%)
  );
}

.ui-color-loupe--x-start {
  left: 16%;
}

.ui-color-loupe--x-center {
  left: 50%;
}

.ui-color-loupe--x-end {
  left: 84%;
}

.ui-color-loupe--y-start {
  top: 16%;
}

.ui-color-loupe--y-center {
  top: 50%;
}

.ui-color-loupe--y-end {
  top: 84%;
}

.ui-color-loupe__bubble {
  position: absolute;
  inset: 0 0 var(--ui-color-loupe-bubble-tail-gap);
  border-radius: var(--ui-color-loupe-radius-lg);
  border: var(--ui-color-loupe-border-width) solid
    color-mix(in oklch, var(--ui-color-loupe-border), transparent 18%);
  background: color-mix(in oklch, var(--ui-color-loupe-bg), var(--ui-color-loupe-fg) 2%);
  overflow: hidden;
}

.ui-color-loupe__checker {
  position: absolute;
  inset: var(--ui-color-loupe-checker-inset);
  border-radius: inherit;
  background:
    linear-gradient(45deg, color-mix(in oklch, var(--ui-color-loupe-border), transparent 32%) 25%, transparent 25%, transparent 75%, color-mix(in oklch, var(--ui-color-loupe-border), transparent 32%) 75%),
    linear-gradient(45deg, color-mix(in oklch, var(--ui-color-loupe-border), transparent 32%) 25%, transparent 25%, transparent 75%, color-mix(in oklch, var(--ui-color-loupe-border), transparent 32%) 75%);
  background-size: var(--ui-color-loupe-checker-size) var(--ui-color-loupe-checker-size);
  background-position: 0 0, var(--ui-color-loupe-checker-shift) var(--ui-color-loupe-checker-shift);
  background-color: color-mix(in oklch, var(--ui-color-loupe-bg), var(--ui-color-loupe-fg) 8%);
}

.ui-color-loupe__fill {
  position: absolute;
  inset: var(--ui-color-loupe-checker-inset);
  border-radius: inherit;
  overflow: hidden;
}

.ui-color-loupe__swatch.ui-color-swatch {
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
  min-inline-size: 0;
  min-block-size: 0;
}

.ui-color-loupe__tail {
  position: absolute;
  left: 50%;
  bottom: var(--ui-color-loupe-tail-bottom);
  inline-size: var(--ui-color-loupe-tail-size);
  block-size: var(--ui-color-loupe-tail-size);
  transform: translateX(-50%) rotate(45deg);
  border-right: var(--ui-color-loupe-border-width) solid
    color-mix(in oklch, var(--ui-color-loupe-border), transparent 18%);
  border-bottom: var(--ui-color-loupe-border-width) solid
    color-mix(in oklch, var(--ui-color-loupe-border), transparent 18%);
  border-radius: 0 0 var(--ui-color-loupe-radius-sm) 0;
  background: color-mix(in oklch, var(--ui-color-loupe-bg), var(--ui-color-loupe-fg) 2%);
}

.ui-color-loupe--open,
.ui-color-loupe[data-open="true"],
.ui-color-loupe[data-state="open"] {
  opacity: 1;
  transform: translate(-50%, calc(var(--ui-color-loupe-rest-y) - var(--ui-color-loupe-rest-offset))) scale(1);
  animation:
    ui-color-loupe-open
    var(--ui-color-loupe-motion-duration)
    var(--ui-color-loupe-motion-easing);
}

.ui-color-loupe--disabled,
.ui-color-loupe[data-disabled="true"] {
  opacity: 0.4;
}

.ui-color-loupe--disabled .ui-color-loupe__bubble,
.ui-color-loupe[data-disabled="true"] .ui-color-loupe__bubble {
  border-color: color-mix(in oklch, var(--ui-color-loupe-border), transparent 34%);
}

.ui-color-loupe--custom-class,
.ui-color-loupe[data-custom-class="true"] {
  isolation: isolate;
}

@keyframes ui-color-loupe-open {
  from {
    opacity: 0;
    transform: translate(-50%, calc(var(--ui-color-loupe-enter-y) - var(--ui-color-loupe-enter-offset))) scale(var(--ui-color-loupe-enter-scale));
  }

  62% {
    opacity: 1;
    transform: translate(-50%, calc(var(--ui-color-loupe-overshoot-y) - var(--ui-color-loupe-rest-offset))) scale(var(--ui-color-loupe-overshoot-scale));
  }

  to {
    opacity: 1;
    transform: translate(-50%, calc(var(--ui-color-loupe-rest-y) - var(--ui-color-loupe-rest-offset))) scale(1);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-color-loupe--open,
  .ui-color-loupe[data-open="true"],
  .ui-color-loupe[data-state="open"] {
    animation: none;
  }
}
"#;
