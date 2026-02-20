pub const CSS: &str = r#"
.ui-menu-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 6px 10px;
  border-radius: 10px;
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
  padding-inline-start: 8px;
}

.ui-menu-item--checked,
.ui-menu-item[data-checked="true"] {
  color: color-mix(in oklab, var(--ui-fg) 92%, var(--ui-accent) 8%);
}

.ui-menu-item--focused,
.ui-menu-item[data-focused="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 82%, var(--ui-accent) 18%);
}

.ui-menu-item--disabled,
.ui-menu-item[data-disabled="true"] {
  opacity: 0.52;
  cursor: not-allowed;
}

.ui-menu-item__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  min-height: 1lh;
  flex-shrink: 0;
}

.ui-menu-item__label {
  min-width: 0;
  flex: 1;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.ui-menu-item__submenu-indicator {
  margin-inline-start: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  color: color-mix(in oklab, var(--ui-fg-muted) 88%, var(--ui-accent) 12%);
}

.ui-menu-item__selection-sr {
  position: absolute;
  width: 1px;
  height: 1px;
  margin: -1px;
  padding: 0;
  border: 0;
  overflow: hidden;
  clip: rect(0 0 0 0);
  white-space: nowrap;
}

.ui-menu-item--submenu,
.ui-menu-item[data-has-submenu="true"] {
  padding-inline-end: 12px;
}

.ui-menu-item--custom-class,
.ui-menu-item[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-border) 70%, var(--ui-accent) 30%) inset;
}
"#;
