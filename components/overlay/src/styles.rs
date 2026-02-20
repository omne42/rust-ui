pub const CSS: &str = r#"
.ui-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;

  --ui-overlay-backdrop-opacity: 0;
  --ui-overlay-panel-opacity: 0;
  --ui-overlay-panel-scale: 0.96;
  --ui-overlay-panel-y: 8px;
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
  outline: 1px dashed color-mix(in oklab, var(--ui-border) 72%, transparent);
}

.ui-overlay__backdrop {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, calc(0.35 * var(--ui-overlay-backdrop-opacity)));
}

.ui-overlay__backdrop[data-state="backdrop"] {
  --ui-overlay-backdrop: 1;
}

.ui-overlay__panel {
  position: relative;
  z-index: 1;
  background: var(--ui-bg);
  color: var(--ui-fg);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  padding: var(--ui-space-lg);
  min-width: 280px;
  max-width: 640px;

  box-shadow: var(--ui-shadow-md);
  opacity: var(--ui-overlay-panel-opacity);
  transform: translateY(var(--ui-overlay-panel-y)) scale(var(--ui-overlay-panel-scale));
  transform-origin: center;
  will-change: transform, opacity;
}

.ui-overlay__panel[data-state="panel"] {
  --ui-overlay-panel: 1;
}
"#;
