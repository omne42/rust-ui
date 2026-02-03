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

.ui-overlay__backdrop {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, calc(0.35 * var(--ui-overlay-backdrop-opacity)));
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
"#;
