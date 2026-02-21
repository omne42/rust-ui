pub const CSS: &str = r#"
.ui-listbox {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: var(--ui-bg, var(--ui-fallback-bg));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  box-sizing: border-box;
  outline: none;
}

.ui-listbox--focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
}

.ui-listbox__options {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox__option {
  position: relative;
  z-index: 1;
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs))
    var(--ui-space-xs, var(--ui-fallback-space-xs));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-listbox .ui-active-highlight {
  transition:
    transform var(--ui-motion-duration-medium, var(--ui-fallback-text-field-motion-duration))
      var(--ui-motion-ease-emphasized, var(--ui-fallback-text-field-motion-easing)),
    height var(--ui-motion-duration-medium, var(--ui-fallback-text-field-motion-duration))
      var(--ui-motion-ease-emphasized, var(--ui-fallback-text-field-motion-easing)),
    opacity var(--ui-motion-duration-fast, var(--ui-fallback-text-field-motion-duration))
      var(--ui-motion-ease-standard, var(--ui-fallback-text-field-motion-easing));
}

.ui-listbox__option[data-selected="true"] {
  font-weight: 600;
}

.ui-listbox__option[data-disabled="true"] {
  opacity: 0.5;
}
"#;

pub const ITEM_CSS: &str = r#"
.ui-listbox-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs))
    var(--ui-space-xs, var(--ui-fallback-space-xs));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-listbox-item--selected,
.ui-listbox-item[data-selected="true"] {
  font-weight: 600;
}

.ui-listbox-item--focused,
.ui-listbox-item[data-focused="true"] {
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 80%,
    var(--ui-accent, var(--ui-fallback-accent)) 20%
  );
}

.ui-listbox-item--disabled,
.ui-listbox-item[data-disabled="true"] {
  opacity: 0.5;
  cursor: not-allowed;
}

.ui-listbox-item--selection-indicator,
.ui-listbox-item[data-show-selection-indicator="true"] {
  padding-inline-start: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-listbox-item__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-space-md, var(--ui-fallback-space-md));
  flex-shrink: 0;
}

.ui-listbox-item__label {
  min-width: 0;
  flex: 1;
  display: inline-flex;
  align-items: center;
}

.ui-listbox-item__divider {
  position: absolute;
  inset-inline: var(--ui-space-xs, var(--ui-fallback-space-xs));
  bottom: 0;
  height: var(--ui-border-width, var(--ui-fallback-border-width));
  background: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 90%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 10%
  );
}

.ui-listbox-item__selection-sr {
  position: absolute;
  width: var(--ui-border-width, var(--ui-fallback-border-width));
  height: var(--ui-border-width, var(--ui-fallback-border-width));
  margin: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * -1);
  padding: 0;
  border: 0;
  overflow: hidden;
  clip: rect(0 0 0 0);
  white-space: nowrap;
}

.ui-listbox-item--divider,
.ui-listbox-item[data-has-divider="true"] {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-listbox-item--custom-class,
.ui-listbox-item[data-custom-class="true"] {
  box-shadow: 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 22%, transparent) inset;
}
"#;

pub const SECTION_CSS: &str = r#"
.ui-listbox-section {
  display: grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox-section__header {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 600;
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 88%,
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
  padding-inline: var(--ui-space-xs, var(--ui-fallback-space-xs));
  padding-block: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox-section--tone-default .ui-listbox-section__header,
.ui-listbox-section[data-tone="default"] .ui-listbox-section__header {
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 88%,
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-listbox-section--tone-quiet .ui-listbox-section__header,
.ui-listbox-section[data-tone="quiet"] .ui-listbox-section__header {
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 96%,
    var(--ui-bg, var(--ui-fallback-bg)) 4%
  );
}

.ui-listbox-section__header[data-sticky="true"] {
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(var(--ui-space-xs, var(--ui-fallback-space-xs)));
  background: color-mix(in oklab, var(--ui-bg, var(--ui-fallback-bg)) 88%, transparent 12%);
}

.ui-listbox-section__items {
  display: grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox-section__divider {
  border-bottom: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 88%,
      var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 12%
    );
  margin-inline: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox-section--empty,
.ui-listbox-section[data-empty="true"] {
  opacity: 0.76;
}

.ui-listbox-section--disabled,
.ui-listbox-section[data-disabled="true"] {
  opacity: 0.52;
}

.ui-listbox-section--sticky-heading,
.ui-listbox-section[data-sticky-heading="true"] {
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-listbox-section--divided,
.ui-listbox-section[data-divided="true"] {
  padding-bottom: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-listbox-section--custom-class,
.ui-listbox-section[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 26%, transparent);
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
