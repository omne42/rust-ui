pub const CSS: &str = r#"
.ui-sidebar-menu {
  --ui-sidebar-menu-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-sidebar-menu-radius-xs: var(
    --ui-radius-xs,
    var(--ui-radius-sm, var(--ui-fallback-radius-sm))
  );
  --ui-sidebar-menu-font-size-100: var(
    --ui-font-size-100,
    var(--ui-fallback-font-size-100)
  );
  --ui-sidebar-menu-line-height-100: var(
    --ui-line-height-100,
    var(--ui-fallback-line-height-100)
  );
  --ui-sidebar-menu-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-sidebar-menu-accent-solid: var(
    --ui-accent-solid,
    var(--ui-accent, var(--ui-fallback-accent))
  );
  --ui-sidebar-menu-border-subtle: var(
    --ui-border-subtle,
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 22%, transparent)
  );
  --ui-sidebar-menu-hover-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 44%,
    transparent
  );
  --ui-sidebar-menu-badge-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 52%,
    transparent
  );
  --ui-sidebar-menu-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-menu-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-menu-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
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
  border-radius: var(--ui-sidebar-menu-radius-sm);
  background: color-mix(in oklab, var(--ui-sidebar-menu-accent-solid) 15%, transparent);
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
  border-radius: var(--ui-sidebar-menu-radius-sm);
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
  color: var(--ui-sidebar-menu-fg-muted);
  font-size: var(--ui-sidebar-menu-font-size-100);
  line-height: var(--ui-sidebar-menu-line-height-100);
}

.ui-sidebar-menu__badge {
  margin-inline-start: auto;
  padding-inline: 0.45rem;
  border-radius: 999px;
  background: var(--ui-sidebar-menu-badge-bg);
  font-size: var(--ui-sidebar-menu-font-size-100);
  line-height: var(--ui-sidebar-menu-line-height-100);
  font-variant-numeric: tabular-nums;
}

.ui-sidebar-menu__action,
.ui-sidebar-menu__toggle {
  border-radius: var(--ui-sidebar-menu-radius-xs);
  padding: 0.25rem 0.35rem;
  line-height: 1;
}

.ui-sidebar-menu__toggle {
  transform-origin: center;
  transition:
    transform var(--ui-sidebar-menu-motion-duration) var(--ui-sidebar-menu-motion-easing);
}

.ui-sidebar-menu__toggle[data-open="true"] {
  transform: rotate(90deg);
}

.ui-sidebar-menu__sub {
  margin-inline-start: 0.95rem;
  padding-inline-start: 0.6rem;
  border-inline-start: 1px solid var(--ui-sidebar-menu-border-subtle);
  display: grid;
  gap: 0.1rem;
}

.ui-sidebar-menu__sub-button {
  display: flex;
  align-items: center;
  gap: 0.45rem;
  width: 100%;
  border-radius: var(--ui-sidebar-menu-radius-xs);
  padding: 0.3rem 0.45rem;
  text-align: left;
}

.ui-sidebar-menu__button:focus-visible,
.ui-sidebar-menu__sub-button:focus-visible,
.ui-sidebar-menu__action:focus-visible,
.ui-sidebar-menu__toggle:focus-visible {
  outline: 2px solid var(--ui-sidebar-menu-accent-solid);
  outline-offset: 1px;
}

.ui-sidebar-menu__button:hover,
.ui-sidebar-menu__sub-button:hover,
.ui-sidebar-menu__action:hover,
.ui-sidebar-menu__toggle:hover {
  background: var(--ui-sidebar-menu-hover-bg);
}

.ui-sidebar-menu__item[data-active="true"] .ui-sidebar-menu__button,
.ui-sidebar-menu__sub-button[data-active="true"] {
  background: color-mix(in oklab, var(--ui-sidebar-menu-accent-solid) 18%, transparent);
}

.ui-sidebar-menu--disabled,
.ui-sidebar-menu[data-disabled="true"] {
  opacity: var(--ui-sidebar-menu-disabled-opacity);
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
