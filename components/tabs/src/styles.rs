pub const CSS: &str = r#"
.ui-tabs {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
}

.ui-tabs[data-motion-source="custom"],
.ui-tabs[data-custom-motion="true"] {
  --ui-tabs-custom-motion: 1;
}

.ui-tabs__list {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  padding: var(--ui-space-2xs);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  box-shadow: var(--ui-shadow-sm);
  width: fit-content;
}

.ui-tabs__indicator {
  position: absolute;
  top: var(--ui-space-2xs);
  left: var(--ui-space-2xs);
  height: calc(100% - (var(--ui-space-2xs) * 2));
  width: var(--ui-tabs-indicator-w, 0px);
  transform: translateX(var(--ui-tabs-indicator-x, 0px));
  opacity: var(--ui-tabs-indicator-o, 0);
  border-radius: var(--ui-radius-sm);
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
  padding: var(--ui-space-sm) var(--ui-space-md);
  border-radius: var(--ui-radius-sm);
  line-height: 1;
  font-size: var(--ui-font-size-150);
  font-weight: 500;
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  transition:
    color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),
    background-color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),
    transform var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing);
}

.ui-tabs__tab[data-selected=\"true\"] {
  color: var(--ui-fg);
  font-weight: 600;
}

.ui-tabs__tab[data-hovered=\"true\"]:not([data-disabled=\"true\"]) {
  color: var(--ui-fg);
  background: var(--ui-accent-soft);
}

.ui-tabs__tab[data-pressed=\"true\"]:not([data-disabled=\"true\"]) {
  transform: scale(0.98);
}

.ui-tabs__tab[data-disabled=\"true\"] {
  cursor: not-allowed;
  opacity: 0.6;
}

.ui-tabs__tab--focus-visible {
  outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-button-focus-outline-offset);
}

.ui-tabs__panel {
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  padding: var(--ui-space-md);
}
"#;
