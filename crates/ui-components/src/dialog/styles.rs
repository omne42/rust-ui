pub const CSS: &str = r#"
.ui-dialog {
  position: relative;
  width: min(100%, 520px);
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
}

.ui-dialog[data-motion-source="custom"],
.ui-dialog[data-custom-motion="true"] {
  --ui-dialog-custom-motion: 1;
}

.ui-dialog--size-sm {
  width: min(100%, 380px);
}

.ui-dialog--size-md {
  width: min(100%, 480px);
}

.ui-dialog--size-lg {
  width: min(100%, 640px);
}

.ui-dialog__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-dialog__title {
  font-size: 16px;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-dialog__description {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-dialog__body {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  min-width: 0;
}

.ui-dialog__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-dialog__close {
  position: absolute;
  top: 2px;
  right: 2px;
}
"#;
