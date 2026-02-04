pub const CSS: &str = r#"
.ui-drop-zone {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-drop-zone__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-drop-zone__zone {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 120px;
  padding: var(--ui-space-lg);
  border-radius: var(--ui-radius-lg);
  border: 1px dashed color-mix(in oklch, var(--ui-border) 80%, var(--ui-fg-muted));
  background: var(--ui-bg);
  color: var(--ui-fg-muted);
  box-shadow: var(--ui-shadow-sm);
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-drop-zone__zone[data-drag-over="true"] {
  border-color: color-mix(in oklch, var(--ui-accent) 60%, var(--ui-border));
  background: var(--ui-accent-soft);
  color: var(--ui-fg);
}

.ui-drop-zone__zone[data-disabled="true"] {
  opacity: 0.5;
  pointer-events: none;
}

.ui-drop-zone__zone:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}
"#;
