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
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
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
  top: var(--ui-tooltip-top, var(--ui-fallback-min-inline-size-none));
  left: var(--ui-tooltip-left, var(--ui-fallback-min-inline-size-none));

  --ui-tooltip-opacity: 0;
  --ui-tooltip-scale: var(
    --ui-overlay-enter-scale,
    var(--ui-fallback-overlay-enter-scale)
  );
  --ui-tooltip-y: var(
    --ui-overlay-enter-offset-y,
    var(--ui-fallback-overlay-enter-offset-y)
  );

  opacity: var(--ui-tooltip-opacity);
  transform: translateY(var(--ui-tooltip-y)) scale(var(--ui-tooltip-scale));
  will-change: transform, opacity;

  pointer-events: none;
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));

  padding: var(--ui-space-sm, var(--ui-fallback-space-sm))
    var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));

  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  max-width: var(
    --ui-tooltip-max-width,
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))
  );
}

.ui-tooltip__panel[data-state="panel"] {
  --ui-tooltip-panel: 1;
}

.ui-tooltip__panel[data-motion-source="custom"],
.ui-tooltip__panel[data-custom-motion="true"] {
  --ui-tooltip-custom-motion: 1;
}

.ui-tooltip__panel[data-delay-source="custom"],
.ui-tooltip__panel[data-custom-delay="true"] {
  --ui-tooltip-delay-source: custom;
}

.ui-tooltip__panel[data-trigger-source="custom"],
.ui-tooltip__panel[data-custom-trigger="true"] {
  --ui-tooltip-trigger-source: custom;
}

.ui-tooltip__panel[data-press-source="custom"],
.ui-tooltip__panel[data-custom-press="true"] {
  --ui-tooltip-press-source: custom;
}

.ui-tooltip__panel[data-id-source="custom"],
.ui-tooltip__panel[data-custom-id="true"] {
  --ui-tooltip-id-source: custom;
}

.ui-tooltip__panel[data-placement="bottom"] {
  transform-origin: top center;
}

.ui-tooltip__panel[data-placement="top"] {
  transform-origin: bottom center;
}
"#;
