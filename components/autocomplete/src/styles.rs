pub const CSS: &str = r#"
.ui-autocomplete {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-autocomplete[data-label-source="custom"],
.ui-autocomplete[data-custom-label="true"],
.ui-autocomplete--custom-label {
  --ui-autocomplete-label-source: custom;
}

.ui-autocomplete[data-description-source="custom"],
.ui-autocomplete[data-custom-description="true"],
.ui-autocomplete--custom-description {
  --ui-autocomplete-description-source: custom;
}

.ui-autocomplete[data-error-source="custom"],
.ui-autocomplete[data-custom-error="true"],
.ui-autocomplete--custom-error {
  --ui-autocomplete-error-source: custom;
}

.ui-autocomplete[data-placeholder-source="custom"],
.ui-autocomplete[data-custom-placeholder="true"],
.ui-autocomplete--custom-placeholder {
  --ui-autocomplete-placeholder-source: custom;
}

.ui-autocomplete[data-id-source="custom"],
.ui-autocomplete[data-custom-id="true"],
.ui-autocomplete--custom-id {
  --ui-autocomplete-id-source: custom;
}

.ui-autocomplete[data-class-source="custom"],
.ui-autocomplete[data-custom-class="true"],
.ui-autocomplete--custom-class {
  --ui-autocomplete-class-source: custom;
}

.ui-autocomplete[data-motion-source="custom"],
.ui-autocomplete[data-custom-motion="true"],
.ui-autocomplete--custom-motion {
  --ui-autocomplete-custom-motion: 1;
}

.ui-autocomplete--empty .ui-autocomplete__input,
.ui-autocomplete[data-empty="true"] .ui-autocomplete__input {
  opacity: 0.72;
}

.ui-autocomplete--controlled .ui-autocomplete__control,
.ui-autocomplete[data-controlled="true"] .ui-autocomplete__control {
  box-shadow: inset 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width)) var(--ui-border, var(--ui-fallback-border));
}

.ui-autocomplete--has-disabled-options .ui-autocomplete__listbox,
.ui-autocomplete[data-has-disabled-options="true"] .ui-autocomplete__listbox {
  --ui-autocomplete-has-disabled-options: 1;
}

.ui-autocomplete__label {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-autocomplete__control {
  position: relative;
  display: flex;
  align-items: stretch;
}

.ui-autocomplete__input {
  width: 100%;
  box-sizing: border-box;

  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));

  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));

  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font: inherit;
  outline: none;
}

.ui-autocomplete--focus-visible .ui-autocomplete__input {
  outline: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 3) solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2);
}

.ui-autocomplete--invalid .ui-autocomplete__input {
  border-color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-autocomplete--invalid.ui-autocomplete--focus-visible .ui-autocomplete__input {
  outline-color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-autocomplete__description,
.ui-autocomplete__error {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-autocomplete__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-autocomplete__error {
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-autocomplete__panel {
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
  padding: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-autocomplete__options {
  position: relative;
}

.ui-autocomplete__option {
  position: relative;
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
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

.ui-autocomplete__option[data-focused=\"true\"] {
  background: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
}

.ui-autocomplete__empty {
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm)) var(--ui-space-md, var(--ui-fallback-space-md));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-autocomplete__option:focus-visible {
  outline: none;
}
"#;
