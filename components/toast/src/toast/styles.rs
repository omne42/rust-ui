pub const CLOSE_BUTTON_CSS: &str = crate::close_button::styles::CSS;

pub const CSS: &str = r#"
.ui-toast-viewport {
  --ui-toast-viewport-offset: var(--ui-overlay-viewport-inset, 16px);
  --ui-toast-single-max-width: var(--ui-overlay-panel-min-width, 240px);
  --ui-toast-max-inline-width: calc(var(--ui-overlay-panel-min-width, 240px) + var(--ui-space-lg, 16px) * 9);
  position: fixed;
  right: var(--ui-toast-viewport-offset);
  bottom: var(--ui-toast-viewport-offset);
  z-index: var(--ui-overlay-z-index, 1100);
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  pointer-events: none;
  max-width: min(92vw, var(--ui-toast-max-inline-width));
}

.ui-toast-viewport--portal,
.ui-toast-viewport[data-state="portal"] {
  position: fixed;
}

.ui-toast-viewport--inline,
.ui-toast-viewport[data-state="inline"],
.ui-toast-viewport[data-portal="false"] {
  position: relative;
  right: auto;
  bottom: auto;
}

.ui-toast-viewport[data-motion-source="custom"],
.ui-toast-viewport[data-custom-motion="true"] {
  --ui-toast-viewport-custom-motion: 1;
}

.ui-toast-viewport[data-store-source="provided"] {
  --ui-toast-viewport-store-source: 1;
}

.ui-toast-viewport[data-store-source="context"] {
  --ui-toast-viewport-store-source: 2;
}

.ui-toast-viewport[data-store-source="local"] {
  --ui-toast-viewport-store-source: 3;
}

.ui-toast-viewport[data-queue="single"] {
  max-width: min(92vw, var(--ui-toast-single-max-width));
}

.ui-toast-viewport[data-queue="bounded"],
.ui-toast-viewport[data-queue="extended"] {
  max-width: min(92vw, var(--ui-toast-max-inline-width));
}

.ui-toast {
  pointer-events: auto;
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-sm);
  padding: var(--ui-space-md);
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);

  opacity: var(--ui-toast-opacity, 1);
  transform: translateY(var(--ui-toast-y, 0px)) scale(var(--ui-toast-scale, 1));
  transform-origin: bottom right;
  will-change: transform, opacity;
}

.ui-toast[data-motion-source="custom"],
.ui-toast[data-custom-motion="true"] {
  --ui-toast-custom-motion: 1;
}

.ui-toast--open,
.ui-toast[data-state="open"] {
  --ui-toast-open: 1;
}

.ui-toast--closing,
.ui-toast[data-state="closing"] {
  --ui-toast-open: 0;
}

.ui-toast--with-description,
.ui-toast[data-description="present"] {
  --ui-toast-description-lines: 2;
}

.ui-toast--title-only,
.ui-toast[data-description="absent"] {
  --ui-toast-description-lines: 0;
}

.ui-toast[data-id-source="custom"],
.ui-toast[data-custom-id="true"] {
  --ui-toast-custom-id: 1;
}

.ui-toast[data-description-source="custom"],
.ui-toast[data-custom-description="true"] {
  --ui-toast-custom-description: 1;
}

.ui-toast[data-close-source="custom"],
.ui-toast[data-custom-close="true"] {
  --ui-toast-custom-close: 1;
}

.ui-toast[data-exit-source="custom"],
.ui-toast[data-custom-exit="true"] {
  --ui-toast-custom-exit: 1;
}

.ui-toast--custom-class,
.ui-toast[data-custom-class="true"] {
  --ui-toast-custom-class: 1;
}

.ui-toast__content {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-2xs, 4px);
  flex: 1;
}

.ui-toast__title {
  font-weight: 700;
  font-size: var(--ui-heading-h6-font-size, 14px);
  line-height: var(--ui-heading-h6-line-height, 20px);
}

.ui-toast__description {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}

.ui-toast__close {
  all: unset;
  width: calc(var(--ui-space-lg, 16px) + var(--ui-space-xs, 8px));
  height: calc(var(--ui-space-lg, 16px) + var(--ui-space-xs, 8px));
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--ui-radius-md, 10px);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  color: var(--ui-fg-muted);
}

.ui-toast[data-close-mode="noop"] .ui-toast__close {
  cursor: default;
}

.ui-toast__close:hover {
  background: var(--ui-accent-soft);
  color: var(--ui-fg);
}

.ui-toast__close:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-toast--variant-accent,
.ui-toast[data-variant="accent"] {
  border-color: color-mix(in oklch, var(--ui-accent) 45%, var(--ui-border));
}

.ui-toast--variant-danger,
.ui-toast[data-variant="danger"] {
  border-color: color-mix(in oklch, var(--ui-danger) 45%, var(--ui-border));
}
"#;
