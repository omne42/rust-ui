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

.ui-spinner--custom-label,
.ui-spinner[data-custom-aria-label="true"] {
  outline: none;
}
"#;
