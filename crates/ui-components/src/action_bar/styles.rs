pub const CSS: &str = r#"
.ui-action-bar {
  --ui-action-bar-translate-y: 0px;
  --ui-action-bar-opacity: 1;

  position: fixed;
  left: 50%;
  z-index: 80;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-sm);
  min-height: 2.5rem;
  max-width: min(46rem, calc(100vw - var(--ui-space-xl) * 2));
  padding: var(--ui-space-sm) var(--ui-space-md);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: color-mix(in oklab, var(--ui-bg-muted) 92%, var(--ui-bg) 8%);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);
  transform: translateX(-50%) translateY(var(--ui-action-bar-translate-y));
  opacity: var(--ui-action-bar-opacity);
  pointer-events: auto;
}

.ui-action-bar--position-bottom,
.ui-action-bar[data-position="bottom"] {
  bottom: calc(var(--ui-space-xl) + env(safe-area-inset-bottom));
}

.ui-action-bar--position-top,
.ui-action-bar[data-position="top"] {
  top: calc(var(--ui-space-xl) + env(safe-area-inset-top));
}

.ui-action-bar--state-hidden,
.ui-action-bar[data-state="hidden"],
.ui-action-bar[data-hidden="true"] {
  pointer-events: none;
}

.ui-action-bar__selection {
  display: inline-flex;
  align-items: baseline;
  gap: var(--ui-space-2xs);
  margin: 0;
  color: var(--ui-fg);
  font-size: var(--ui-font-size-150, 14px);
  font-weight: 600;
  line-height: var(--ui-line-height-150, 20px);
  white-space: nowrap;
}

.ui-action-bar__selection-count {
  font-variant-numeric: tabular-nums;
  opacity: 0.92;
}

.ui-action-bar__actions {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-action-bar__clear {
  border: 0;
  padding: 0;
  color: var(--ui-accent);
  background: transparent;
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
  text-decoration: underline;
  text-underline-offset: 0.12em;
  cursor: pointer;
}

.ui-action-bar__clear:hover {
  color: color-mix(in oklab, var(--ui-accent) 84%, white 16%);
}

.ui-action-bar__clear:focus-visible {
  outline: 2px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-action-bar--selection-empty .ui-action-bar__selection,
.ui-action-bar[data-selection="empty"] .ui-action-bar__selection {
  color: var(--ui-fg-muted);
}

.ui-action-bar--selection-single .ui-action-bar__selection,
.ui-action-bar[data-selection="single"] .ui-action-bar__selection {
  color: color-mix(in oklab, var(--ui-fg) 94%, var(--ui-accent) 6%);
}

.ui-action-bar--selection-multiple .ui-action-bar__selection,
.ui-action-bar[data-selection="multiple"] .ui-action-bar__selection {
  color: color-mix(in oklab, var(--ui-fg) 86%, var(--ui-accent) 14%);
}

.ui-action-bar--clearable,
.ui-action-bar[data-has-clear="true"] {
  gap: var(--ui-space-md);
}

.ui-action-bar--label-custom,
.ui-action-bar[data-label-source="custom"] {
  border: 1px dashed var(--ui-border);
}

.ui-action-bar--selection-custom,
.ui-action-bar[data-selection-source="custom"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 32%, var(--ui-bg-muted) 68%);
}

.ui-action-bar--clear-label-custom,
.ui-action-bar[data-clear-label-source="custom"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent), var(--ui-shadow-md);
}

.ui-action-bar--motion-custom,
.ui-action-bar[data-motion-source="custom"] {
  backdrop-filter: blur(8px);
}

.ui-action-bar--custom-class,
.ui-action-bar[data-custom-class="true"] {
  border-width: 2px;
}

@media (max-width: 640px) {
  .ui-action-bar {
    max-width: calc(100vw - var(--ui-space-md) * 2);
    gap: var(--ui-space-xs);
    padding: var(--ui-space-xs) var(--ui-space-sm);
  }

  .ui-action-bar__selection {
    font-size: var(--ui-button-size-s-font-size, 13px);
    line-height: var(--ui-button-size-s-line-height, 18px);
  }
}
"#;
