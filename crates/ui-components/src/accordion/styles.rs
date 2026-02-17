pub const CSS: &str = r#"
.ui-accordion {
  display: flex;
  flex-direction: column;
  gap: var(--ui-accordion-item-gap, var(--ui-space-xs));
  --ui-accordion-item-gap: var(--ui-space-xs);
  --ui-accordion-item-bg: transparent;
  --ui-accordion-item-border: var(--ui-layout-divider, var(--ui-border));
  --ui-accordion-item-border-width: 0px;
  --ui-accordion-item-shadow: none;
  --ui-accordion-trigger-color: var(--ui-layout-foreground, var(--ui-fg));
  --ui-accordion-trigger-hover-bg: var(--ui-layout-content-2, var(--ui-bg-muted));
  --ui-accordion-focus-ring: var(--ui-layout-focus, var(--ui-focus-ring));
  --ui-accordion-panel-bg: transparent;
  --ui-accordion-panel-color: var(--ui-layout-foreground, var(--ui-fg));
  --ui-accordion-panel-border-width: 1px;
}

.ui-accordion[data-variant="shadow"] {
  --ui-accordion-item-bg: var(--ui-layout-content-1, var(--ui-bg));
  --ui-accordion-item-border-width: 1px;
  --ui-accordion-item-shadow: var(--ui-shadow-sm);
  --ui-accordion-panel-bg: var(--ui-layout-content-1, var(--ui-bg));
}

.ui-accordion[data-variant="bordered"] {
  --ui-accordion-item-bg: var(--ui-layout-content-1, var(--ui-bg));
  --ui-accordion-item-border-width: 1px;
  --ui-accordion-item-shadow: none;
  --ui-accordion-panel-bg: var(--ui-layout-content-1, var(--ui-bg));
}

.ui-accordion[data-variant="splitted"] {
  --ui-accordion-item-gap: var(--ui-space-sm);
  --ui-accordion-item-bg: var(--ui-layout-content-1, var(--ui-bg));
  --ui-accordion-item-border-width: 1px;
  --ui-accordion-item-shadow: var(--ui-shadow-xs);
  --ui-accordion-panel-bg: var(--ui-layout-content-1, var(--ui-bg));
}

.ui-accordion[data-motion-source="custom"],
.ui-accordion[data-custom-motion="true"] {
  --ui-accordion-custom-motion: 1;
}

.ui-accordion__item {
  border-radius: var(--ui-radius-md);
  border: var(--ui-accordion-item-border-width) solid var(--ui-accordion-item-border);
  background: var(--ui-accordion-item-bg);
  box-shadow: var(--ui-accordion-item-shadow);
  overflow: hidden;
}

.ui-accordion__trigger {
  width: 100%;
  min-height: var(--ui-component-height-100);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-sm) var(--ui-space-md);

  background: transparent;
  color: var(--ui-accordion-trigger-color);
  border: none;

  font-size: var(--ui-font-size-200);
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
  background: var(--ui-accordion-trigger-hover-bg);
}

.ui-accordion__trigger--focus-visible {
  outline: 3px solid var(--ui-accordion-focus-ring);
  outline-offset: -3px;
}

.ui-accordion__indicator {
  width: calc(var(--ui-component-height-100) * 0.5625);
  height: calc(var(--ui-component-height-100) * 0.5625);
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
  border-top: var(--ui-accordion-panel-border-width) solid var(--ui-accordion-item-border);
  padding: var(--ui-space-md);
  color: var(--ui-accordion-panel-color);
  background: var(--ui-accordion-panel-bg);
}

.ui-accordion__debug {
  margin-top: var(--ui-space-sm);
  border: 1px dashed var(--ui-accordion-item-border);
  border-radius: var(--ui-radius-sm);
  padding: var(--ui-space-xs) var(--ui-space-sm);
  background: var(--ui-accordion-trigger-hover-bg);
  color: var(--ui-accordion-trigger-color);
  font-size: var(--ui-font-size-100);
}

.ui-accordion__debug-list {
  margin: var(--ui-space-xs) 0 0;
  padding: 0;
  list-style-type: none;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-2xs);
}

.ui-accordion__debug-item {
  display: flex;
  align-items: baseline;
  gap: var(--ui-space-xs);
}

.ui-accordion__debug-replay {
  cursor: pointer;
}
"#;
