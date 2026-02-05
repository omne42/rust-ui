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
  position: relative;
  border-radius: var(--ui-radius-lg);
  border: 1px dashed color-mix(in oklch, var(--ui-border) 80%, var(--ui-fg-muted));
  background: var(--ui-bg);
  color: var(--ui-fg-muted);
  box-shadow: var(--ui-shadow-sm);
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  --ui-drop-zone-scale: 1;
  --ui-drop-zone-highlight: 0;

  transform: scale(var(--ui-drop-zone-scale));
  transform-origin: center;
  will-change: transform;
}

.ui-drop-zone__zone::before {
  content: "";
  position: absolute;
  inset: 0;
  background: var(--ui-accent-soft);
  opacity: var(--ui-drop-zone-highlight);
  border-radius: inherit;
  pointer-events: none;
}

.ui-drop-zone__zone[data-hovered="true"],
.ui-drop-zone__zone[data-drop-target="true"] {
  color: var(--ui-fg);
}

.ui-drop-zone__zone[data-drop-target="true"] {
  border-color: color-mix(in oklch, var(--ui-accent) 60%, var(--ui-border));
}

.ui-drop-zone__zone[data-disabled="true"] {
  opacity: 0.5;
  pointer-events: none;
}

.ui-drop-zone__zone[data-focus-visible="true"] {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-drop-zone__button {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
  pointer-events: none;
}
"#;
