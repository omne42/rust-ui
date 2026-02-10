pub const CSS: &str = r#"
.ui-menubar {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs);
  min-height: 2.25rem;
  padding: var(--ui-space-2xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: color-mix(in oklab, var(--ui-bg) 92%, var(--ui-bg-muted) 8%);
  box-shadow: var(--ui-shadow-xs);
}

.ui-menubar[data-motion-source="custom"],
.ui-menubar[data-custom-motion="true"] {
  --ui-menubar-custom-motion: 1;
}

.ui-menubar__menu {
  position: relative;
  display: inline-flex;
}

.ui-menubar__trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 2rem;
  padding: 0 var(--ui-space-sm);
  border: 1px solid transparent;
  border-radius: var(--ui-radius-sm);
  background: transparent;
  color: var(--ui-fg);
  font: inherit;
  font-size: var(--ui-font-size-sm);
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-menubar__trigger:hover:not(:disabled),
.ui-menubar__menu[data-open="true"] .ui-menubar__trigger {
  border-color: color-mix(in oklab, var(--ui-border) 80%, var(--ui-accent) 20%);
  background: color-mix(in oklab, var(--ui-bg-muted) 78%, var(--ui-accent) 22%);
}

.ui-menubar__trigger:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-menubar__trigger:disabled,
.ui-menubar__menu[data-disabled="true"] .ui-menubar__trigger {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-menubar--empty {
  border-color: color-mix(in oklab, var(--ui-border) 65%, var(--ui-fg-muted) 35%);
}

.ui-menubar--open {
  box-shadow: var(--ui-shadow-sm);
}
"#;
