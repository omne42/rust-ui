pub const CSS: &str = r#"
.ui-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  --ui-checkbox-scale: 1;
  --ui-checkbox-indicator: 0;

  border: none;
  padding: 0;
  background: transparent;
  color: inherit;
  font: inherit;

  transform: scale(var(--ui-checkbox-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-checkbox[data-state="checked"] {
  --ui-checkbox-indicator: 1;
}

.ui-checkbox:not(:disabled) {
  cursor: pointer;
}

.ui-checkbox:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.ui-checkbox--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-md);
}

.ui-checkbox__box {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-accent-fg);

  box-sizing: border-box;
  transition:
    background-color 200ms ease,
    border-color 200ms ease,
    color 200ms ease;
}

@media (prefers-reduced-motion: reduce) {
  .ui-checkbox__box {
    transition: none;
  }
}

.ui-checkbox[data-state="checked"] .ui-checkbox__box {
  background: var(--ui-accent);
  border-color: var(--ui-accent);
}

.ui-checkbox[data-state="unchecked"] .ui-checkbox__box {
  background: var(--ui-bg);
  border-color: var(--ui-border);
  color: transparent;
}

.ui-checkbox--variant-accent[data-state="unchecked"] .ui-checkbox__box {
  background: var(--ui-bg-muted);
}

.ui-checkbox__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;

  opacity: var(--ui-checkbox-indicator, 0);
  transform: scale(
    calc(0.8 + (var(--ui-checkbox-indicator, 0) * 0.2))
  );
  transform-origin: center;
  will-change: transform, opacity;
}

.ui-checkbox__label {
  font-size: 14px;
  line-height: 1.2;
}

.ui-checkbox--size-default .ui-checkbox__box {
  width: 20px;
  height: 20px;
  border-radius: 4px;
}

.ui-checkbox--size-default .ui-checkbox__indicator svg {
  width: 14px;
  height: 14px;
}

.ui-checkbox--size-sm .ui-checkbox__box {
  width: 18px;
  height: 18px;
  border-radius: 5px;
}

.ui-checkbox--size-sm .ui-checkbox__indicator svg {
  width: 12px;
  height: 12px;
}

.ui-checkbox--size-lg .ui-checkbox__box {
  width: 24px;
  height: 24px;
  border-radius: 7px;
}

.ui-checkbox--size-lg .ui-checkbox__indicator svg {
  width: 16px;
  height: 16px;
}
"#;
