pub const CSS: &str = r#"
.ui-tooltip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
}

.ui-tooltip[data-state="open"],
.ui-tooltip[data-open="true"],
.ui-tooltip[data-state="closed"],
.ui-tooltip[data-closed="true"] {
  cursor: default;
}

.ui-tooltip[data-disabled="true"] {
  opacity: 0.72;
}

.ui-tooltip[data-trigger="focus"] {
  --ui-tooltip-trigger: focus;
}

.ui-tooltip[data-press-behavior="persist"] {
  --ui-tooltip-press-behavior: persist;
}

.ui-tooltip[data-class-source="custom"],
.ui-tooltip--custom-class {
  --ui-tooltip-class-source: custom;
}

.ui-tooltip[data-motion-source="custom"],
.ui-tooltip[data-custom-motion="true"],
.ui-tooltip--custom-motion {
  --ui-tooltip-custom-motion: 1;
}

.ui-tooltip[data-delay-source="custom"],
.ui-tooltip--custom-delay {
  --ui-tooltip-delay-source: custom;
}

.ui-tooltip[data-trigger-source="custom"],
.ui-tooltip--custom-trigger {
  --ui-tooltip-trigger-source: custom;
}

.ui-tooltip[data-press-source="custom"],
.ui-tooltip--custom-press {
  --ui-tooltip-press-source: custom;
}

.ui-tooltip[data-id-source="custom"],
.ui-tooltip--custom-id {
  --ui-tooltip-id-source: custom;
}

.ui-tooltip__panel {
  position: fixed;
  top: var(--ui-tooltip-top, 0px);
  left: var(--ui-tooltip-left, 0px);

  --ui-tooltip-opacity: 0;
  --ui-tooltip-scale: 0.98;
  --ui-tooltip-y: 6px;

  opacity: var(--ui-tooltip-opacity);
  transform: translateY(var(--ui-tooltip-y)) scale(var(--ui-tooltip-scale));
  will-change: transform, opacity;

  pointer-events: none;
  z-index: 1100;

  padding: 8px 10px;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);

  font-size: 12px;
  line-height: 1.2;
  max-width: 280px;
}

.ui-tooltip__panel[data-state="panel"] {
  --ui-tooltip-panel: 1;
}

.ui-tooltip__panel[data-placement="bottom"] {
  transform-origin: top center;
}

.ui-tooltip__panel[data-placement="top"] {
  transform-origin: bottom center;
}
"#;
