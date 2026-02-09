pub const CSS: &str = r#"
.ui-command-dialog {
  display: grid;
  gap: 0;
}

.ui-command-dialog__modal.ui-modal {
  width: min(44rem, calc(100vw - 2rem));
  padding: 0;
  overflow: hidden;
}

.ui-command-dialog__modal.ui-modal .ui-modal__title,
.ui-command-dialog__modal.ui-modal .ui-modal__description {
  padding-inline: 12px;
  padding-top: 12px;
  margin: 0;
}

.ui-command-dialog__modal.ui-modal .ui-modal__body {
  gap: 0;
  padding: 0;
}

.ui-command-dialog__command.ui-command {
  width: 100%;
  border: 0;
  border-radius: 0;
  box-shadow: none;
}

.ui-command-dialog--open,
.ui-command-dialog[data-state="open"] {
  opacity: 1;
}

.ui-command-dialog--closed,
.ui-command-dialog[data-state="closed"] {
  opacity: 0.98;
}

.ui-command-dialog--persistent,
.ui-command-dialog[data-close-on-action="false"] {
  border-top: 1px solid var(--ui-border-subtle, rgba(125, 125, 125, 0.35));
}

.ui-command-dialog--disabled,
.ui-command-dialog[data-disabled="true"] {
  opacity: 0.72;
}

.ui-command-dialog--custom-class,
.ui-command-dialog[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
