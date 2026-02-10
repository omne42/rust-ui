pub const CSS: &str = r#"
.ui-accordion {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-accordion[data-motion-source="custom"],
.ui-accordion[data-custom-motion="true"] {
  --ui-accordion-custom-motion: 1;
}

.ui-accordion__item {
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  overflow: hidden;
}

.ui-accordion__trigger {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-sm) var(--ui-space-md);

  background: transparent;
  color: var(--ui-fg);
  border: none;

  font-size: 14px;
  font-weight: 600;
  line-height: 1.2;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-accordion__trigger:not(:disabled) {
  cursor: pointer;
}

.ui-accordion__trigger:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-accordion__trigger[data-hovered="true"]:not(:disabled) {
  background: var(--ui-bg-muted);
}

.ui-accordion__trigger--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: -3px;
}

.ui-accordion__indicator {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;

  transform: rotate(var(--ui-accordion-indicator-rotation, 0deg));
  transform-origin: center;
}

.ui-accordion__panel {
  height: var(--ui-accordion-panel-height, auto);
  opacity: var(--ui-accordion-panel-opacity, 1);
  transform: translateY(var(--ui-accordion-panel-y, 0px));
  overflow: hidden;
  will-change: height, opacity, transform;
}

.ui-accordion__panel-surface {
  border-top: 1px solid var(--ui-border);
  padding: var(--ui-space-md);
  color: var(--ui-fg);
  background: var(--ui-bg);
}
"#;
