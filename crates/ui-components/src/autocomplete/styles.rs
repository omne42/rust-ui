pub const CSS: &str = r#"
.ui-autocomplete {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-autocomplete__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-autocomplete__control {
  position: relative;
  display: flex;
  align-items: stretch;
}

.ui-autocomplete__input {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);

  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;
  outline: none;
}

.ui-autocomplete--focus-visible .ui-autocomplete__input {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-autocomplete--invalid .ui-autocomplete__input {
  border-color: var(--ui-danger);
}

.ui-autocomplete--invalid.ui-autocomplete--focus-visible .ui-autocomplete__input {
  outline-color: var(--ui-danger);
}

.ui-autocomplete__description,
.ui-autocomplete__error {
  font-size: 12px;
  line-height: 1.3;
}

.ui-autocomplete__description {
  color: var(--ui-fg-muted);
}

.ui-autocomplete__error {
  color: var(--ui-danger);
}

.ui-autocomplete__panel {
  position: fixed;
  top: var(--ui-popover-top, 0px);
  left: var(--ui-popover-left, 0px);
  width: var(--ui-popover-anchor-width, 240px);
  max-width: calc(100vw - 16px);
  z-index: 1000;

  padding: 0;
  background: var(--ui-bg);
  color: var(--ui-fg);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  box-shadow: var(--ui-shadow-md);

  --ui-popover-opacity: 0;
  --ui-popover-scale: 0.98;
  --ui-popover-y: 6px;

  opacity: var(--ui-popover-opacity);
  transform: translateY(var(--ui-popover-y)) scale(var(--ui-popover-scale));
  will-change: transform, opacity;
}

.ui-autocomplete__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-autocomplete__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-autocomplete__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-autocomplete__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}

.ui-autocomplete__listbox {
  padding: 4px;
}

.ui-autocomplete__options {
  position: relative;
}

.ui-autocomplete__option {
  position: relative;
  padding: var(--ui-space-sm) var(--ui-space-md);
  border-radius: var(--ui-radius-md);
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-autocomplete__option[data-disabled=\"true\"] {
  cursor: not-allowed;
  opacity: 0.5;
}

.ui-autocomplete__option[data-selected=\"true\"] {
  font-weight: 600;
}

.ui-autocomplete__option:focus-visible {
  outline: none;
}
"#;
