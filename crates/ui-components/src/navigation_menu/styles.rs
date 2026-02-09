pub const CSS: &str = r#"
.ui-navigation-menu {
  display: block;
  width: fit-content;
  max-width: 100%;
}

.ui-navigation-menu__list {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  padding: var(--ui-space-2xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: color-mix(in oklab, var(--ui-bg) 92%, var(--ui-bg-muted) 8%);
}

.ui-navigation-menu__item {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 2rem;
  padding: 0 var(--ui-space-sm);
  border-radius: var(--ui-radius-sm);
  color: var(--ui-fg);
  font-size: var(--ui-font-size-sm);
  text-decoration: none;
  transition: color 120ms ease;
  user-select: none;
}

.ui-navigation-menu__item:hover:not([data-disabled="true"]) {
  color: color-mix(in oklab, var(--ui-fg) 70%, var(--ui-accent) 30%);
}

.ui-navigation-menu__item[data-selected="true"] {
  color: var(--ui-accent-contrast);
}

.ui-navigation-menu__item:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-navigation-menu__item[data-disabled="true"] {
  opacity: 0.56;
  pointer-events: none;
}

.ui-navigation-menu--empty .ui-navigation-menu__list {
  border-color: color-mix(in oklab, var(--ui-border) 72%, var(--ui-fg-muted) 28%);
}

.ui-navigation-menu--selected .ui-navigation-menu__list {
  box-shadow: var(--ui-shadow-sm);
}
"#;
