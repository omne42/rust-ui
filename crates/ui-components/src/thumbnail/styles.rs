pub const CSS: &str = r#"
.ui-thumbnail {
  --ui-thumbnail-size: 3.5rem;
  --ui-thumbnail-background: transparent;
  --ui-thumbnail-scale: 1;
  --ui-thumbnail-ring-opacity: 0;

  position: relative;
  display: inline-flex;
  inline-size: var(--ui-thumbnail-size);
  block-size: var(--ui-thumbnail-size);
  border-radius: var(--ui-radius-sm);
  box-sizing: border-box;
  transform: scale(var(--ui-thumbnail-scale));
  will-change: transform;
}

.ui-thumbnail::after {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  box-shadow: inset 0 0 0 2px color-mix(in oklch, var(--ui-accent) 84%, transparent);
  opacity: var(--ui-thumbnail-ring-opacity);
  pointer-events: none;
}

.ui-thumbnail__frame {
  position: relative;
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
  border: 1px solid color-mix(in oklch, var(--ui-fg) 16%, transparent);
  overflow: hidden;
  background:
    linear-gradient(var(--ui-thumbnail-background), var(--ui-thumbnail-background)),
    repeating-conic-gradient(
      color-mix(in oklab, var(--ui-fg) 8%, transparent) 0 25%,
      transparent 0 50%
    )
    0 0 / 0.5rem 0.5rem;
}

.ui-thumbnail__content {
  inline-size: 100%;
  block-size: 100%;
  display: grid;
  place-items: center;
}

.ui-thumbnail__content > * {
  inline-size: 100%;
  block-size: 100%;
  object-fit: contain;
  display: block;
}

.ui-thumbnail--cover .ui-thumbnail__content > *,
.ui-thumbnail[data-cover="true"] .ui-thumbnail__content > * {
  object-fit: cover;
}

.ui-thumbnail--layer .ui-thumbnail__frame,
.ui-thumbnail[data-layer="true"] .ui-thumbnail__frame {
  border-color: color-mix(in oklch, var(--ui-accent) 42%, transparent);
  box-shadow: inset 0 0 0 1px color-mix(in oklch, var(--ui-accent) 26%, transparent);
}

.ui-thumbnail--selected,
.ui-thumbnail[data-selected="true"] {
  --ui-thumbnail-ring-opacity: 1;
}

.ui-thumbnail--focused .ui-thumbnail__frame,
.ui-thumbnail[data-focused="true"] .ui-thumbnail__frame {
  outline: 2px solid color-mix(in oklch, var(--ui-accent) 36%, transparent);
  outline-offset: 1px;
}

.ui-thumbnail--size-50,
.ui-thumbnail[data-size="50"] {
  --ui-thumbnail-size: 1rem;
}

.ui-thumbnail--size-75,
.ui-thumbnail[data-size="75"] {
  --ui-thumbnail-size: 1.25rem;
}

.ui-thumbnail--size-100,
.ui-thumbnail[data-size="100"] {
  --ui-thumbnail-size: 1.5rem;
}

.ui-thumbnail--size-200,
.ui-thumbnail[data-size="200"] {
  --ui-thumbnail-size: 2rem;
}

.ui-thumbnail--size-300,
.ui-thumbnail[data-size="300"] {
  --ui-thumbnail-size: 2.5rem;
}

.ui-thumbnail--size-400,
.ui-thumbnail[data-size="400"] {
  --ui-thumbnail-size: 3rem;
}

.ui-thumbnail--size-500,
.ui-thumbnail[data-size="500"] {
  --ui-thumbnail-size: 3.5rem;
}

.ui-thumbnail--size-600,
.ui-thumbnail[data-size="600"] {
  --ui-thumbnail-size: 4rem;
}

.ui-thumbnail--size-700,
.ui-thumbnail[data-size="700"] {
  --ui-thumbnail-size: 4.5rem;
}

.ui-thumbnail--size-800,
.ui-thumbnail[data-size="800"] {
  --ui-thumbnail-size: 5rem;
}

.ui-thumbnail--size-900,
.ui-thumbnail[data-size="900"] {
  --ui-thumbnail-size: 5.5rem;
}

.ui-thumbnail--size-1000,
.ui-thumbnail[data-size="1000"] {
  --ui-thumbnail-size: 6rem;
}
"#;
