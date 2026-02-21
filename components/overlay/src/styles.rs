pub const CSS: &str = r#"
.ui-overlay {
  position: fixed;
  inset: 0;
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));
  display: flex;
  align-items: center;
  justify-content: center;
  padding: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg))
      + var(--ui-space-sm, var(--ui-fallback-space-sm))
  );

  --ui-overlay-backdrop-opacity: 0;
  --ui-overlay-panel-opacity: 0;
  --ui-overlay-panel-scale: var(
    --ui-overlay-enter-scale,
    var(--ui-fallback-overlay-enter-scale)
  );
  --ui-overlay-panel-y: var(
    --ui-overlay-enter-offset-y,
    var(--ui-fallback-overlay-enter-offset-y)
  );
}

.ui-overlay[data-motion-source="custom"],
.ui-overlay[data-custom-motion="true"],
.ui-overlay--custom-motion {
  --ui-overlay-custom-motion: 1;
}

.ui-overlay[data-class-source="custom"],
.ui-overlay--custom-class {
  --ui-overlay-class-source: custom;
}

.ui-overlay--custom-role,
.ui-overlay[data-role-source="custom"],
.ui-overlay[data-custom-role="true"] {
  --ui-overlay-role-source: custom;
}

.ui-overlay--custom-aria-labelledby,
.ui-overlay[data-aria-labelledby-source="custom"],
.ui-overlay[data-custom-aria-labelledby="true"] {
  --ui-overlay-aria-labelledby-source: custom;
}

.ui-overlay--custom-aria-describedby,
.ui-overlay[data-aria-describedby-source="custom"],
.ui-overlay[data-custom-aria-describedby="true"] {
  --ui-overlay-aria-describedby-source: custom;
}

.ui-overlay[data-dismiss-source="custom"],
.ui-overlay--custom-dismiss {
  --ui-overlay-dismiss-source: custom;
}

.ui-overlay[data-keyboard-dismiss-source="custom"],
.ui-overlay--custom-keyboard-dismiss {
  --ui-overlay-keyboard-dismiss-source: custom;
}

.ui-overlay[data-exit-source="custom"],
.ui-overlay[data-custom-exit="true"],
.ui-overlay--custom-exit {
  --ui-overlay-exit-source: custom;
}

.ui-overlay[data-state="open"],
.ui-overlay[data-open="true"] {
  pointer-events: auto;
}

.ui-overlay[data-state="closed"],
.ui-overlay[data-closed="true"] {
  pointer-events: none;
}

.ui-overlay[data-dismissable="true"] .ui-overlay__backdrop {
  cursor: pointer;
}

.ui-overlay[data-keyboard-dismiss-disabled="true"] .ui-overlay__panel {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 72%,
      transparent
    );
}

.ui-overlay__backdrop {
  position: absolute;
  inset: 0;
  background: color-mix(
    in oklch,
    var(--ui-fg, var(--ui-fallback-fg)) 24%,
    transparent
  );
  opacity: var(--ui-overlay-backdrop-opacity);
}

.ui-overlay__backdrop[data-state="backdrop"] {
  --ui-overlay-backdrop: 1;
}

.ui-overlay__panel {
  position: relative;
  z-index: 1;
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  padding: var(--ui-space-lg, var(--ui-fallback-space-lg));
  min-width: var(
    --ui-overlay-panel-min-width,
    var(--ui-fallback-overlay-panel-min-width)
  );
  max-width: calc(
    100vw
      - var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))
      * 2
  );

  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  opacity: var(--ui-overlay-panel-opacity);
  transform: translateY(var(--ui-overlay-panel-y)) scale(var(--ui-overlay-panel-scale));
  transform-origin: center;
  will-change: transform, opacity;
}

.ui-overlay__panel[data-state="panel"] {
  --ui-overlay-panel: 1;
}
"#;
