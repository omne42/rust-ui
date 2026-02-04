pub const CSS: &str = r#"
.ui-hover-card {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-hover-card__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
  cursor: default;
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
  transform-origin: top left;
  will-change: transform, opacity;
}
"#;
