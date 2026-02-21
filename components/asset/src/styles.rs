pub const CSS: &str = r#"
.ui-asset {
  --ui-asset-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-asset-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-asset-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-asset-icon-min-size: calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) * 1.125);
  --ui-asset-icon-max-size: calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) * 3.4);
  --ui-asset-focus-shadow-blur: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2);
  display: inline-flex;
  color: var(--ui-asset-fg);
}

.ui-asset__content {
  inline-size: 100%;
  block-size: 100%;
  display: grid;
  place-items: center;
}

.ui-asset__icon {
  inline-size: max(var(--ui-asset-icon-min-size), min(100%, var(--ui-asset-icon-max-size)));
  block-size: 100%;
  max-inline-size: 100%;
  max-block-size: 100%;
  color: color-mix(in oklch, var(--ui-asset-fg) 82%, var(--ui-asset-bg));
}

.ui-asset__icon--file,
.ui-asset[data-variant="file"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-asset-fg) 86%, var(--ui-asset-accent) 14%);
}

.ui-asset__icon--folder,
.ui-asset[data-variant="folder"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-asset-accent) 64%, var(--ui-asset-fg) 36%);
}

.ui-asset__icon--custom-fallback,
.ui-asset[data-variant="custom"] .ui-asset__icon {
  color: color-mix(in oklch, var(--ui-asset-fg) 72%, var(--ui-asset-accent) 28%);
}

.ui-asset--selected .ui-asset__icon,
.ui-asset[data-selected="true"] .ui-asset__icon {
  color: var(--ui-asset-accent);
}

.ui-asset--focused .ui-asset__icon,
.ui-asset[data-focused="true"] .ui-asset__icon {
  filter: drop-shadow(
    0 0 var(--ui-asset-focus-shadow-blur)
      color-mix(in oklch, var(--ui-asset-accent) 38%, transparent)
  );
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
