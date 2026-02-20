pub const CSS: &str = r#"
.ui-toggle {
  --ui-toggle-selected: 0;
}

.ui-toggle[data-state="selected"],
.ui-toggle[data-selected="true"] {
  --ui-toggle-selected: 1;
}

.ui-toggle[data-state="unselected"],
.ui-toggle[data-unselected="true"] {
  --ui-toggle-selected: 0;
}

.ui-toggle[data-interaction="pressed"] {
  --ui-toggle-interaction: pressed;
}

.ui-toggle[data-interaction="hovered"] {
  --ui-toggle-interaction: hovered;
}

.ui-toggle[data-interaction="focus-visible"] {
  --ui-toggle-interaction: focus-visible;
}

.ui-toggle[data-variant="outline"] {
  --ui-toggle-variant: outline;
}

.ui-toggle[data-variant="ghost"] {
  --ui-toggle-variant: ghost;
}

.ui-toggle[data-size="sm"] {
  --ui-toggle-size: sm;
}

.ui-toggle[data-size="lg"] {
  --ui-toggle-size: lg;
}

.ui-toggle[data-variant-source="custom"] {
  --ui-toggle-variant-source: custom;
}

.ui-toggle[data-size-source="custom"] {
  --ui-toggle-size-source: custom;
}

.ui-toggle[data-class-source="custom"],
.ui-toggle--custom-class {
  --ui-toggle-class-source: custom;
}

.ui-toggle[data-motion-source="custom"],
.ui-toggle[data-custom-motion="true"],
.ui-toggle--custom-motion {
  --ui-toggle-custom-motion: 1;
}

.ui-toggle[data-aria-source="custom"] {
  --ui-toggle-aria-source: custom;
}

.ui-toggle[data-custom-aria-label="true"],
.ui-toggle[data-custom-aria="true"] {
  --ui-toggle-custom-aria-label: 1;
}

.ui-toggle[data-handler-source="custom"] {
  --ui-toggle-handler-source: custom;
}

.ui-toggle[data-selected="true"] .ui-toggle-button__label {
  font-weight: 600;
}
"#;

#[cfg(feature = "component-toggle_group")]
pub const TOGGLE_GROUP_CSS: &str = r#"
.ui-toggle-group {
  display: inline-flex;
}

.ui-toggle-group__items {
  display: inline-flex;
  gap: var(--ui-space-xs);
}

.ui-toggle-group--horizontal .ui-toggle-group__items {
  flex-direction: row;
  align-items: center;
}

.ui-toggle-group--vertical .ui-toggle-group__items {
  flex-direction: column;
  align-items: flex-start;
}

.ui-toggle-group--disabled {
  opacity: 0.8;
}

.ui-toggle-group--attached .ui-toggle-group__items {
  gap: 0;
}

.ui-toggle-group--attached .ui-toggle-group__item {
  position: relative;
}

.ui-toggle-group--attached .ui-toggle-group__item.ui-toggle-button--focus-visible {
  z-index: 1;
}

.ui-toggle-group--attached.ui-toggle-group--horizontal .ui-toggle-group__item:not(:first-child) {
  margin-left: -1px;
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--horizontal .ui-toggle-group__item:not(:last-child) {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--vertical .ui-toggle-group__item:not(:first-child) {
  margin-top: -1px;
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--vertical .ui-toggle-group__item:not(:last-child) {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}
"#;
