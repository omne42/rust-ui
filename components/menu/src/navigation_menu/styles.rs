pub const CSS: &str = r#"
.ui-navigation-menu {
  display: block;
  width: fit-content;
  max-width: 100%;
}

.ui-navigation-menu--selected,
.ui-navigation-menu[data-state="selected"] {
  --ui-navigation-menu-selected: 1;
}

.ui-navigation-menu--focused,
.ui-navigation-menu[data-state="focused"] {
  --ui-navigation-menu-focused: 1;
}

.ui-navigation-menu--empty,
.ui-navigation-menu[data-state="empty"],
.ui-navigation-menu[data-items="empty"] {
  --ui-navigation-menu-empty: 1;
}

.ui-navigation-menu--has-items,
.ui-navigation-menu[data-items="populated"] {
  --ui-navigation-menu-empty: 0;
}

.ui-navigation-menu--auto-activation,
.ui-navigation-menu[data-focus-activation="auto"] {
  --ui-navigation-menu-focus-auto: 1;
}

.ui-navigation-menu--manual-activation,
.ui-navigation-menu[data-focus-activation="manual"] {
  --ui-navigation-menu-focus-auto: 0;
}

.ui-navigation-menu--controlled,
.ui-navigation-menu[data-selection-mode="controlled"] {
  --ui-navigation-menu-controlled: 1;
}

.ui-navigation-menu--uncontrolled,
.ui-navigation-menu[data-selection-mode="uncontrolled"] {
  --ui-navigation-menu-controlled: 0;
}

.ui-navigation-menu--custom-motion,
.ui-navigation-menu[data-motion-source="custom"],
.ui-navigation-menu[data-custom-motion="true"] {
  --ui-navigation-menu-custom-motion: 1;
}

.ui-navigation-menu--custom-id,
.ui-navigation-menu[data-id-source="custom"],
.ui-navigation-menu[data-custom-id="true"] {
  --ui-navigation-menu-custom-id: 1;
}

.ui-navigation-menu--custom-aria-label,
.ui-navigation-menu[data-aria-label-source="custom"],
.ui-navigation-menu[data-custom-aria-label="true"] {
  --ui-navigation-menu-custom-aria-label: 1;
}

.ui-navigation-menu[data-class-source="custom"],
.ui-navigation-menu[data-custom-class="true"] {
  --ui-navigation-menu-custom-class: 1;
}

.ui-navigation-menu--custom-activate-on-focus,
.ui-navigation-menu[data-activate-on-focus-source="custom"],
.ui-navigation-menu[data-custom-activate-on-focus="true"] {
  --ui-navigation-menu-custom-activate-on-focus: 1;
}

.ui-navigation-menu--custom-selected-id,
.ui-navigation-menu[data-selected-id-source="custom"],
.ui-navigation-menu[data-custom-selected-id="true"] {
  --ui-navigation-menu-custom-selected-id: 1;
}

.ui-navigation-menu--custom-default-selected-id,
.ui-navigation-menu[data-default-selected-id-source="custom"],
.ui-navigation-menu[data-custom-default-selected-id="true"] {
  --ui-navigation-menu-custom-default-selected-id: 1;
}

.ui-navigation-menu--custom-selected-id-change,
.ui-navigation-menu[data-selected-id-change-source="custom"],
.ui-navigation-menu[data-custom-selected-id-change="true"] {
  --ui-navigation-menu-custom-selected-id-change: 1;
}

.ui-navigation-menu__list {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 92%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 8%
  );
}

.ui-navigation-menu__item {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: var(--ui-button-size-m-height, var(--ui-fallback-button-size-m-height));
  padding: 0 var(--ui-space-sm, var(--ui-fallback-space-sm));
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  text-decoration: none;
  transition: color
    var(
      --ui-label-motion-color-duration,
      var(--ui-fallback-label-motion-color-duration)
    )
    var(--ui-label-motion-easing, var(--ui-fallback-label-motion-easing));
  user-select: none;
}

.ui-navigation-menu__item:hover:not([data-disabled="true"]) {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 70%,
    var(--ui-accent, var(--ui-fallback-accent)) 30%
  );
}

.ui-navigation-menu__item[data-selected="true"],
.ui-navigation-menu__item[data-state="selected"] {
  color: var(--ui-accent-fg, var(--ui-fallback-accent-fg));
}

.ui-navigation-menu__item:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
}

.ui-navigation-menu__item[data-disabled="true"],
.ui-navigation-menu__item[data-state="disabled"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
  pointer-events: none;
}

.ui-navigation-menu--empty .ui-navigation-menu__list,
.ui-navigation-menu[data-state="empty"] .ui-navigation-menu__list {
  border-color: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 72%,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 28%
  );
}

.ui-navigation-menu--selected .ui-navigation-menu__list,
.ui-navigation-menu[data-state="selected"] .ui-navigation-menu__list {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}
"#;
