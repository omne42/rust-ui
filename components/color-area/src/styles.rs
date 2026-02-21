pub const CSS: &str = r#"
.ui-color-area {
  --ui-color-area-border-width: var(
    --ui-border-width,
    var(--ui-fallback-border-width)
  );
  --ui-color-area-preview-size: var(
    --ui-component-height-100,
    var(--ui-fallback-component-height-100)
  );
  --ui-color-area-cell-size: var(
    --ui-font-size-150,
    var(--ui-fallback-font-size-150)
  );
  --ui-color-area-thumb-size: calc(var(--ui-color-area-cell-size) / 2);
  --ui-color-area-thumb-radius: var(
    --ui-color-swatch-radius-full,
    var(--ui-fallback-color-swatch-radius-full)
  );
  --ui-color-area-disabled-opacity: var(
    --ui-checkbox-disabled-opacity,
    var(--ui-fallback-checkbox-disabled-opacity)
  );
  --ui-color-area-focus-ring-width: var(
    --ui-checkbox-focus-outline-width,
    var(--ui-fallback-checkbox-focus-outline-width)
  );
  --ui-color-area-focus-ring-offset: var(
    --ui-checkbox-focus-outline-offset,
    var(--ui-fallback-checkbox-focus-outline-offset)
  );
  --ui-color-area-common-white: var(--ui-common-white, var(--ui-fallback-common-white));
  --ui-color-area-common-black: var(--ui-common-black, var(--ui-fallback-fg));
  --ui-color-area-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-color-area-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-color-area-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-color-area-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-color-area-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-color-area-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-color-area-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-color-area-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-color-area-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-color-area-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-color-area-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-color-area-max-inline-size: calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) * 10);

  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-color-area-space-sm);
  min-inline-size: min(100%, var(--ui-color-area-max-inline-size));
  transition: opacity var(--ui-color-area-motion-duration) var(--ui-color-area-motion-easing);
}

.ui-color-area__label {
  color: var(--ui-color-area-fg-muted);
  font-size: var(--ui-color-area-font-size-100);
  font-weight: 600;
  line-height: var(--ui-color-area-line-height-100);
}

.ui-color-area__preview {
  inline-size: var(--ui-color-area-preview-size);
  block-size: var(--ui-color-area-preview-size);
  border-radius: var(--ui-color-area-radius-sm);
  border: var(--ui-color-area-border-width) solid color-mix(in oklab, var(--ui-color-area-fg-muted) 28%, transparent);
  background:
    var(--ui-color-area-preview-color, color-mix(in oklab, var(--ui-color-area-accent) 50%, var(--ui-color-area-common-white)));
}

.ui-color-area__grid {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-color-area-border-width);
  border: var(--ui-color-area-border-width) solid color-mix(in oklab, var(--ui-color-area-fg-muted) 28%, transparent);
  border-radius: var(--ui-color-area-radius-sm);
  padding: var(--ui-color-area-border-width);
  background:
    linear-gradient(to top, color-mix(in oklab, var(--ui-color-area-common-black) 22%, transparent), transparent),
    linear-gradient(to right, var(--ui-color-area-common-white), color-mix(in oklab, var(--ui-color-area-accent) 88%, transparent));
}

.ui-color-area__row {
  display: inline-flex;
  gap: var(--ui-color-area-border-width);
}

.ui-color-area__cell {
  appearance: none;
  border: none;
  padding: 0;
  margin: 0;
  inline-size: var(--ui-color-area-cell-size);
  block-size: var(--ui-color-area-cell-size);
  background: transparent;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.ui-color-area__thumb {
  inline-size: var(--ui-color-area-thumb-size);
  block-size: var(--ui-color-area-thumb-size);
  border-radius: var(--ui-color-area-thumb-radius);
  background: transparent;
  box-shadow: 0 0 0 var(--ui-color-area-border-width) transparent;
}

.ui-color-area__cell[data-selected="true"] .ui-color-area__thumb,
.ui-color-area__cell[aria-selected="true"] .ui-color-area__thumb {
  background: var(--ui-color-area-bg);
  box-shadow: 0 0 0 var(--ui-color-area-border-width) color-mix(in oklab, var(--ui-color-area-fg) 82%, transparent);
}

.ui-color-area__cell:focus-visible {
  outline: var(--ui-color-area-focus-ring-width) solid color-mix(in oklab, var(--ui-color-area-accent) 84%, transparent);
  outline-offset: var(--ui-color-area-focus-ring-offset);
}

.ui-color-area__axes {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: var(--ui-color-area-space-xs) var(--ui-color-area-space-sm);
  align-items: center;
}

.ui-color-area__axis-label {
  color: var(--ui-color-area-fg-muted);
  font-size: var(--ui-color-area-font-size-100);
  line-height: var(--ui-color-area-line-height-100);
}

.ui-color-area__axis-input {
  inline-size: 100%;
}

.ui-color-area--with-preview,
.ui-color-area[data-has-preview="true"] {
  --ui-color-area-with-preview: 1;
}

.ui-color-area--disabled,
.ui-color-area[data-disabled="true"] {
  opacity: var(--ui-color-area-disabled-opacity);
}

.ui-color-area--disabled .ui-color-area__cell,
.ui-color-area[data-disabled="true"] .ui-color-area__cell,
.ui-color-area--disabled .ui-color-area__axis-input,
.ui-color-area[data-disabled="true"] .ui-color-area__axis-input {
  cursor: not-allowed;
}

.ui-color-area--custom-class,
.ui-color-area[data-custom-class="true"],
.ui-color-area[data-class-source="custom"] {
  --ui-color-area-custom-class: 1;
}

@media (prefers-reduced-motion: reduce) {
  .ui-color-area {
    transition: none;
  }
}
"#;
