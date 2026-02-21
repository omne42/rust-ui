pub const CSS: &str = r#"
.ui-disclosure {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-disclosure[data-motion-source="custom"],
.ui-disclosure[data-custom-motion="true"] {
  --ui-disclosure-custom-motion: 1;
}

.ui-disclosure__trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  width: 100%;

  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm))
    var(--ui-space-md, var(--ui-fallback-space-md));

  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));

  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  font-weight: var(--ui-font-weight-semibold, 600);
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-disclosure__trigger:not(:disabled) {
  cursor: pointer;
}

.ui-disclosure__trigger:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-disclosure__trigger[data-hovered="true"]:not(:disabled) {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
}

.ui-disclosure__trigger--focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
}

.ui-disclosure__indicator {
  width: var(--ui-icon-size-200, var(--ui-fallback-icon-size-200));
  height: var(--ui-icon-size-200, var(--ui-fallback-icon-size-200));
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;

  transform: rotate(var(--ui-disclosure-indicator-rotation, 0deg));
  transform-origin: center;
}

.ui-disclosure__panel {
  height: var(--ui-disclosure-panel-height, auto);
  opacity: var(--ui-disclosure-panel-opacity, 1);
  transform: translateY(var(--ui-disclosure-panel-y, 0px));
  overflow: hidden;
  will-change: height, opacity, transform;

  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-disclosure__panel-surface {
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
}
"#;
