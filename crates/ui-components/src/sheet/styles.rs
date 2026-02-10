pub const CSS: &str = r#"
.ui-sheet {
  position: fixed;
  inset: 0;
  z-index: 1000;
}

.ui-sheet[data-motion-source="custom"],
.ui-sheet[data-custom-motion="true"],
.ui-sheet--custom-motion {
  --ui-sheet-custom-motion: 1;
}

.ui-sheet[data-placement-source="custom"],
.ui-sheet--custom-placement {
  --ui-sheet-placement-source: custom;
}

.ui-sheet[data-dismiss-source="custom"],
.ui-sheet--custom-dismiss {
  --ui-sheet-dismiss-source: custom;
}

.ui-sheet[data-keyboard-dismiss-source="custom"],
.ui-sheet--custom-keyboard-dismiss {
  --ui-sheet-keyboard-dismiss-source: custom;
}

.ui-sheet[data-aria-labelledby-source="custom"] {
  --ui-sheet-aria-labelledby-source: custom;
}

.ui-sheet[data-aria-describedby-source="custom"] {
  --ui-sheet-aria-describedby-source: custom;
}

.ui-sheet[data-exit-source="custom"],
.ui-sheet[data-custom-exit="true"],
.ui-sheet--custom-exit {
  --ui-sheet-exit-source: custom;
}

.ui-sheet[data-state="open"],
.ui-sheet[data-open="true"],
.ui-sheet[data-state="closed"],
.ui-sheet[data-closed="true"] {
  pointer-events: auto;
}

.ui-sheet[data-dismissable="true"] .ui-sheet__backdrop {
  cursor: pointer;
}

.ui-sheet[data-keyboard-dismiss-disabled="true"] .ui-sheet__panel {
  outline: 1px dashed color-mix(in oklab, var(--ui-border) 72%, transparent);
}

.ui-sheet__backdrop {
  position: absolute;
  inset: 0;
  background: color-mix(in oklch, var(--ui-fg) 24%, transparent);
  opacity: var(--ui-sheet-backdrop-opacity, 0);
}

.ui-sheet__backdrop[data-state="backdrop"] {
  --ui-sheet-backdrop-state: 1;
}

.ui-sheet__panel {
  position: absolute;
  background: var(--ui-bg);
  color: var(--ui-fg);
  border: 1px solid var(--ui-border);
  box-shadow: var(--ui-shadow-lg, var(--ui-shadow-sm));
  opacity: var(--ui-sheet-panel-opacity, 0);
  transform: translate3d(var(--ui-sheet-panel-x, 0px), var(--ui-sheet-panel-y, 0px), 0);
  will-change: transform, opacity;
}

.ui-sheet__panel[data-state="panel"] {
  --ui-sheet-panel-state: 1;
}

.ui-sheet--placement-bottom .ui-sheet__panel {
  left: 0;
  right: 0;
  bottom: 0;
  max-height: 90vh;
  border-top-left-radius: var(--ui-radius-lg);
  border-top-right-radius: var(--ui-radius-lg);
  padding: var(--ui-space-lg);
}

.ui-sheet--placement-left .ui-sheet__panel,
.ui-sheet--placement-right .ui-sheet__panel {
  top: 0;
  bottom: 0;
  width: min(420px, 92vw);
  padding: var(--ui-space-lg);
}

.ui-sheet--placement-left .ui-sheet__panel {
  left: 0;
  border-top-right-radius: var(--ui-radius-lg);
  border-bottom-right-radius: var(--ui-radius-lg);
}

.ui-sheet--placement-right .ui-sheet__panel {
  right: 0;
  border-top-left-radius: var(--ui-radius-lg);
  border-bottom-left-radius: var(--ui-radius-lg);
}
"#;
