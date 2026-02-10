pub const CSS: &str = r#"
.ui-alert-dialog {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  width: min(100%, 480px);
}

.ui-alert-dialog[data-motion-source="custom"],
.ui-alert-dialog[data-custom-motion="true"] {
  --ui-alert-dialog-custom-motion: 1;
}

.ui-alert-dialog__header {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-md);
}

.ui-alert-dialog__header-text {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  min-width: 0;
}

.ui-alert-dialog__type-icon {
  width: 20px;
  height: 20px;
  flex: 0 0 20px;
  margin-top: 2px;
  color: var(--ui-fg-muted);
}

.ui-alert-dialog__type-icon svg {
  display: block;
  width: 100%;
  height: 100%;
}

.ui-alert-dialog--variant-warning .ui-alert-dialog__type-icon {
  color: var(--ui-accent);
}

.ui-alert-dialog--variant-error .ui-alert-dialog__type-icon {
  color: var(--ui-danger);
}

.ui-alert-dialog__title {
  font-size: 16px;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-alert-dialog__description {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-alert-dialog__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-alert-dialog--variant-destructive .ui-alert-dialog__title {
  color: var(--ui-fg);
}
"#;
