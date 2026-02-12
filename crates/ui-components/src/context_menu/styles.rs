pub const CSS: &str = r#"
.ui-context-menu {
  display: inline-flex;
  align-items: stretch;
}

.ui-context-menu--open,
.ui-context-menu[data-state="open"] {
  opacity: 1;
}

.ui-context-menu--closed,
.ui-context-menu[data-state="closed"] {
  opacity: 1;
}

.ui-context-menu--disabled,
.ui-context-menu[data-state="disabled"],
.ui-context-menu[data-disabled="true"] {
  opacity: 0.72;
}

.ui-context-menu--enabled,
.ui-context-menu[data-disabled="false"] {
  opacity: 1;
}

.ui-context-menu--has-items,
.ui-context-menu[data-items="populated"] {
  --ui-context-menu-item-count: 1;
}

.ui-context-menu--empty,
.ui-context-menu[data-items="empty"] {
  --ui-context-menu-item-count: 0;
}

.ui-context-menu--persistent,
.ui-context-menu[data-action-mode="keep-open"],
.ui-context-menu[data-close-on-action="false"] {
  box-shadow: var(--ui-shadow-sm);
}

.ui-context-menu--close-on-action,
.ui-context-menu[data-action-mode="close"],
.ui-context-menu[data-close-on-action="true"] {
  box-shadow: none;
}

.ui-context-menu--controlled,
.ui-context-menu[data-open-mode="controlled"] {
  --ui-context-menu-controlled: 1;
}

.ui-context-menu--uncontrolled,
.ui-context-menu[data-open-mode="uncontrolled"] {
  --ui-context-menu-controlled: 0;
}

.ui-context-menu--placement-top-start,
.ui-context-menu--placement-top-end,
.ui-context-menu[data-placement="top-start"],
.ui-context-menu[data-placement="top-end"] {
  --ui-context-menu-placement-y: -1;
}

.ui-context-menu--placement-bottom-start,
.ui-context-menu--placement-bottom-end,
.ui-context-menu[data-placement="bottom-start"],
.ui-context-menu[data-placement="bottom-end"] {
  --ui-context-menu-placement-y: 1;
}

.ui-context-menu--custom-motion,
.ui-context-menu[data-motion-source="custom"],
.ui-context-menu[data-custom-motion="true"] {
  --ui-context-menu-custom-motion: 1;
}

.ui-context-menu[data-id-source="custom"],
.ui-context-menu[data-custom-id="true"],
.ui-context-menu--custom-id {
  --ui-context-menu-custom-id: 1;
}

.ui-context-menu[data-aria-label-source="custom"],
.ui-context-menu[data-custom-aria-label="true"],
.ui-context-menu--custom-aria-label {
  --ui-context-menu-custom-aria-label: 1;
}

.ui-context-menu[data-class-source="custom"],
.ui-context-menu[data-custom-class="true"],
.ui-context-menu--custom-class {
  --ui-context-menu-custom-class: 1;
}

.ui-context-menu[data-disabled-source="custom"],
.ui-context-menu[data-custom-disabled="true"],
.ui-context-menu--custom-disabled {
  --ui-context-menu-custom-disabled: 1;
}

.ui-context-menu[data-disabled-indices-source="custom"],
.ui-context-menu[data-custom-disabled-indices="true"],
.ui-context-menu--custom-disabled-indices {
  --ui-context-menu-custom-disabled-indices: 1;
}

.ui-context-menu[data-item-kinds-source="custom"],
.ui-context-menu[data-custom-item-kinds="true"],
.ui-context-menu--custom-item-kinds {
  --ui-context-menu-custom-item-kinds: 1;
}

.ui-context-menu[data-close-on-action-source="custom"],
.ui-context-menu[data-custom-close-on-action="true"],
.ui-context-menu--custom-close-on-action {
  --ui-context-menu-custom-close-on-action: 1;
}

.ui-context-menu[data-placement-source="custom"],
.ui-context-menu[data-custom-placement="true"],
.ui-context-menu--custom-placement {
  --ui-context-menu-custom-placement: 1;
}

.ui-context-menu[data-open-source="custom"],
.ui-context-menu[data-custom-open="true"],
.ui-context-menu--custom-open {
  --ui-context-menu-custom-open: 1;
}

.ui-context-menu[data-default-open-source="custom"],
.ui-context-menu[data-custom-default-open="true"],
.ui-context-menu--custom-default-open {
  --ui-context-menu-custom-default-open: 1;
}

.ui-context-menu[data-open-change-source="custom"],
.ui-context-menu[data-custom-open-change="true"],
.ui-context-menu--custom-open-change {
  --ui-context-menu-custom-open-change: 1;
}

.ui-context-menu__trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: min(100%, 22rem);
  min-height: 7.5rem;
  border: 1px dashed color-mix(in oklab, var(--ui-border) 82%, var(--ui-accent) 18%);
  border-radius: var(--ui-radius-lg);
  background: color-mix(in oklab, var(--ui-bg) 90%, var(--ui-bg-muted) 10%);
  color: var(--ui-fg-muted);
  font: inherit;
  padding: var(--ui-space-md);
  text-align: center;
  cursor: context-menu;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-context-menu__trigger:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-context-menu--disabled .ui-context-menu__trigger,
.ui-context-menu__trigger:disabled {
  opacity: 0.58;
  cursor: not-allowed;
}

.ui-context-menu--empty .ui-context-menu__trigger {
  border: 1px dotted color-mix(in oklab, var(--ui-border) 82%, var(--ui-accent) 18%);
}
"#;
