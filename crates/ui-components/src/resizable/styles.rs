pub const CSS: &str = r#"
.ui-resizable {
  --ui-resizable-split: 50%;
  --ui-resizable-handle-size: 1px;
  --ui-resizable-hit-size: var(--ui-space-xs, 8px);
  --ui-resizable-panel-duration: var(--ui-text-field-motion-duration, 180ms);
  --ui-resizable-handle-duration: var(--ui-text-field-motion-duration, 180ms);
  --ui-resizable-motion-easing: var(--ui-text-field-motion-easing, cubic-bezier(0.2, 0, 0, 1));
  display: flex;
  width: 100%;
  min-width: 0;
  min-height: 0;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  overflow: hidden;
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
}

.ui-resizable[data-orientation="vertical"] {
  flex-direction: column;
}

.ui-resizable__panel {
  min-inline-size: 0;
  min-block-size: 0;
  overflow: auto;
}

.ui-resizable__panel--first {
  flex: 0 0 var(--ui-resizable-split);
  transition: flex-basis var(--ui-resizable-runtime-panel-duration, var(--ui-resizable-panel-duration)) var(--ui-resizable-motion-easing);
}

.ui-resizable__panel--second {
  flex: 1 1 0%;
}

.ui-resizable__handle {
  position: relative;
  flex: 0 0 var(--ui-resizable-handle-size);
  border: 0;
  padding: 0;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--ui-border);
  color: var(--ui-fg-muted);
  cursor: col-resize;
  outline: none;
  transition:
    background-color var(--ui-resizable-runtime-handle-duration, var(--ui-resizable-handle-duration)) var(--ui-resizable-motion-easing),
    color var(--ui-resizable-runtime-handle-duration, var(--ui-resizable-handle-duration)) var(--ui-resizable-motion-easing);
}

.ui-resizable[data-orientation="vertical"] .ui-resizable__handle {
  cursor: row-resize;
}

.ui-resizable__handle::after {
  content: "";
  position: absolute;
  inset-block: 0;
  left: 50%;
  width: var(--ui-resizable-hit-size);
  transform: translateX(-50%);
}

.ui-resizable[data-orientation="vertical"] .ui-resizable__handle::after {
  inset-inline: 0;
  top: 50%;
  left: auto;
  width: auto;
  height: var(--ui-resizable-hit-size);
  transform: translateY(-50%);
}

.ui-resizable__handle-grip {
  display: none;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-3xs, 2px);
  z-index: 1;
}

.ui-resizable[data-handle="with-handle"] .ui-resizable__handle-grip,
.ui-resizable__handle[data-with-handle="true"] .ui-resizable__handle-grip {
  display: inline-flex;
}

.ui-resizable__handle-dot {
  width: var(--ui-space-3xs, 2px);
  height: var(--ui-space-3xs, 2px);
  border-radius: 999px;
  background: currentColor;
}

.ui-resizable[data-orientation="vertical"] .ui-resizable__handle-grip {
  transform: rotate(90deg);
}

.ui-resizable__handle:focus-visible,
.ui-resizable[data-state="dragging"] .ui-resizable__handle,
.ui-resizable__handle[data-dragging="true"] {
  background: var(--ui-accent);
  color: var(--ui-accent-fg);
}

.ui-resizable--disabled,
.ui-resizable[data-disabled="true"] {
  opacity: 0.72;
}

.ui-resizable--disabled .ui-resizable__handle,
.ui-resizable__handle[data-disabled="true"] {
  cursor: not-allowed;
}

.ui-resizable--custom-class,
.ui-resizable[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
