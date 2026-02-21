pub const CSS: &str = r#"
.ui-menubar {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  min-height: var(--ui-button-size-m-height, var(--ui-fallback-button-size-m-height));
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 92%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 8%
  );
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-menubar--open,
.ui-menubar[data-state="open"] {
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}

.ui-menubar--closed,
.ui-menubar[data-state="closed"] {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-menubar--empty,
.ui-menubar[data-state="empty"],
.ui-menubar[data-menus="empty"] {
  border-color: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 65%,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 35%
  );
}

.ui-menubar--has-menus,
.ui-menubar[data-menus="populated"] {
  border-color: var(--ui-border, var(--ui-fallback-border));
}

.ui-menubar--persistent,
.ui-menubar[data-action-mode="keep-open"],
.ui-menubar[data-close-on-action="false"] {
  --ui-menubar-persistent: 1;
}

.ui-menubar--close-on-action,
.ui-menubar[data-action-mode="close"],
.ui-menubar[data-close-on-action="true"] {
  --ui-menubar-persistent: 0;
}

.ui-menubar--controlled,
.ui-menubar[data-open-mode="controlled"] {
  --ui-menubar-controlled: 1;
}

.ui-menubar--uncontrolled,
.ui-menubar[data-open-mode="uncontrolled"] {
  --ui-menubar-controlled: 0;
}

.ui-menubar--custom-motion,
.ui-menubar[data-motion-source="custom"],
.ui-menubar[data-custom-motion="true"] {
  --ui-menubar-custom-motion: 1;
}

.ui-menubar--custom-id,
.ui-menubar[data-id-source="custom"],
.ui-menubar[data-custom-id="true"] {
  --ui-menubar-custom-id: 1;
}

.ui-menubar[data-class-source="custom"],
.ui-menubar[data-custom-class="true"] {
  --ui-menubar-custom-class: 1;
}

.ui-menubar--custom-close-on-action,
.ui-menubar[data-close-on-action-source="custom"],
.ui-menubar[data-custom-close-on-action="true"] {
  --ui-menubar-custom-close-on-action: 1;
}

.ui-menubar--custom-placement,
.ui-menubar[data-placement-source="custom"],
.ui-menubar[data-custom-placement="true"] {
  --ui-menubar-custom-placement: 1;
}

.ui-menubar--custom-open-index,
.ui-menubar[data-open-index-source="custom"],
.ui-menubar[data-custom-open-index="true"] {
  --ui-menubar-custom-open-index: 1;
}

.ui-menubar--custom-default-open-index,
.ui-menubar[data-default-open-index-source="custom"],
.ui-menubar[data-custom-default-open-index="true"] {
  --ui-menubar-custom-default-open-index: 1;
}

.ui-menubar--custom-open-index-change,
.ui-menubar[data-open-index-change-source="custom"],
.ui-menubar[data-custom-open-index-change="true"] {
  --ui-menubar-custom-open-index-change: 1;
}

.ui-menubar__menu {
  position: relative;
  display: inline-flex;
}

.ui-menubar__trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: var(--ui-button-size-m-height, var(--ui-fallback-button-size-m-height));
  padding: 0 var(--ui-space-sm, var(--ui-fallback-space-sm));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: transparent;
  color: var(--ui-fg, var(--ui-fallback-fg));
  font: inherit;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-menubar__trigger:hover:not(:disabled),
.ui-menubar__menu[data-open="true"] .ui-menubar__trigger,
.ui-menubar__menu[data-state="open"] .ui-menubar__trigger {
  border-color: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 80%,
    var(--ui-accent, var(--ui-fallback-accent)) 20%
  );
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 78%,
    var(--ui-accent, var(--ui-fallback-accent)) 22%
  );
}

.ui-menubar__trigger:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
}

.ui-menubar__trigger:disabled,
.ui-menubar__menu[data-disabled="true"] .ui-menubar__trigger,
.ui-menubar__menu[data-state="disabled"] .ui-menubar__trigger {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
  cursor: not-allowed;
}
"#;
