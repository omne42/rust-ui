pub const CSS: &str = r#"
.ui-flip-card {
  position: relative;
  display: inline-flex;
  inline-size: min(21rem, 92vw);
  aspect-ratio: 4 / 3;
  perspective: 1200px;
  border-radius: var(--ui-radius-lg);
  cursor: pointer;
  outline: none;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-flip-card[data-disabled="true"] {
  opacity: 0.6;
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
  box-shadow: 0 0 0 3px var(--ui-focus-ring);
}

.ui-flip-card__inner {
  position: relative;
  inline-size: 100%;
  block-size: 100%;
  transform-stYle: preserve-3d;
  transform: rotateY(var(--ui-flip-card-rotation, 0deg))
    scale(var(--ui-flip-card-scale, 1))
    rotateX(var(--ui-flip-card-tilt, 0deg));
  will-change: transform;
}

.ui-flip-card__face {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  backface-visibility: hidden;
  display: grid;
  align-content: center;
  justify-items: start;
  gap: var(--ui-space-xs);
  padding: var(--ui-space-lg);
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
  font-size: var(--ui-heading-h6-font-size, 14px);
  line-height: var(--ui-heading-h6-line-height, 20px);
  font-weight: 650;
}

.ui-flip-card__description {
  color: var(--ui-fg-muted);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
}

@media (prefers-reduced-motion: reduce) {
  .ui-flip-card__inner {
    transition: none;
  }
}
"#;
