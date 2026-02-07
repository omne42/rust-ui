pub const CSS: &str = r#"
.ui-spinner {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-spinner__progress {
  display: inline-flex;
}

.ui-spinner--size-sm,
.ui-spinner[data-size="sm"] {
  --ui-cp-size: 16px;
  --ui-cp-thickness: 2px;
}

.ui-spinner--size-md,
.ui-spinner[data-size="md"] {
  --ui-cp-size: 20px;
  --ui-cp-thickness: 2px;
}

.ui-spinner--size-lg,
.ui-spinner[data-size="lg"] {
  --ui-cp-size: 28px;
  --ui-cp-thickness: 3px;
}

.ui-spinner--label-custom .ui-spinner__progress,
.ui-spinner[data-label-source="custom"] .ui-spinner__progress {
  border-top-color: color-mix(in oklch, var(--ui-accent), var(--ui-fg) 12%);
}

.ui-spinner--custom-class,
.ui-spinner[data-custom-class="true"] {
  isolation: isolate;
}

.ui-spinner[data-class-source="custom"] .ui-spinner__progress {
  box-shadow: inset 0 0 0 1px color-mix(in oklch, var(--ui-border), transparent 45%);
}

.ui-spinner[data-state="indeterminate"] .ui-spinner__progress,
.ui-spinner[data-indeterminate="true"] .ui-spinner__progress {
  animation-play-state: running;
}
"#;
