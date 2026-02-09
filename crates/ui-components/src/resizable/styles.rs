pub const CSS: &str = r#"
.ui-resizable {
  --ui-resizable-split: 50%;
  --ui-resizable-handle-size: 1px;
  --ui-resizable-hit-size: 0.5rem;
  display: flex;
  width: 100%;
  min-width: 0;
  min-height: 0;
  border: 1px solid var(--ui-border-subtle, color-mix(in oklab, currentColor 18%, transparent));
  border-radius: var(--ui-radius-md, 0.75rem);
  overflow: hidden;
  background: var(--ui-bg-surface, color-mix(in oklab, currentColor 4%, transparent));
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
  background: var(--ui-border-subtle, color-mix(in oklab, currentColor 22%, transparent));
  color: var(--ui-fg-muted, color-mix(in oklab, currentColor 72%, transparent));
  cursor: col-resize;
  outline: none;
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
  gap: 0.125rem;
  z-index: 1;
}

.ui-resizable[data-handle="with-handle"] .ui-resizable__handle-grip,
.ui-resizable__handle[data-with-handle="true"] .ui-resizable__handle-grip {
  display: inline-flex;
}

.ui-resizable__handle-dot {
  width: 0.1875rem;
  height: 0.1875rem;
  border-radius: 999px;
  background: currentColor;
}

.ui-resizable[data-orientation="vertical"] .ui-resizable__handle-grip {
  transform: rotate(90deg);
}

.ui-resizable__handle:focus-visible,
.ui-resizable[data-state="dragging"] .ui-resizable__handle,
.ui-resizable__handle[data-dragging="true"] {
  background: var(--ui-accent-solid, color-mix(in oklab, currentColor 52%, transparent));
  color: var(--ui-accent-contrast, white);
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
