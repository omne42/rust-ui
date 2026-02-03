pub const CSS: &str = r#"
.ui-sheet {
  position: fixed;
  inset: 0;
  z-index: 1000;
}

.ui-sheet__backdrop {
  position: absolute;
  inset: 0;
  background: color-mix(in oklch, var(--ui-fg) 24%, transparent);
  opacity: var(--ui-sheet-backdrop-opacity, 0);
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
