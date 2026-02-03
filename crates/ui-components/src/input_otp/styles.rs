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

.ui-input-otp__group {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-input-otp__cell {
  width: 44px;
  height: 44px;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  text-align: center;
  font-size: 18px;
  font-weight: 700;
}

.ui-input-otp__cell:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-input-otp--disabled .ui-input-otp__cell {
  pointer-events: none;
  opacity: 0.5;
}
"#;
