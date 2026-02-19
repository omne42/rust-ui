pub const CSS: &str = r#"
.ui-sidebar-menu {
  position: relative;
  display: grid;
  gap: 0.4rem;
  width: 100%;
}

.ui-sidebar-menu__list {
  position: relative;
  display: grid;
  gap: 0.2rem;
}

.ui-sidebar-menu__highlight {
  position: absolute;
  inset-inline: 0;
  height: var(--ui-active-highlight-h, 0px);
  transform: translateY(var(--ui-active-highlight-y, 0px));
  opacity: var(--ui-active-highlight-o, 0);
  border-radius: var(--ui-radius-sm, 0.5rem);
  background: color-mix(in oklab, var(--ui-accent-solid, currentColor) 15%, transparent);
  pointer-events: none;
}

.ui-sidebar-menu__item {
  position: relative;
  z-index: 1;
  display: grid;
  gap: 0.2rem;
}

.ui-sidebar-menu__item-main {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  align-items: center;
  gap: 0.3rem;
}

.ui-sidebar-menu__button,
.ui-sidebar-menu__sub-button,
.ui-sidebar-menu__action,
.ui-sidebar-menu__toggle {
  border: 0;
  background: transparent;
  color: inherit;
  font: inherit;
}

.ui-sidebar-menu__button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
  min-width: 0;
  border-radius: var(--ui-radius-sm, 0.5rem);
  padding: 0.4rem 0.5rem;
  text-align: left;
}

.ui-sidebar-menu__label,
.ui-sidebar-menu__sub-label {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ui-sidebar-menu__href,
.ui-sidebar-menu__sub-href {
  color: var(--ui-fg-muted, color-mix(in oklab, currentColor 65%, transparent));
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-sidebar-menu__badge {
  margin-inline-start: auto;
  padding-inline: 0.45rem;
  border-radius: 999px;
  background: color-mix(in oklab, currentColor 12%, transparent);
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-variant-numeric: tabular-nums;
}

.ui-sidebar-menu__action,
.ui-sidebar-menu__toggle {
  border-radius: var(--ui-radius-xs, 0.375rem);
  padding: 0.25rem 0.35rem;
  line-height: 1;
}

.ui-sidebar-menu__toggle {
  transform-origin: center;
  transition: transform 150ms ease;
}

.ui-sidebar-menu__toggle[data-open="true"] {
  transform: rotate(90deg);
}

.ui-sidebar-menu__sub {
  margin-inline-start: 0.95rem;
  padding-inline-start: 0.6rem;
  border-inline-start: 1px solid
    var(--ui-border-subtle, color-mix(in oklab, currentColor 22%, transparent));
  display: grid;
  gap: 0.1rem;
}

.ui-sidebar-menu__sub-button {
  display: flex;
  align-items: center;
  gap: 0.45rem;
  width: 100%;
  border-radius: var(--ui-radius-xs, 0.375rem);
  padding: 0.3rem 0.45rem;
  text-align: left;
}

.ui-sidebar-menu__button:focus-visible,
.ui-sidebar-menu__sub-button:focus-visible,
.ui-sidebar-menu__action:focus-visible,
.ui-sidebar-menu__toggle:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  outline-offset: 1px;
}

.ui-sidebar-menu__button:hover,
.ui-sidebar-menu__sub-button:hover,
.ui-sidebar-menu__action:hover,
.ui-sidebar-menu__toggle:hover {
  background: color-mix(in oklab, currentColor 10%, transparent);
}

.ui-sidebar-menu__item[data-active="true"] .ui-sidebar-menu__button,
.ui-sidebar-menu__sub-button[data-active="true"] {
  background: color-mix(in oklab, var(--ui-accent-solid, currentColor) 18%, transparent);
}

.ui-sidebar-menu--disabled,
.ui-sidebar-menu[data-disabled="true"] {
  opacity: 0.62;
}

.ui-sidebar-menu--empty,
.ui-sidebar-menu[data-empty="true"] {
  min-height: 2.5rem;
}

.ui-sidebar-menu--custom-class,
.ui-sidebar-menu[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
