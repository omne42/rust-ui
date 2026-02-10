pub const CSS: &str = r#"
.ui-tabs {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ui-tabs[data-motion-source="custom"],
.ui-tabs[data-custom-motion="true"] {
  --ui-tabs-custom-motion: 1;
}

.ui-tabs__list {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px;
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  width: fit-content;
}

.ui-tabs__indicator {
  position: absolute;
  top: 4px;
  left: 4px;
  height: calc(100% - 8px);
  width: var(--ui-tabs-indicator-w, 0px);
  transform: translateX(var(--ui-tabs-indicator-x, 0px));
  opacity: var(--ui-tabs-indicator-o, 0);
  border-radius: calc(var(--ui-radius-md) - 2px);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  pointer-events: none;
  will-change: transform, width, opacity;
}

.ui-tabs__tab {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  appearance: none;
  background: transparent;
  border: 0;
  outline: none;
  color: var(--ui-fg-muted);
  padding: 8px 12px;
  border-radius: calc(var(--ui-radius-md) - 2px);
  line-height: 1;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-tabs__tab[data-selected=\"true\"] {
  color: var(--ui-fg);
}

.ui-tabs__tab[data-hovered=\"true\"]:not([data-disabled=\"true\"]) {
  color: var(--ui-fg);
}

.ui-tabs__tab[data-pressed=\"true\"]:not([data-disabled=\"true\"]) {
  transform: scale(0.98);
}

.ui-tabs__tab[data-disabled=\"true\"] {
  cursor: not-allowed;
  opacity: 0.6;
}

.ui-tabs__tab--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}
"#;
