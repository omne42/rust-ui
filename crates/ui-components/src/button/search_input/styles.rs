pub const CSS: &str = r#"
.ui-search-input-button {
  --ui-search-input-button-scale: var(--ui-button-scale, 1);
  --ui-search-input-button-placeholder-color: var(--ui-fg-muted);

  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  height: 36px;
  padding: 0 var(--ui-space-md);
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  transform: scale(var(--ui-search-input-button-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-search-input-button--enabled,
.ui-search-input-button[data-state="enabled"],
.ui-search-input-button[data-enabled="true"] {
  cursor: pointer;
}

.ui-search-input-button--disabled,
.ui-search-input-button[data-state="disabled"],
.ui-search-input-button[data-disabled="true"],
.ui-search-input-button:disabled {
  opacity: 0.5;
  pointer-events: none;
}

.ui-search-input-button--custom-class,
.ui-search-input-button[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-search-input-button[data-motion-source="custom"],
.ui-search-input-button[data-custom-motion="true"] {
  --ui-search-input-button-custom-motion: 1;
}

.ui-search-input-button--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-search-input-button--custom-placeholder,
.ui-search-input-button[data-placeholder="custom"] {
  --ui-search-input-button-placeholder-color: var(--ui-fg);
}

.ui-search-input-button--custom-compact-placeholder .ui-search-input-button__placeholder--compact,
.ui-search-input-button[data-compact-placeholder="custom"] .ui-search-input-button__placeholder--compact {
  font-weight: 600;
}

.ui-search-input-button__icon {
  width: 14px;
  height: 14px;
  color: var(--ui-fg-muted);
  flex-shrink: 0;
}

.ui-search-input-button__placeholder {
  font-size: 12px;
  color: var(--ui-search-input-button-placeholder-color);
  white-space: nowrap;
}

.ui-search-input-button__placeholder--full {
  display: none;
}

.ui-search-input-button__placeholder--compact {
  display: inline-flex;
}

.ui-search-input-button__shortcut {
  margin-left: auto;
  display: none;
  align-items: center;
  gap: 4px;
}

.ui-search-input-button__key {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--ui-radius-md) - 2px);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  padding: 2px 6px;
  font-size: 10px;
  font-weight: 600;
  line-height: 1;
  color: var(--ui-fg-muted);
}

@media (min-width: 640px) {
  .ui-search-input-button__placeholder--full {
    display: inline-flex;
  }

  .ui-search-input-button__placeholder--compact {
    display: none;
  }

  .ui-search-input-button--with-shortcut .ui-search-input-button__shortcut,
  .ui-search-input-button[data-shortcut="visible"] .ui-search-input-button__shortcut {
    display: inline-flex;
  }
}
"#;
