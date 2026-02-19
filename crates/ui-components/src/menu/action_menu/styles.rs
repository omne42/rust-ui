pub const CSS: &str = r#"
.ui-action-menu {
  display: inline-flex;
}

.ui-action-menu--open,
.ui-action-menu[data-state="open"] {
  --ui-action-menu-open: 1;
}

.ui-action-menu--closed,
.ui-action-menu[data-state="closed"] {
  --ui-action-menu-open: 0;
}

.ui-action-menu--disabled,
.ui-action-menu[data-state="disabled"],
.ui-action-menu[data-disabled="true"] {
  opacity: 0.72;
}

.ui-action-menu--enabled,
.ui-action-menu[data-enabled="true"] {
  opacity: 1;
}

.ui-action-menu--empty,
.ui-action-menu[data-state="empty"],
.ui-action-menu[data-items="empty"] {
  --ui-action-menu-empty: 1;
}

.ui-action-menu--has-items,
.ui-action-menu[data-items="populated"] {
  --ui-action-menu-empty: 0;
}

.ui-action-menu--persistent,
.ui-action-menu[data-action-mode="keep-open"],
.ui-action-menu[data-keep-open-on-action="true"] {
  --ui-action-menu-persistent: 1;
}

.ui-action-menu--close-on-action,
.ui-action-menu[data-action-mode="close"],
.ui-action-menu[data-close-on-action="true"] {
  --ui-action-menu-persistent: 0;
}

.ui-action-menu--controlled,
.ui-action-menu[data-open-mode="controlled"] {
  --ui-action-menu-controlled: 1;
}

.ui-action-menu--uncontrolled,
.ui-action-menu[data-open-mode="uncontrolled"] {
  --ui-action-menu-controlled: 0;
}

.ui-action-menu--custom-motion,
.ui-action-menu[data-motion-source="custom"],
.ui-action-menu[data-custom-motion="true"] {
  --ui-action-menu-custom-motion: 1;
}

.ui-action-menu[data-id-source="custom"],
.ui-action-menu[data-custom-id="true"],
.ui-action-menu--custom-id {
  --ui-action-menu-custom-id: 1;
}

.ui-action-menu[data-aria-label-source="custom"],
.ui-action-menu[data-custom-aria-label="true"],
.ui-action-menu--custom-aria-label {
  --ui-action-menu-custom-aria-label: 1;
}

.ui-action-menu[data-class-source="custom"],
.ui-action-menu[data-custom-class="true"],
.ui-action-menu--custom-class {
  --ui-action-menu-custom-class: 1;
}

.ui-action-menu[data-disabled-source="custom"],
.ui-action-menu[data-custom-disabled="true"],
.ui-action-menu--custom-disabled {
  --ui-action-menu-custom-disabled: 1;
}

.ui-action-menu[data-disabled-indices-source="custom"],
.ui-action-menu[data-custom-disabled-indices="true"],
.ui-action-menu--custom-disabled-indices {
  --ui-action-menu-custom-disabled-indices: 1;
}

.ui-action-menu[data-item-kinds-source="custom"],
.ui-action-menu[data-custom-item-kinds="true"],
.ui-action-menu--custom-item-kinds {
  --ui-action-menu-custom-item-kinds: 1;
}

.ui-action-menu[data-close-on-action-source="custom"],
.ui-action-menu[data-custom-close-on-action="true"],
.ui-action-menu--custom-close-on-action {
  --ui-action-menu-custom-close-on-action: 1;
}

.ui-action-menu[data-placement-source="custom"],
.ui-action-menu[data-custom-placement="true"],
.ui-action-menu--custom-placement {
  --ui-action-menu-custom-placement: 1;
}

.ui-action-menu[data-open-source="custom"],
.ui-action-menu[data-custom-open="true"],
.ui-action-menu--custom-open {
  --ui-action-menu-custom-open: 1;
}

.ui-action-menu[data-default-open-source="custom"],
.ui-action-menu[data-custom-default-open="true"],
.ui-action-menu--custom-default-open {
  --ui-action-menu-custom-default-open: 1;
}

.ui-action-menu[data-open-change-source="custom"],
.ui-action-menu[data-custom-open-change="true"],
.ui-action-menu--custom-open-change {
  --ui-action-menu-custom-open-change: 1;
}

.ui-action-menu--persistent .ui-action-button,
.ui-action-menu[data-action-mode="keep-open"] .ui-action-button {
  box-shadow: var(--ui-shadow-sm);
}
"#;
