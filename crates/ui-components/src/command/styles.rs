pub const CSS: &str = r#"
.ui-command {
  width: min(100%, 30rem);
  display: flex;
  flex-direction: column;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  overflow: hidden;
}

.ui-command--disabled,
.ui-command[data-disabled="true"] {
  opacity: 0.64;
}

.ui-command__input-wrap {
  padding: 8px;
  border-bottom: 1px solid color-mix(in oklab, var(--ui-border) 86%, var(--ui-bg) 14%);
  background: color-mix(in oklab, var(--ui-bg) 94%, var(--ui-bg-muted) 6%);
}

.ui-command__input {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  font: inherit;
  line-height: 1.35;
  padding: 9px 12px;
  outline: none;
}

.ui-command__input:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 1px;
}

.ui-command__list {
  max-height: 21rem;
  overflow: auto;
}

.ui-command__options {
  position: relative;
  padding: 6px;
}

.ui-command__group {
  display: grid;
  gap: 4px;
}

.ui-command__group + .ui-command__group {
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid color-mix(in oklab, var(--ui-border) 84%, var(--ui-bg) 16%);
}

.ui-command__group-heading {
  margin: 0;
  padding-inline: 10px;
  font-size: 12px;
  line-height: 1.2;
  font-weight: 600;
  color: var(--ui-fg-muted);
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

.ui-command__group-items {
  display: grid;
  gap: 2px;
}

.ui-command__option {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 9px 10px;
  border-radius: var(--ui-radius-md);
  user-select: none;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

.ui-command__option[data-focused="true"] {
  color: color-mix(in oklab, var(--ui-fg) 96%, var(--ui-accent) 4%);
}

.ui-command__option[data-selected="true"] .ui-command__item-label {
  font-weight: 600;
}

.ui-command__option[data-disabled="true"] {
  opacity: 0.52;
  cursor: not-allowed;
}

.ui-command__shortcut {
  border: 1px solid color-mix(in oklab, var(--ui-border) 84%, var(--ui-bg-muted) 16%);
  border-radius: var(--ui-radius-sm);
  padding-inline: 6px;
  padding-block: 2px;
  font-size: 11px;
  line-height: 1;
  color: var(--ui-fg-muted);
  background: color-mix(in oklab, var(--ui-bg) 88%, var(--ui-bg-muted) 12%);
}

.ui-command__empty {
  padding: 14px 12px;
  color: var(--ui-fg-muted);
  font-size: 13px;
  line-height: 1.3;
}
"#;
