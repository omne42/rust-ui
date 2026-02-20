pub const CSS: &str = r#"
.ui-alert-dialog {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  width: min(100%, var(--ui-overlay-panel-min-width));
}

.ui-alert-dialog--open,
.ui-alert-dialog[data-state="open"] {
  --ui-alert-dialog-open: 1;
}

.ui-alert-dialog--closed,
.ui-alert-dialog[data-state="closed"] {
  --ui-alert-dialog-open: 0;
}

.ui-alert-dialog--with-description,
.ui-alert-dialog[data-description="present"] {
  --ui-alert-dialog-description-lines: 2;
}

.ui-alert-dialog--title-only,
.ui-alert-dialog[data-description="absent"] {
  --ui-alert-dialog-description-lines: 0;
}

.ui-alert-dialog--cancel-shown,
.ui-alert-dialog[data-cancel="shown"] {
  --ui-alert-dialog-cancel-visible: 1;
}

.ui-alert-dialog--cancel-hidden,
.ui-alert-dialog[data-cancel="hidden"] {
  --ui-alert-dialog-cancel-visible: 0;
}

.ui-alert-dialog--secondary-shown,
.ui-alert-dialog[data-secondary="shown"] {
  --ui-alert-dialog-secondary-visible: 1;
}

.ui-alert-dialog--secondary-hidden,
.ui-alert-dialog[data-secondary="hidden"] {
  --ui-alert-dialog-secondary-visible: 0;
}

.ui-alert-dialog[data-motion-source="custom"],
.ui-alert-dialog[data-custom-motion="true"],
.ui-alert-dialog--custom-motion {
  --ui-alert-dialog-custom-motion: 1;
}

.ui-alert-dialog[data-variant-source="custom"],
.ui-alert-dialog[data-custom-variant="true"],
.ui-alert-dialog--custom-variant {
  --ui-alert-dialog-custom-variant: 1;
}

.ui-alert-dialog[data-id-source="custom"],
.ui-alert-dialog[data-custom-id="true"],
.ui-alert-dialog--custom-id {
  --ui-alert-dialog-custom-id: 1;
}

.ui-alert-dialog[data-title-source="custom"],
.ui-alert-dialog[data-custom-title="true"],
.ui-alert-dialog--custom-title {
  --ui-alert-dialog-custom-title: 1;
}

.ui-alert-dialog[data-description-source="custom"],
.ui-alert-dialog[data-custom-description="true"],
.ui-alert-dialog--custom-description {
  --ui-alert-dialog-custom-description: 1;
}

.ui-alert-dialog[data-cancel-source="custom"],
.ui-alert-dialog[data-custom-cancel="true"],
.ui-alert-dialog--custom-cancel {
  --ui-alert-dialog-custom-cancel: 1;
}

.ui-alert-dialog[data-secondary-source="custom"],
.ui-alert-dialog[data-custom-secondary="true"],
.ui-alert-dialog--custom-secondary {
  --ui-alert-dialog-custom-secondary: 1;
}

.ui-alert-dialog[data-confirm-source="custom"],
.ui-alert-dialog[data-custom-confirm="true"],
.ui-alert-dialog--custom-confirm {
  --ui-alert-dialog-custom-confirm: 1;
}

.ui-alert-dialog[data-auto-focus-source="custom"],
.ui-alert-dialog[data-custom-auto-focus="true"],
.ui-alert-dialog--custom-auto-focus {
  --ui-alert-dialog-custom-auto-focus: 1;
}

.ui-alert-dialog[data-exit-source="custom"],
.ui-alert-dialog[data-custom-exit="true"],
.ui-alert-dialog--custom-exit {
  --ui-alert-dialog-custom-exit: 1;
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
  width: var(--ui-icon-size-200);
  height: var(--ui-icon-size-200);
  flex: 0 0 var(--ui-icon-size-200);
  margin-top: var(--ui-space-2xs);
  color: var(--ui-fg-muted);
}

.ui-alert-dialog__type-icon svg {
  display: block;
  width: 100%;
  height: 100%;
}

.ui-alert-dialog--variant-warning .ui-alert-dialog__type-icon,
.ui-alert-dialog[data-variant="warning"] .ui-alert-dialog__type-icon {
  color: var(--ui-accent);
}

.ui-alert-dialog--variant-error .ui-alert-dialog__type-icon,
.ui-alert-dialog[data-variant="error"] .ui-alert-dialog__type-icon {
  color: var(--ui-danger);
}

.ui-alert-dialog__title[data-slot="alert-dialog-title"] {
  font-size: var(--ui-heading-h5-font-size);
  line-height: var(--ui-heading-h5-line-height);
  font-weight: 700;
  margin: 0;
}

.ui-alert-dialog__description[data-slot="alert-dialog-description"] {
  font-size: var(--ui-font-size-150);
  line-height: var(--ui-line-height-150);
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-alert-dialog__footer[data-slot="alert-dialog-footer"] {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-alert-dialog__action {
  display: contents;
}

.ui-alert-dialog--variant-destructive .ui-alert-dialog__title,
.ui-alert-dialog[data-variant="destructive"] .ui-alert-dialog__title {
  color: var(--ui-fg);
}
"#;
