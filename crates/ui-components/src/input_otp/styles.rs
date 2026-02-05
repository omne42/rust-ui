pub const CSS: &str = r#"
.ui-input-otp {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-input-otp__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-input-otp__control {
  position: relative;
  display: inline-flex;
  align-items: center;
  width: fit-content;
}

.ui-input-otp__input {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  border: 0;
  padding: 0;
  margin: 0;
  background: transparent;
  color: transparent;
  caret-color: transparent;
}

.ui-input-otp__group {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-input-otp__slot {
  width: 44px;
  height: 44px;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  position: relative;
  user-select: none;
}

.ui-input-otp__slot-value {
  font-size: 18px;
  font-weight: 700;
  line-height: 1;
}

.ui-input-otp--disabled .ui-input-otp__slot {
  pointer-events: none;
  opacity: 0.5;
}

.ui-input-otp--invalid .ui-input-otp__slot {
  border-color: var(--ui-danger);
}

.ui-input-otp--invalid.ui-input-otp--focus-visible .ui-input-otp__slot[data-active="true"] {
  outline-color: var(--ui-danger);
}

.ui-input-otp--focus-visible .ui-input-otp__slot[data-active="true"] {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-input-otp__caret {
  position: absolute;
  width: 2px;
  height: 18px;
  border-radius: 999px;
  background: var(--ui-fg);
  animation: ui-input-otp-caret-blink 1000ms steps(1, end) infinite;
}

@keyframes ui-input-otp-caret-blink {
  0%,
  49% {
    opacity: 1;
  }
  50%,
  100% {
    opacity: 0;
  }
}

.ui-input-otp__description {
  font-size: 13px;
  color: var(--ui-fg-muted);
}

.ui-input-otp__error {
  font-size: 13px;
  color: var(--ui-danger);
}
"#;
