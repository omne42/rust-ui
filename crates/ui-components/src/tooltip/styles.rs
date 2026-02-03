pub const CSS: &str = r#"
.ui-tooltip {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-tooltip__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: help;
  -webkit-tap-highlight-color: transparent;
}

.ui-tooltip__trigger:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 3px;
  border-radius: var(--ui-radius-md);
}

.ui-tooltip__panel {
  position: absolute;
  top: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%) scale(var(--ui-tooltip-scale, 0.98));
  transform-origin: top center;

  opacity: var(--ui-tooltip-opacity, 0);
  will-change: transform, opacity;

  pointer-events: none;
  z-index: 30;

  padding: 8px 10px;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);

  font-size: 12px;
  line-height: 1.2;
  max-width: 280px;
}
"#;
