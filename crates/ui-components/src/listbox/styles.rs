pub const CSS: &str = r#"
.ui-listbox {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;
  outline: none;
}

.ui-listbox--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-listbox__options {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ui-listbox__option {
  position: relative;
  z-index: 1;
  padding: 6px 8px;
  border-radius: 8px;
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-listbox__option[data-selected=\"true\"] {
  font-weight: 600;
}

.ui-listbox__option[data-disabled=\"true\"] {
  opacity: 0.5;
}
"#;
