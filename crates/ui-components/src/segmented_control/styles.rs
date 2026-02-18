pub const CSS: &str = r#"
.ui-segmented-control {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-segmented-control__label {
  font-size: var(--ui-font-size-100);
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-segmented-control__options {
  position: relative;
  display: inline-flex;
  padding: var(--ui-space-2xs);
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
  top: var(--ui-space-2xs);
  left: var(--ui-space-2xs);
  width: var(--ui-segmented-control-indicator-w, 0px);
  height: var(--ui-segmented-control-indicator-h, 0px);
  transform:
    translateX(var(--ui-segmented-control-indicator-x, 0px))
    translateY(var(--ui-segmented-control-indicator-y, 0px));
  opacity: var(--ui-segmented-control-indicator-o, 0);
  background: var(--ui-bg);
  border-radius: calc(var(--ui-radius-lg) - var(--ui-space-2xs));
  box-shadow: var(--ui-shadow-sm);
  pointer-events: none;
  will-change: transform, width, height, opacity;
}

.ui-segmented-control__option {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: calc((var(--ui-space-sm) + var(--ui-space-2xs)) / 2);
  border: 0;
  background: transparent;
  color: var(--ui-fg);
  border-radius: calc(var(--ui-radius-lg) - var(--ui-space-2xs));
  font-weight: 600;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  min-width: calc(var(--ui-component-height-100) + var(--ui-space-md));
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
  outline: calc(var(--ui-space-2xs) - (var(--ui-space-3xs) / 2)) solid var(--ui-focus-ring);
  outline-offset: var(--ui-space-3xs);
}

.ui-segmented-control--size-default .ui-segmented-control__option {
  height: calc(var(--ui-component-height-100) + (var(--ui-space-2xs) / 2));
  padding-inline: calc(var(--ui-space-md) + (var(--ui-space-2xs) / 2));
  font-size: var(--ui-font-size-150);
}

.ui-segmented-control--size-sm .ui-segmented-control__option {
  height: calc(var(--ui-component-height-100) - (var(--ui-space-2xs) / 2));
  padding-inline: var(--ui-space-md);
  font-size: var(--ui-font-size-100);
}

.ui-segmented-control--size-lg .ui-segmented-control__option {
  height: calc(var(--ui-component-height-100) + var(--ui-space-sm) - (var(--ui-space-2xs) / 2));
  padding-inline: calc(var(--ui-space-md) + var(--ui-space-sm) - (var(--ui-space-2xs) / 2));
  font-size: var(--ui-font-size-200);
}
"#;
