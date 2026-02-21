pub const CSS: &str = r#"
.ui-combo-box {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-combo-box[data-label-source="custom"],
.ui-combo-box[data-custom-label="true"],
.ui-combo-box--custom-label {
  --ui-combo-box-label-source: custom;
}

.ui-combo-box[data-description-source="custom"],
.ui-combo-box[data-custom-description="true"],
.ui-combo-box--custom-description {
  --ui-combo-box-description-source: custom;
}

.ui-combo-box[data-error-source="custom"],
.ui-combo-box[data-custom-error="true"],
.ui-combo-box--custom-error {
  --ui-combo-box-error-source: custom;
}

.ui-combo-box[data-placeholder-source="custom"],
.ui-combo-box[data-custom-placeholder="true"],
.ui-combo-box--custom-placeholder {
  --ui-combo-box-placeholder-source: custom;
}

.ui-combo-box[data-id-source="custom"],
.ui-combo-box[data-custom-id="true"],
.ui-combo-box--custom-id {
  --ui-combo-box-id-source: custom;
}

.ui-combo-box[data-class-source="custom"],
.ui-combo-box[data-custom-class="true"],
.ui-combo-box--custom-class {
  --ui-combo-box-class-source: custom;
}

.ui-combo-box[data-motion-source="custom"],
.ui-combo-box[data-custom-motion="true"],
.ui-combo-box--custom-motion {
  --ui-combo-box-custom-motion: 1;
}

.ui-combo-box--empty .ui-combo-box__trigger,
.ui-combo-box[data-empty="true"] .ui-combo-box__trigger {
  opacity: 0.72;
}

.ui-combo-box--controlled .ui-combo-box__trigger,
.ui-combo-box[data-controlled="true"] .ui-combo-box__trigger {
  box-shadow: inset 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width)) var(--ui-border, var(--ui-fallback-border));
}

.ui-combo-box--has-disabled-options .ui-combo-box__listbox,
.ui-combo-box[data-has-disabled-options="true"] .ui-combo-box__listbox {
  --ui-combo-box-has-disabled-options: 1;
}

.ui-combo-box__field {
  position: relative;
}

.ui-combo-box__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;
  color: var(--ui-fg, var(--ui-fallback-fg));
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

  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));

  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-right: 0;
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md)) 0 0 var(--ui-radius-md, var(--ui-fallback-radius-md));

  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font: inherit;

  outline: none;
}

.ui-combo-box__trigger {
  width: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
  display: inline-flex;
  align-items: center;
  justify-content: center;

  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-left: 0;
  border-radius: 0 var(--ui-radius-md, var(--ui-fallback-radius-md)) var(--ui-radius-md, var(--ui-fallback-radius-md)) 0;

  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
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
  outline: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 3) solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2);
}

.ui-combo-box--invalid .ui-combo-box__input,
.ui-combo-box--invalid .ui-combo-box__trigger {
  border-color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-combo-box--invalid.ui-combo-box--focus-visible .ui-combo-box__input,
.ui-combo-box--invalid.ui-combo-box--focus-visible .ui-combo-box__trigger {
  outline-color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-combo-box__description,
.ui-combo-box__error {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-combo-box__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-combo-box__error {
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-combo-box__panel {
  position: fixed;
  top: var(--ui-popover-top, calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 0));
  left: var(--ui-popover-left, calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 0));
  width: var(--ui-popover-anchor-width, var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)));
  max-width: calc(100vw - var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset)));
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));

  padding: 0;
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));

  --ui-popover-opacity: 0;
  --ui-popover-scale: var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale));
  --ui-popover-y: var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y));

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
  padding: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-combo-box__options {
  position: relative;
}

.ui-combo-box__option {
  position: relative;
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
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
  background: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
}

.ui-combo-box__empty {
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-combo-box__option:focus-visible {
  outline: none;
}
"#;
