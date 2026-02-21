pub const CSS: &str = r#"
.ui-action-bar {
  --ui-action-bar-translate-y: var(--ui-action-bar-translate-y-initial, var(--ui-fallback-action-bar-translate-y-initial));
  --ui-action-bar-opacity: var(--ui-action-bar-opacity-initial, var(--ui-fallback-action-bar-opacity-initial));

  position: fixed;
  left: 50%;
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));
  display: inline-flex;
  align-items: center;
  gap: clamp(
    var(--ui-space-xs, var(--ui-fallback-space-xs)),
    1.5vw,
    var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
  min-height: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
  max-width: min(
    var(--ui-action-bar-max-width, var(--ui-fallback-action-bar-max-width)),
    calc(
      100vw - clamp(
          var(--ui-space-md, var(--ui-fallback-space-md)),
          4vw,
          var(--ui-space-xl, var(--ui-fallback-space-xl))
        ) * 2
    )
  );
  padding: clamp(
      var(--ui-space-xs, var(--ui-fallback-space-xs)),
      1.5vw,
      var(--ui-space-sm, var(--ui-fallback-space-sm))
    )
    clamp(
      var(--ui-space-sm, var(--ui-fallback-space-sm)),
      2vw,
      var(--ui-space-md, var(--ui-fallback-space-md))
    );
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 92%,
    var(--ui-bg, var(--ui-fallback-bg)) 8%
  );
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  transform: translateX(-50%)
    translateY(var(--ui-action-bar-translate-y, var(--ui-fallback-action-bar-translate-y-initial)));
  opacity: var(--ui-action-bar-opacity, var(--ui-fallback-action-bar-opacity-initial));
  pointer-events: auto;
}

.ui-action-bar--position-bottom,
.ui-action-bar[data-position="bottom"] {
  bottom: calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) + env(safe-area-inset-bottom));
}

.ui-action-bar--position-top,
.ui-action-bar[data-position="top"] {
  top: calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) + env(safe-area-inset-top));
}

.ui-action-bar--state-hidden,
.ui-action-bar[data-state="hidden"],
.ui-action-bar[data-hidden="true"] {
  pointer-events: none;
}

.ui-action-bar__selection {
  display: inline-flex;
  align-items: baseline;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  margin: 0;
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: clamp(
    var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size)),
    1.4vw,
    var(--ui-font-size-150, var(--ui-fallback-font-size-150))
  );
  font-weight: 600;
  line-height: clamp(
    var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height)),
    1.8vw,
    var(--ui-line-height-150, var(--ui-fallback-line-height-150))
  );
  white-space: nowrap;
}

.ui-action-bar__selection-count {
  font-variant-numeric: tabular-nums;
  opacity: 0.92;
}

.ui-action-bar__actions {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-action-bar__clear {
  border: 0;
  padding: 0;
  color: var(--ui-accent, var(--ui-fallback-accent));
  background: transparent;
  font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
  text-decoration: underline;
  text-underline-offset: var(
    --ui-action-bar-clear-underline-offset,
    var(--ui-fallback-action-bar-clear-underline-offset)
  );
  cursor: pointer;
}

.ui-action-bar__clear:hover {
  color: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 84%,
    var(--ui-common-white, var(--ui-fallback-common-white)) 16%
  );
}

.ui-action-bar__clear:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-action-bar--selection-empty .ui-action-bar__selection,
.ui-action-bar[data-selection="empty"] .ui-action-bar__selection {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-action-bar--selection-single .ui-action-bar__selection,
.ui-action-bar[data-selection="single"] .ui-action-bar__selection {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 94%,
    var(--ui-accent, var(--ui-fallback-accent)) 6%
  );
}

.ui-action-bar--selection-multiple .ui-action-bar__selection,
.ui-action-bar[data-selection="multiple"] .ui-action-bar__selection {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 86%,
    var(--ui-accent, var(--ui-fallback-accent)) 14%
  );
}

.ui-action-bar--clearable,
.ui-action-bar[data-has-clear="true"] {
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-action-bar--label-custom,
.ui-action-bar[data-label-source="custom"] {
  border: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    var(--ui-border, var(--ui-fallback-border));
}

.ui-action-bar--selection-custom,
.ui-action-bar[data-selection-source="custom"] {
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 32%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 68%
  );
}

.ui-action-bar--clear-label-custom,
.ui-action-bar[data-clear-label-source="custom"] {
  box-shadow: var(
    --ui-action-bar-clear-label-custom-shadow,
    var(--ui-fallback-action-bar-clear-label-custom-shadow)
  );
}

.ui-action-bar--motion-custom,
.ui-action-bar[data-motion-source="custom"] {
  backdrop-filter: blur(var(--ui-underlay-backdrop-blur, var(--ui-fallback-underlay-backdrop-blur)));
}

.ui-action-bar--custom-class,
.ui-action-bar[data-custom-class="true"] {
  border-width: var(
    --ui-action-bar-emphasis-border-width,
    var(--ui-fallback-action-bar-emphasis-border-width)
  );
}
"#;
