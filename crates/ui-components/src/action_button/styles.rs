pub const CSS: &str = r#"
.ui-action-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-sm);
  white-space: nowrap;
  position: relative;
  border-radius: var(--ui-radius-md);
  border: 1px solid transparent;
  box-sizing: border-box;
  line-height: 1;
  font-weight: 600;
  font-size: 13px;
  user-select: none;
  -webkit-tap-highlight-color: transparent;

  transform: scale(var(--ui-action-button-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-action-button[data-motion-source="custom"],
.ui-action-button[data-custom-motion="true"] {
  --ui-action-button-custom-motion: 1;
}

.ui-action-button:not(:disabled) {
  cursor: pointer;
}

.ui-action-button:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-action-button--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-action-button__spinner {
  width: 16px;
  height: 16px;
  border-radius: 9999px;
  border: 2px solid currentColor;
  border-top-color: transparent;
  box-sizing: border-box;

  animation: ui-action-button-spin 0.8s linear infinite;
}

.ui-action-button[data-loading-placement=\"center\"] .ui-action-button__spinner {
  position: absolute;
  left: 50%;
  top: 50%;
  margin-left: -8px;
  margin-top: -8px;
}

.ui-action-button[data-loading=\"true\"][data-loading-placement=\"center\"] .ui-action-button__label {
  visibility: hidden;
}

@keyframes ui-action-button-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-action-button__spinner {
    animation: none;
  }
}

.ui-action-button__start,
.ui-action-button__end {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-action-button__label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-space-xs);
}

.ui-action-button svg {
  pointer-events: none;
  flex-shrink: 0;
}

.ui-action-button--size-xs {
  height: 28px;
  padding: 0 10px;
  border-radius: calc(var(--ui-radius-md) - 2px);
  font-size: 12px;
}

.ui-action-button--size-s {
  height: 32px;
  padding: 0 12px;
}

.ui-action-button--size-m {
  height: 36px;
  padding: 0 14px;
}

.ui-action-button--size-l {
  height: 40px;
  padding: 0 16px;
}

.ui-action-button--size-xl {
  height: 44px;
  padding: 0 18px;
  font-size: 14px;
}

.ui-action-button--icon-only {
  width: var(--ui-action-button-icon-size, 36px);
  padding: 0;
}

.ui-action-button--filled {
  background: var(--ui-bg);
  color: var(--ui-fg);
  border-color: var(--ui-border);
  box-shadow: var(--ui-shadow-sm);
}

.ui-action-button--filled[data-hovered=\"true\"] {
  background: var(--ui-bg-muted);
}

.ui-action-button--quiet {
  background: transparent;
  color: var(--ui-fg);
  border-color: transparent;
}

.ui-action-button--quiet[data-hovered=\"true\"] {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
}
"#;
