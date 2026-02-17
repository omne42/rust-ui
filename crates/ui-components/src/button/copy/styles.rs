pub const CSS: &str = r#"
.ui-button-copy {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  position: relative;
  --ui-button-copy-burst: 0;
  --ui-button-copy-feedback-scale: 0.08;
  --ui-button-copy-feedback-glow: 1;
  transform: scale(
    calc(1 + (var(--ui-button-copy-burst) * var(--ui-button-copy-feedback-scale)))
  );
  transform-origin: center;
}

.ui-button-copy[data-motion-source="custom"],
.ui-button-copy[data-custom-motion="true"] {
  --ui-button-copy-custom-motion: 1;
}

.ui-button-copy::after {
  content: "";
  position: absolute;
  inset: -2px;
  border-radius: var(--ui-radius-md);
  pointer-events: none;
  opacity: calc(var(--ui-button-copy-burst) * 0.35 * var(--ui-button-copy-feedback-glow));
  box-shadow: 0 0 0 calc(var(--ui-space-xs) + (var(--ui-button-copy-burst) * 8px))
    color-mix(in oklch, var(--ui-accent), transparent 66%);
}

.ui-button-copy__button {
  position: relative;
  z-index: 1;
}

.ui-button-copy[data-copied="true"] .ui-button-copy__button {
  box-shadow: var(--ui-shadow-md);
}

.ui-button-copy[data-mode="icon-only"] .ui-button-copy__button {
  min-width: 0;
}

.ui-button-copy__content {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-button-copy__icon {
  width: 1em;
  height: 1em;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: transform 180ms ease;
}

.ui-button-copy[data-copied="true"] .ui-button-copy__icon {
  transform: scale(1.08);
}

.ui-button-copy__icon-svg {
  width: 1em;
  height: 1em;
  stroke: currentColor;
}

.ui-button-copy__text {
  display: inline-flex;
  align-items: center;
}

.ui-button-copy__a11y-status {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

@media (prefers-reduced-motion: reduce) {
  .ui-button-copy {
    transform: none;
  }

  .ui-button-copy__icon {
    transition: none;
  }

  .ui-button-copy::after {
    opacity: 0;
  }
}
"#;
