pub const CSS: &str = r#"
.ui-color-thumb {
  --ui-color-thumb-handle-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration, 180ms)
  );
  --ui-color-thumb-loupe-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration, 180ms)
  );
  --ui-color-thumb-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing, ease)
  );
  position: absolute;
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 1;
}

.ui-color-thumb--x-start {
  left: var(--ui-color-thumb-x-start, var(--ui-fallback-color-thumb-x-start));
}

.ui-color-thumb--x-center {
  left: var(--ui-color-thumb-x-center, var(--ui-fallback-color-thumb-x-center));
}

.ui-color-thumb--x-end {
  left: var(--ui-color-thumb-x-end, var(--ui-fallback-color-thumb-x-end));
}

.ui-color-thumb--y-start {
  top: var(--ui-color-thumb-y-start, var(--ui-fallback-color-thumb-y-start));
}

.ui-color-thumb--y-center {
  top: var(--ui-color-thumb-y-center, var(--ui-fallback-color-thumb-y-center));
}

.ui-color-thumb--y-end {
  top: var(--ui-color-thumb-y-end, var(--ui-fallback-color-thumb-y-end));
}

.ui-color-thumb__handle {
  pointer-events: auto;
  inline-size: var(--ui-color-thumb-handle-size, var(--ui-fallback-color-thumb-handle-size));
  block-size: var(--ui-color-thumb-handle-size, var(--ui-fallback-color-thumb-handle-size));
  border-radius: var(--ui-color-thumb-radius-full, var(--ui-fallback-color-thumb-radius-full));
  border:
    var(
      --ui-color-thumb-handle-border-width,
      var(--ui-fallback-color-thumb-handle-border-width)
    )
    solid var(--ui-bg, var(--ui-fallback-bg));
  box-shadow:
    0 0 0 1px color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)), transparent 56%),
    var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  background: color-mix(
    in oklch,
    var(--ui-bg, var(--ui-fallback-bg)),
    var(--ui-fg, var(--ui-fallback-fg)) 4%
  );
  transition:
    transform var(--ui-color-thumb-handle-duration) var(--ui-color-thumb-motion-easing),
    box-shadow var(--ui-color-thumb-handle-duration) var(--ui-color-thumb-motion-easing),
    opacity var(--ui-color-thumb-loupe-duration) var(--ui-color-thumb-motion-easing);
}

.ui-color-thumb__fill,
.ui-color-thumb__loupe-fill {
  display: block;
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
}

.ui-color-thumb__swatch.ui-color-swatch,
.ui-color-thumb__loupe-swatch.ui-color-swatch {
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
  min-inline-size: 0;
  min-block-size: 0;
}

.ui-color-thumb__loupe {
  position: absolute;
  left: 50%;
  bottom: calc(100% + var(--ui-space-xs, var(--ui-fallback-space-xs)));
  transform: translateX(-50%)
    translateY(
      var(
        --ui-color-thumb-loupe-hidden-offset,
        var(--ui-fallback-color-thumb-loupe-hidden-offset)
      )
    )
    scale(var(--ui-color-thumb-loupe-hidden-scale, var(--ui-fallback-color-thumb-loupe-hidden-scale)));
  inline-size: var(--ui-color-thumb-loupe-size, var(--ui-fallback-color-thumb-loupe-size));
  block-size: var(--ui-color-thumb-loupe-size, var(--ui-fallback-color-thumb-loupe-size));
  border-radius: var(--ui-color-thumb-radius-full, var(--ui-fallback-color-thumb-radius-full));
  padding: var(--ui-color-thumb-loupe-padding, var(--ui-fallback-color-thumb-loupe-padding));
  background: var(--ui-bg, var(--ui-fallback-bg));
  border:
    var(--ui-color-thumb-loupe-border-width, var(--ui-fallback-color-thumb-loupe-border-width))
    solid color-mix(in oklch, var(--ui-border, var(--ui-fallback-border)), transparent 24%);
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  opacity: 0;
  visibility: hidden;
  pointer-events: none;
  transition:
    opacity var(--ui-color-thumb-loupe-duration) var(--ui-color-thumb-motion-easing),
    transform var(--ui-color-thumb-loupe-duration) var(--ui-color-thumb-motion-easing),
    visibility 0s linear var(--ui-color-thumb-loupe-duration);
}

.ui-color-thumb[data-loupe-visible="true"] .ui-color-thumb__loupe,
.ui-color-thumb--dragging .ui-color-thumb__loupe {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0) scale(1);
  transition-delay: 0s;
}

.ui-color-thumb--focused .ui-color-thumb__handle,
.ui-color-thumb[data-focused="true"] .ui-color-thumb__handle {
  transform: scale(1.12);
  box-shadow:
    0 0 0 2px color-mix(in oklch, var(--ui-accent, var(--ui-fallback-accent)), transparent 72%),
    0 0 0 1px color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)), transparent 56%),
    var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-color-thumb--dragging .ui-color-thumb__handle,
.ui-color-thumb[data-dragging="true"] .ui-color-thumb__handle {
  transform: scale(1.18);
}

.ui-color-thumb--disabled,
.ui-color-thumb[data-disabled="true"] {
  opacity: var(--ui-color-thumb-disabled-opacity, var(--ui-fallback-color-thumb-disabled-opacity));
}

.ui-color-thumb--disabled .ui-color-thumb__handle,
.ui-color-thumb[data-disabled="true"] .ui-color-thumb__handle {
  pointer-events: none;
}

.ui-color-thumb--custom-class,
.ui-color-thumb[data-custom-class="true"] {
  isolation: isolate;
}
"#;
