pub const CSS: &str = r#"
.ui-drop-zone {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-drop-zone[data-motion-source="custom"],
.ui-drop-zone[data-custom-motion="true"] {
  --ui-drop-zone-custom-motion: 1;
}

.ui-drop-zone__label {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 600;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-drop-zone__zone {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: var(--ui-drop-zone-min-height, var(--ui-fallback-drop-zone-min-height));
  padding: var(--ui-space-lg, var(--ui-fallback-space-lg));
  position: relative;
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  border:
    var(--ui-drop-zone-border-width, var(--ui-fallback-drop-zone-border-width))
    dashed
    color-mix(
      in oklch,
      var(--ui-border, var(--ui-fallback-border)) 80%,
      var(--ui-fg-muted, var(--ui-fallback-fg-muted))
    );
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  --ui-drop-zone-scale: 1;
  --ui-drop-zone-highlight: 0;

  transform: scale(var(--ui-drop-zone-scale));
  transform-origin: center;
  will-change: transform;
}

.ui-drop-zone__zone::before {
  content: "";
  position: absolute;
  inset: 0;
  background: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
  opacity: var(--ui-drop-zone-highlight);
  border-radius: inherit;
  pointer-events: none;
}

.ui-drop-zone__zone[data-hovered="true"],
.ui-drop-zone__zone[data-drop-target="true"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-drop-zone__zone[data-drop-target="true"] {
  border-color:
    color-mix(
      in oklch,
      var(--ui-accent, var(--ui-fallback-accent)) 60%,
      var(--ui-border, var(--ui-fallback-border))
    );
}

.ui-drop-zone__zone[data-disabled="true"] {
  opacity: var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity));
  pointer-events: none;
}

.ui-drop-zone__zone[data-focus-visible="true"] {
  outline:
    var(--ui-drop-zone-focus-outline-width, var(--ui-fallback-drop-zone-focus-outline-width))
    solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-drop-zone-focus-outline-offset, var(--ui-fallback-drop-zone-focus-outline-offset));
}

.ui-drop-zone__button {
  position: absolute;
  width: var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size));
  height: var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size));
  padding: 0;
  margin: calc(-1 * var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size)));
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
  pointer-events: none;
}
"#;
