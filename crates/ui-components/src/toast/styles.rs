pub const CSS: &str = r#"
.ui-toast-viewport {
  position: fixed;
  right: 16px;
  bottom: 16px;
  z-index: 1100;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  pointer-events: none;
  max-width: min(92vw, 420px);
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

.ui-toast--custom-class,
.ui-toast[data-custom-class="true"] {
  --ui-toast-custom-class: 1;
}

.ui-toast__content {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.ui-toast__title {
  font-weight: 700;
  font-size: 13px;
  line-height: 1.2;
}

.ui-toast__description {
  font-size: 12px;
  line-height: 1.35;
  color: var(--ui-fg-muted);
}

.ui-toast__close {
  all: unset;
  width: 28px;
  height: 28px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  color: var(--ui-fg-muted);
}

.ui-toast__close:hover {
  background: var(--ui-accent-soft);
  color: var(--ui-fg);
}

.ui-toast__close:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-toast--variant-accent {
  border-color: color-mix(in oklch, var(--ui-accent) 45%, var(--ui-border));
}

.ui-toast--variant-danger {
  border-color: color-mix(in oklch, var(--ui-danger) 45%, var(--ui-border));
}
"#;
