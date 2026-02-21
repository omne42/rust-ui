pub const CSS: &str = r#"
.ui-popover {
  position: fixed;
  inset: 0;
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));
}

.ui-popover[data-motion-source="custom"],
.ui-popover[data-custom-motion="true"],
.ui-popover--custom-motion {
  --ui-popover-custom-motion: 1;
}

.ui-popover[data-placement-source="custom"],
.ui-popover--custom-placement {
  --ui-popover-placement-source: custom;
}

.ui-popover--custom-modal,
.ui-popover[data-modal-source="custom"],
.ui-popover[data-modal="non-modal"],
.ui-popover[data-non-modal="true"],
.ui-popover[data-custom-modal="true"],
.ui-popover--non-modal {
  --ui-popover-modal-source: custom;
}

.ui-popover[data-class-source="custom"],
.ui-popover--custom-class {
  --ui-popover-class-source: custom;
}

.ui-popover[data-exit-source="custom"],
.ui-popover[data-custom-exit="true"],
.ui-popover--custom-exit {
  --ui-popover-exit-source: custom;
}

.ui-popover[data-state="open"],
.ui-popover[data-open="true"] {
  pointer-events: auto;
}

.ui-popover[data-state="closed"],
.ui-popover[data-closed="true"] {
  pointer-events: auto;
}

.ui-popover__panel {
  position: fixed;
  top: var(--ui-popover-top, var(--ui-fallback-min-inline-size-none));
  left: var(--ui-popover-left, var(--ui-fallback-min-inline-size-none));
  min-width: max(
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)),
    var(--ui-popover-anchor-width, var(--ui-fallback-min-inline-size-none))
  );
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));

  --ui-popover-opacity: 0;
  --ui-popover-scale: var(
    --ui-overlay-enter-scale,
    var(--ui-fallback-overlay-enter-scale)
  );
  --ui-popover-y: var(
    --ui-overlay-enter-offset-y,
    var(--ui-fallback-overlay-enter-offset-y)
  );

  opacity: var(--ui-popover-opacity);
  transform: translateY(var(--ui-popover-y)) scale(var(--ui-popover-scale));
  will-change: transform, opacity;
}

.ui-popover__panel[data-state="panel"] {
  --ui-popover-panel: 1;
}

.ui-popover__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-popover__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-popover__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-popover__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}
"#;
