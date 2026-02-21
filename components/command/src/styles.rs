pub const CSS: &str = r#"
.ui-command {
  width: min(100%, var(--ui-command-panel-max-width, var(--ui-fallback-command-panel-max-width)));
  display: flex;
  flex-direction: column;
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  overflow: hidden;
}

.ui-command--empty,
.ui-command[data-state="empty"],
.ui-command[data-items="empty"] {
  --ui-command-empty: 1;
}

.ui-command--has-items,
.ui-command[data-items="populated"] {
  --ui-command-empty: 0;
}

.ui-command--querying,
.ui-command[data-query="present"] {
  --ui-command-querying: 1;
}

.ui-command--idle,
.ui-command[data-query="absent"] {
  --ui-command-querying: 0;
}

.ui-command--disabled,
.ui-command[data-disabled="disabled"],
.ui-command[data-is-disabled="true"] {
  opacity: var(--ui-command-disabled-opacity, var(--ui-fallback-command-disabled-opacity));
}

.ui-command--enabled,
.ui-command[data-disabled="enabled"],
.ui-command[data-is-enabled="true"] {
  opacity: 1;
}

.ui-command--custom-motion,
.ui-command[data-motion-source="custom"],
.ui-command[data-custom-motion="true"] {
  --ui-command-custom-motion: 1;
}

.ui-command[data-id-source="custom"],
.ui-command[data-custom-id="true"],
.ui-command--custom-id {
  --ui-command-custom-id: 1;
}

.ui-command[data-placeholder-source="custom"],
.ui-command[data-custom-placeholder="true"],
.ui-command--custom-placeholder {
  --ui-command-custom-placeholder: 1;
}

.ui-command[data-empty-label-source="custom"],
.ui-command[data-custom-empty-label="true"],
.ui-command--custom-empty-label {
  --ui-command-custom-empty-label: 1;
}

.ui-command[data-aria-label-source="custom"],
.ui-command[data-custom-aria-label="true"],
.ui-command--custom-aria-label {
  --ui-command-custom-aria-label: 1;
}

.ui-command[data-class-source="custom"],
.ui-command[data-custom-class="true"],
.ui-command--custom-class {
  --ui-command-custom-class: 1;
}

.ui-command[data-disabled-source="custom"],
.ui-command[data-custom-disabled="true"],
.ui-command--custom-disabled {
  --ui-command-custom-disabled: 1;
}

.ui-command[data-action-source="custom"],
.ui-command[data-custom-action="true"],
.ui-command--custom-action {
  --ui-command-custom-action: 1;
}

.ui-command__input-wrap {
  padding: var(--ui-command-input-wrap-padding, var(--ui-fallback-command-input-wrap-padding));
  border-bottom: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border))
        var(--ui-command-input-wrap-border-mix, var(--ui-fallback-command-input-wrap-border-mix)),
      var(--ui-bg, var(--ui-fallback-bg))
        calc(100% - var(--ui-command-input-wrap-border-mix, var(--ui-fallback-command-input-wrap-border-mix)))
    );
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg))
      var(--ui-command-input-wrap-bg-mix, var(--ui-fallback-command-input-wrap-bg-mix)),
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
      calc(100% - var(--ui-command-input-wrap-bg-mix, var(--ui-fallback-command-input-wrap-bg-mix)))
  );
}

.ui-command__input {
  width: 100%;
  box-sizing: border-box;
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font: inherit;
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  padding:
    var(--ui-command-input-padding-y, var(--ui-fallback-command-input-padding-y))
    var(--ui-command-input-padding-x, var(--ui-fallback-command-input-padding-x));
  outline: none;
}

.ui-command__input:focus-visible {
  outline: var(--ui-command-input-focus-outline-width, var(--ui-fallback-command-input-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-command-input-focus-outline-offset, var(--ui-fallback-command-input-focus-outline-offset));
}

.ui-command__list {
  max-height: var(--ui-command-list-max-height, var(--ui-fallback-command-list-max-height));
  overflow: auto;
}

.ui-command__options {
  position: relative;
  padding: var(--ui-command-options-padding, var(--ui-fallback-command-options-padding));
}

.ui-command__group {
  display: grid;
  gap: var(--ui-command-group-gap, var(--ui-fallback-command-group-gap));
}

.ui-command__group + .ui-command__group {
  margin-top: var(--ui-command-group-spacing, var(--ui-fallback-command-group-spacing));
  padding-top: var(--ui-command-group-spacing, var(--ui-fallback-command-group-spacing));
  border-top: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border))
        var(--ui-command-group-border-mix, var(--ui-fallback-command-group-border-mix)),
      var(--ui-bg, var(--ui-fallback-bg))
        calc(100% - var(--ui-command-group-border-mix, var(--ui-fallback-command-group-border-mix)))
    );
}

.ui-command__group-heading {
  margin: 0;
  padding-inline: var(--ui-command-group-heading-padding-x, var(--ui-fallback-command-group-heading-padding-x));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 600;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  text-transform: uppercase;
  letter-spacing: var(
    --ui-command-group-heading-letter-spacing,
    var(--ui-fallback-command-group-heading-letter-spacing)
  );
}

.ui-command__group-items {
  display: grid;
  gap: var(--ui-command-group-items-gap, var(--ui-fallback-command-group-items-gap));
}

.ui-command__option {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-command-option-gap, var(--ui-fallback-command-option-gap));
  padding:
    var(--ui-command-option-padding-y, var(--ui-fallback-command-option-padding-y))
    var(--ui-command-option-padding-x, var(--ui-fallback-command-option-padding-x));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  user-select: none;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

.ui-command__option[data-focused="true"],
.ui-command__option[data-state="focused"] {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg))
      var(--ui-command-option-focus-mix, var(--ui-fallback-command-option-focus-mix)),
    var(--ui-accent, var(--ui-fallback-accent))
      calc(100% - var(--ui-command-option-focus-mix, var(--ui-fallback-command-option-focus-mix)))
  );
}

.ui-command__option[data-selected="true"] .ui-command__item-label,
.ui-command__option[data-state="selected"] .ui-command__item-label {
  font-weight: 600;
}

.ui-command__option[data-disabled="true"],
.ui-command__option[data-state="disabled"] {
  opacity: var(--ui-command-option-disabled-opacity, var(--ui-fallback-command-option-disabled-opacity));
  cursor: not-allowed;
}

.ui-command__shortcut {
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border))
        var(--ui-command-shortcut-border-mix, var(--ui-fallback-command-shortcut-border-mix)),
      var(--ui-bg-muted, var(--ui-fallback-bg-muted))
        calc(100% - var(--ui-command-shortcut-border-mix, var(--ui-fallback-command-shortcut-border-mix)))
    );
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  padding-inline: var(--ui-command-shortcut-padding-x, var(--ui-fallback-command-shortcut-padding-x));
  padding-block: var(--ui-command-shortcut-padding-y, var(--ui-fallback-command-shortcut-padding-y));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg))
      var(--ui-command-shortcut-bg-mix, var(--ui-fallback-command-shortcut-bg-mix)),
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
      calc(100% - var(--ui-command-shortcut-bg-mix, var(--ui-fallback-command-shortcut-bg-mix)))
  );
}

.ui-command__empty {
  padding:
    var(--ui-command-empty-padding-y, var(--ui-fallback-command-empty-padding-y))
    var(--ui-command-empty-padding-x, var(--ui-fallback-command-empty-padding-x));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
}
"#;
