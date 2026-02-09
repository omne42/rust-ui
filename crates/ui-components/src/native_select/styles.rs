pub const CSS: &str = r#"
.ui-native-select {
  position: relative;
  display: inline-flex;
  align-items: center;
  min-width: 220px;
}

.ui-native-select__control {
  width: 100%;
  appearance: none;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);
  transition:
    border-color 120ms ease,
    box-shadow 120ms ease,
    background-color 120ms ease,
    color 120ms ease;
  padding-right: 34px;
}

.ui-native-select__control:focus-visible {
  outline: 2px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-color: color-mix(in oklch, var(--ui-focus-ring) 60%, var(--ui-border));
}

.ui-native-select__control:disabled {
  cursor: not-allowed;
  opacity: 0.65;
}

.ui-native-select__indicator {
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--ui-fg-muted);
  pointer-events: none;
}

.ui-native-select--size-sm .ui-native-select__control {
  min-height: 30px;
  font-size: 12px;
  padding: 0 28px 0 10px;
}

.ui-native-select--size-md .ui-native-select__control {
  min-height: 34px;
  font-size: 13px;
  padding: 0 32px 0 12px;
}

.ui-native-select--size-lg .ui-native-select__control {
  min-height: 38px;
  font-size: 14px;
  padding: 0 34px 0 14px;
}

.ui-native-select--invalid .ui-native-select__control {
  border-color: color-mix(in oklch, var(--ui-danger) 64%, var(--ui-border));
}

.ui-native-select--selected .ui-native-select__control {
  border-color: color-mix(in oklch, var(--ui-accent) 52%, var(--ui-border));
}

.ui-native-select--empty .ui-native-select__control {
  color: var(--ui-fg-muted);
}

.ui-native-select--disabled .ui-native-select__control {
  background: var(--ui-bg-muted);
  color: color-mix(in oklch, var(--ui-fg-muted) 86%, var(--ui-bg));
}
"#;
