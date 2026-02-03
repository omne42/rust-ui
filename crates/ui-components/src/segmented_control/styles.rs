pub const CSS: &str = r#"
.ui-segmented-control {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-segmented-control__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-segmented-control__options {
  position: relative;
  display: inline-flex;
  padding: 4px;
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  box-shadow: var(--ui-shadow-sm);
}

.ui-segmented-control--horizontal .ui-segmented-control__options {
  flex-direction: row;
  align-items: center;
}

.ui-segmented-control--vertical .ui-segmented-control__options {
  flex-direction: column;
  align-items: stretch;
}

.ui-segmented-control__indicator {
  position: absolute;
  top: 4px;
  bottom: 4px;
  left: 4px;
  width: var(--ui-segmented-control-indicator-w, 0px);
  transform: translateX(var(--ui-segmented-control-indicator-x, 0px));
  opacity: var(--ui-segmented-control-indicator-o, 0);
  background: var(--ui-bg);
  border-radius: calc(var(--ui-radius-lg) - 4px);
  box-shadow: var(--ui-shadow-sm);
  pointer-events: none;
  will-change: transform, width, opacity;
}

.ui-segmented-control__option {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  border: 0;
  background: transparent;
  color: var(--ui-fg);
  border-radius: calc(var(--ui-radius-lg) - 4px);
  font-weight: 600;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  min-width: 44px;
}

.ui-segmented-control__option:not(:disabled) {
  cursor: pointer;
}

.ui-segmented-control__option:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-segmented-control__option[data-hovered="true"]:not(:disabled) {
  color: color-mix(in oklch, var(--ui-fg) 92%, var(--ui-fg-muted));
}

.ui-segmented-control__option--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-segmented-control--size-default .ui-segmented-control__option {
  height: 34px;
  padding: 0 14px;
  font-size: 13px;
}

.ui-segmented-control--size-sm .ui-segmented-control__option {
  height: 30px;
  padding: 0 12px;
  font-size: 12px;
}

.ui-segmented-control--size-lg .ui-segmented-control__option {
  height: 38px;
  padding: 0 18px;
  font-size: 14px;
}
"#;
