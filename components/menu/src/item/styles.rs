pub const CSS: &str = r#"
.ui-menu-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  width: 100%;
  padding: var(--ui-space-xs, var(--ui-fallback-space-xs))
    var(--ui-space-sm, var(--ui-fallback-space-sm));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  outline: none;
}

.ui-menu-item--kind-action,
.ui-menu-item[data-kind="action"] {
  font-weight: 500;
}

.ui-menu-item--kind-checkbox,
.ui-menu-item[data-kind="checkbox"],
.ui-menu-item--kind-radio,
.ui-menu-item[data-kind="radio"] {
  font-weight: 500;
}

.ui-menu-item--checkable,
.ui-menu-item[data-checkable="true"] {
  padding-inline-start: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-menu-item--checked,
.ui-menu-item[data-checked="true"] {
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 92%,
    var(--ui-accent, var(--ui-fallback-accent)) 8%
  );
}

.ui-menu-item--focused,
.ui-menu-item[data-focused="true"] {
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 82%,
    var(--ui-accent, var(--ui-fallback-accent)) 18%
  );
}

.ui-menu-item--disabled,
.ui-menu-item[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
  cursor: not-allowed;
}

.ui-menu-item__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-icon-size-100, var(--ui-fallback-icon-size-100));
  min-height: 1lh;
  flex-shrink: 0;
}

.ui-menu-item__label {
  min-width: 0;
  flex: 1;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-menu-item__submenu-indicator {
  margin-inline-start: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-icon-size-100, var(--ui-fallback-icon-size-100));
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 88%,
    var(--ui-accent, var(--ui-fallback-accent)) 12%
  );
}

.ui-menu-item__selection-sr {
  position: absolute;
  width: var(--ui-border-width, var(--ui-fallback-border-width));
  height: var(--ui-border-width, var(--ui-fallback-border-width));
  margin: calc(-1 * var(--ui-border-width, var(--ui-fallback-border-width)));
  padding: 0;
  border: 0;
  overflow: hidden;
  clip: rect(0 0 0 0);
  white-space: nowrap;
}

.ui-menu-item--submenu,
.ui-menu-item[data-has-submenu="true"] {
  padding-inline-end: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-menu-item--custom-class,
.ui-menu-item[data-custom-class="true"] {
  box-shadow: 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 70%,
      var(--ui-accent, var(--ui-fallback-accent)) 30%
    ) inset;
}
"#;
