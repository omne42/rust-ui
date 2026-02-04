pub const CSS: &str = r#"
.ui-popover {
  position: fixed;
  inset: 0;
  z-index: 1000;
}

.ui-popover__panel {
  position: fixed;
  top: var(--ui-popover-top, 0px);
  left: var(--ui-popover-left, 0px);
  min-width: max(240px, var(--ui-popover-anchor-width, 0px));
  padding: var(--ui-space-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  box-shadow: var(--ui-shadow-md);

  --ui-popover-opacity: 0;
  --ui-popover-scale: 0.98;
  --ui-popover-y: 6px;

  opacity: var(--ui-popover-opacity);
  transform: translateY(var(--ui-popover-y)) scale(var(--ui-popover-scale));
  will-change: transform, opacity;
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
