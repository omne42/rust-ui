pub const CSS: &str = r#"
.ui-tabs {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ui-tabs__list {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px;
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  width: fit-content;
}

.ui-tabs__tab {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  appearance: none;
  background: transparent;
  border: 0;
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

.ui-tabs__tab[data-active=\"true\"] {
  color: var(--ui-fg);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-tabs__tab[data-active=\"true\"]::after {
  content: \"\";
  position: absolute;
  left: 10px;
  right: 10px;
  bottom: 4px;
  height: 2px;
  border-radius: 999px;
  background: var(--ui-accent);
}

.ui-tabs__tab:focus {
  outline: none;
}

.ui-tabs__tab:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}
"#;
