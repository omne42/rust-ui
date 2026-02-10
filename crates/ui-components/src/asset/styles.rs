pub const CSS: &str = r#"
.ui-asset {
  display: inline-flex;
  color: var(--ui-fg);
}

.ui-asset__content {
  inline-size: 100%;
  block-size: 100%;
  display: grid;
  place-items: center;
}

.ui-asset__icon {
  inline-size: max(2.25rem, min(100%, 4.25rem));
  block-size: 100%;
  max-inline-size: 100%;
  max-block-size: 100%;
  color: color-mix(in oklch, var(--ui-fg) 82%, var(--ui-bg));
}

.ui-asset__icon--file,
.ui-asset[data-variant="file"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-fg) 86%, var(--ui-accent) 14%);
}

.ui-asset__icon--folder,
.ui-asset[data-variant="folder"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-accent) 64%, var(--ui-fg) 36%);
}

.ui-asset__icon--custom-fallback,
.ui-asset[data-variant="custom"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-fg) 72%, var(--ui-accent) 28%);
}

.ui-asset--selected .ui-asset__icon,
.ui-asset[data-selected="true"] .ui-asset__icon {
  color: var(--ui-accent);
}

.ui-asset--focused .ui-asset__icon,
.ui-asset[data-focused="true"] .ui-asset__icon {
  filter: drop-shadow(0 0 0.25rem color-mix(in oklch, var(--ui-accent) 38%, transparent));
}

.ui-asset--variant-file,
.ui-asset[data-variant="file"] {
  --ui-asset-variant: file;
}

.ui-asset--variant-folder,
.ui-asset[data-variant="folder"] {
  --ui-asset-variant: folder;
}

.ui-asset--variant-custom,
.ui-asset[data-variant="custom"] {
  --ui-asset-variant: custom;
}

.ui-asset--size-500,
.ui-asset[data-size="500"] {
  --ui-asset-scale: 1;
}

.ui-asset--size-600,
.ui-asset[data-size="600"] {
  --ui-asset-scale: 1.06;
}

.ui-asset--size-700,
.ui-asset[data-size="700"] {
  --ui-asset-scale: 1.12;
}

.ui-asset--size-800,
.ui-asset[data-size="800"] {
  --ui-asset-scale: 1.18;
}

.ui-asset--size-900,
.ui-asset[data-size="900"] {
  --ui-asset-scale: 1.24;
}

.ui-asset--size-1000,
.ui-asset[data-size="1000"] {
  --ui-asset-scale: 1.3;
}

.ui-asset .ui-asset__icon {
  transform: scale(var(--ui-asset-scale, 1));
  transition: transform 180ms ease, color 180ms ease, filter 180ms ease;
}

@media (forced-colors: active) {
  .ui-asset,
  .ui-asset__icon {
    color: CanvasText;
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-asset .ui-asset__icon {
    transition: none;
  }
}
"#;
