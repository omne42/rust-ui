pub const CSS: &str = r#"
.ui-flip-card {
  position: relative;
  display: inline-flex;
  inline-size: min(
    var(--ui-flip-card-max-inline-size, var(--ui-fallback-flip-card-max-inline-size)),
    var(--ui-flip-card-max-inline-viewport, var(--ui-fallback-flip-card-max-inline-viewport))
  );
  aspect-ratio:
    var(--ui-flip-card-aspect-ratio-width, var(--ui-fallback-flip-card-aspect-ratio-width))
    /
    var(--ui-flip-card-aspect-ratio-height, var(--ui-fallback-flip-card-aspect-ratio-height));
  perspective: var(--ui-flip-card-perspective, var(--ui-fallback-flip-card-perspective));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  cursor: pointer;
  outline: none;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-flip-card[data-disabled="true"] {
  opacity: var(--ui-flip-card-disabled-opacity, var(--ui-fallback-flip-card-disabled-opacity));
  cursor: not-allowed;
}

.ui-flip-card[data-motion-source="custom"],
.ui-flip-card[data-custom-motion="true"],
.ui-flip-card--custom-motion {
  --ui-flip-card-custom-motion: 1;
}

.ui-flip-card[data-class-source="custom"],
.ui-flip-card--custom-class {
  --ui-flip-card-class-source: custom;
}

.ui-flip-card[data-id-source="custom"],
.ui-flip-card--custom-id {
  --ui-flip-card-id-source: custom;
}

.ui-flip-card[data-flip-mode="hover"],
.ui-flip-card--hover {
  --ui-flip-card-flip-mode: hover;
}

.ui-flip-card[data-flip-mode="toggle"],
.ui-flip-card--toggle {
  --ui-flip-card-flip-mode: toggle;
}

.ui-flip-card:focus-visible {
  box-shadow: 0 0 0
    var(--ui-flip-card-focus-outline-width, var(--ui-fallback-flip-card-focus-outline-width))
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
}

.ui-flip-card__inner {
  position: relative;
  inline-size: 100%;
  block-size: 100%;
  transform-style: preserve-3d;
  transform: rotateY(var(--ui-flip-card-rotation, 0deg))
    scale(var(--ui-flip-card-scale, 1))
    rotateX(var(--ui-flip-card-tilt, 0deg));
  will-change: transform;
}

.ui-flip-card__face {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  backface-visibility: hidden;
  display: grid;
  align-content: center;
  justify-items: start;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  padding: var(--ui-space-lg, var(--ui-fallback-space-lg));
}

.ui-flip-card__front {
  transform: rotateY(0deg);
}

.ui-flip-card__back {
  transform: rotateY(180deg);
}

.ui-flip-card__face[data-visible="true"],
.ui-flip-card__face--visible {
  --ui-flip-card-face-visible: 1;
}

.ui-flip-card__face[data-visible="false"],
.ui-flip-card__face--hidden {
  --ui-flip-card-face-visible: 0;
}

.ui-flip-card__title {
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));
  line-height: var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height));
  font-weight: var(--ui-flip-card-title-font-weight, var(--ui-fallback-flip-card-title-font-weight));
}

.ui-flip-card__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

@media (prefers-reduced-motion: reduce) {
  .ui-flip-card__inner {
    transition: none;
  }
}
"#;
