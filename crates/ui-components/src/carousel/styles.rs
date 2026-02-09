pub const CSS: &str = r#"
.ui-carousel {
  display: grid;
  gap: var(--ui-space-sm);
  max-width: min(100%, 42rem);
}

.ui-carousel__viewport {
  position: relative;
  overflow: hidden;
  min-height: 10rem;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: color-mix(in oklab, var(--ui-bg) 92%, var(--ui-bg-muted) 8%);
}

.ui-carousel__slide {
  display: none;
  grid-template-rows: auto auto;
  gap: var(--ui-space-xs);
  min-height: 10rem;
  padding: var(--ui-space-lg);
}

.ui-carousel__slide[data-selected="true"] {
  display: grid;
}

.ui-carousel__slide[data-disabled="true"] {
  opacity: 0.56;
}

.ui-carousel__title {
  margin: 0;
  font-size: var(--ui-font-size-lg);
  font-weight: var(--ui-font-weight-semibold);
  color: var(--ui-fg);
}

.ui-carousel__description {
  margin: 0;
  color: var(--ui-fg-muted);
  font-size: var(--ui-font-size-sm);
}

.ui-carousel__controls {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-carousel__button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 2rem;
  min-width: 5.5rem;
  padding: 0 var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;
  cursor: pointer;
}

.ui-carousel__button:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-carousel__button:disabled {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-carousel__indicators {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  padding: var(--ui-space-2xs);
}

.ui-carousel__indicator {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  border: 0;
  border-radius: var(--ui-radius-sm);
  background: transparent;
  cursor: pointer;
}

.ui-carousel__indicator-dot {
  width: 0.5rem;
  height: 0.5rem;
  border-radius: var(--ui-radius-pill);
  background: color-mix(in oklab, var(--ui-fg-muted) 72%, transparent 28%);
}

.ui-carousel__indicator[data-selected="true"] .ui-carousel__indicator-dot {
  background: var(--ui-accent-contrast);
}

.ui-carousel__indicator:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-carousel__indicator:disabled {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-carousel--vertical .ui-carousel__controls {
  justify-content: flex-start;
}

.ui-carousel--empty .ui-carousel__viewport {
  border-color: color-mix(in oklab, var(--ui-border) 72%, var(--ui-fg-muted) 28%);
}

.ui-carousel--selected .ui-carousel__viewport {
  box-shadow: var(--ui-shadow-sm);
}
"#;
