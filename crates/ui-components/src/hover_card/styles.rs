pub const CSS: &str = r#"
.ui-hover-card {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-hover-card[data-state="open"],
.ui-hover-card[data-open="true"],
.ui-hover-card[data-state="closed"],
.ui-hover-card[data-closed="true"] {
  cursor: default;
}

.ui-hover-card[data-disabled="true"] {
  opacity: 0.72;
}

.ui-hover-card[data-class-source="custom"],
.ui-hover-card--custom-class {
  --ui-hover-card-class-source: custom;
}

.ui-hover-card[data-motion-source="custom"],
.ui-hover-card[data-custom-motion="true"],
.ui-hover-card--custom-motion {
  --ui-hover-card-custom-motion: 1;
}

.ui-hover-card[data-delay-source="custom"],
.ui-hover-card--custom-delay {
  --ui-hover-card-delay-source: custom;
}

.ui-hover-card[data-id-source="custom"],
.ui-hover-card--custom-id {
  --ui-hover-card-id-source: custom;
}

.ui-hover-card__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
  cursor: default;
}

.ui-hover-card__trigger[data-state="trigger"] {
  --ui-hover-card-trigger: 1;
}

.ui-hover-card__trigger:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 3px;
  border-radius: var(--ui-radius-md);
}

.ui-hover-card__panel {
  position: fixed;
  top: var(--ui-hover-card-top, 0px);
  left: var(--ui-hover-card-left, 0px);
  min-width: max(260px, var(--ui-hover-card-anchor-width, 0px));
  max-width: min(92vw, 420px);
  padding: var(--ui-space-md);
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);
  z-index: 1000;

  opacity: var(--ui-hover-card-opacity, 0);
  transform: translateY(var(--ui-hover-card-y, 8px)) scale(var(--ui-hover-card-scale, 0.98));
  will-change: transform, opacity;
}

.ui-hover-card__panel[data-state="panel"] {
  --ui-hover-card-panel: 1;
}

.ui-hover-card__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-hover-card__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-hover-card__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-hover-card__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}
"#;
