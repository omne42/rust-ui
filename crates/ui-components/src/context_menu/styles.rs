pub const CSS: &str = r#"
.ui-context-menu {
  display: inline-flex;
  align-items: stretch;
}

.ui-context-menu__trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: min(100%, 22rem);
  min-height: 7.5rem;
  border: 1px dashed color-mix(in oklab, var(--ui-border) 82%, var(--ui-accent) 18%);
  border-radius: var(--ui-radius-lg);
  background: color-mix(in oklab, var(--ui-bg) 90%, var(--ui-bg-muted) 10%);
  color: var(--ui-fg-muted);
  font: inherit;
  padding: var(--ui-space-md);
  text-align: center;
  cursor: context-menu;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-context-menu__trigger:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-context-menu--disabled .ui-context-menu__trigger,
.ui-context-menu__trigger:disabled {
  opacity: 0.58;
  cursor: not-allowed;
}

.ui-context-menu--persistent .ui-context-menu__trigger {
  box-shadow: var(--ui-shadow-sm);
}

.ui-context-menu--empty .ui-context-menu__trigger {
  border: 1px dotted color-mix(in oklab, var(--ui-border) 82%, var(--ui-accent) 18%);
}
"#;
