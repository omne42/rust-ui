pub const CSS: &str = r#"
.ui-combo-box {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-combo-box[data-motion-source="custom"],
.ui-combo-box[data-custom-motion="true"] {
  --ui-combo-box-custom-motion: 1;
}

.ui-combo-box--empty .ui-combo-box__trigger {
  opacity: 0.72;
}

.ui-combo-box--controlled .ui-combo-box__trigger {
  box-shadow: inset 0 0 0 1px var(--ui-border);
}

.ui-combo-box--has-disabled-options .ui-combo-box__listbox {
  --ui-combo-box-has-disabled-options: 1;
}

.ui-combo-box__field {
  position: relative;
}

.ui-combo-box__label {
  font-size: 14px;
  line-height: 1.2;
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-combo-box__control {
  position: relative;
  display: flex;
  align-items: stretch;
  gap: 0;
}

.ui-combo-box__input {
  width: 100%;
  box-sizing: border-box;
  flex: 1;

  padding: var(--ui-space-sm) var(--ui-space-md);

  border: 1px solid var(--ui-border);
  border-right: 0;
  border-radius: var(--ui-radius-md) 0 0 var(--ui-radius-md);

  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;

  outline: none;
}

.ui-combo-box__trigger {
  width: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;

  border: 1px solid var(--ui-border);
  border-left: 0;
  border-radius: 0 var(--ui-radius-md) var(--ui-radius-md) 0;

  background: var(--ui-bg);
  color: var(--ui-fg);
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-combo-box__trigger:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.ui-combo-box__trigger svg {
  pointer-events: none;
}

.ui-combo-box--focus-visible .ui-combo-box__input,
.ui-combo-box--focus-visible .ui-combo-box__trigger {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-combo-box--invalid .ui-combo-box__input,
.ui-combo-box--invalid .ui-combo-box__trigger {
  border-color: var(--ui-danger);
}

.ui-combo-box--invalid.ui-combo-box--focus-visible .ui-combo-box__input,
.ui-combo-box--invalid.ui-combo-box--focus-visible .ui-combo-box__trigger {
  outline-color: var(--ui-danger);
}

.ui-combo-box__description,
.ui-combo-box__error {
  font-size: 12px;
  line-height: 1.3;
}

.ui-combo-box__description {
  color: var(--ui-fg-muted);
}

.ui-combo-box__error {
  color: var(--ui-danger);
}

.ui-combo-box__panel {
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

.ui-combo-box__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-combo-box__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-combo-box__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-combo-box__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}

.ui-combo-box__listbox {
  padding: 4px;
}

.ui-combo-box__options {
  position: relative;
}

.ui-combo-box__option {
  position: relative;
  padding: var(--ui-space-sm) var(--ui-space-md);
  border-radius: var(--ui-radius-md);
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-combo-box__option[data-disabled=\"true\"] {
  cursor: not-allowed;
  opacity: 0.5;
}

.ui-combo-box__option[data-selected=\"true\"] {
  font-weight: 600;
}

.ui-combo-box__option[data-focused=\"true\"] {
  background: var(--ui-accent-soft);
}

.ui-combo-box__empty {
  padding: var(--ui-space-sm) var(--ui-space-md);
  color: var(--ui-fg-muted);
}

.ui-combo-box__option:focus-visible {
  outline: none;
}
"#;
