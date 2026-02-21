pub const CSS: &str = r#"
.ui-command-dialog {
  display: grid;
  gap: 0;
  --ui-command-dialog-open-opacity: 1;
  --ui-command-dialog-closed-opacity: 0.98;
  --ui-command-dialog-disabled-opacity: var(
    --ui-checkbox-disabled-opacity,
    var(--ui-fallback-checkbox-disabled-opacity)
  );
  --ui-command-dialog-border-width: var(
    --ui-border-width,
    var(--ui-fallback-border-width)
  );
  --ui-command-dialog-border-color: var(--ui-border, var(--ui-fallback-border));
  --ui-command-dialog-modal-viewport-inset: var(
    --ui-overlay-viewport-inset,
    var(--ui-fallback-overlay-viewport-inset)
  );
  --ui-command-dialog-modal-max-inline-viewport: calc(
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 4
  );
  --ui-command-dialog-modal-max-width: var(
    --ui-command-panel-max-width,
    var(--ui-fallback-command-panel-max-width)
  );
  --ui-command-dialog-modal-header-padding-inline: var(
    --ui-space-sm,
    var(--ui-fallback-space-sm)
  );
  --ui-command-dialog-modal-header-padding-top: var(
    --ui-space-sm,
    var(--ui-fallback-space-sm)
  );
}

.ui-command-dialog--open,
.ui-command-dialog[data-state="open"] {
  opacity: var(--ui-command-dialog-open-opacity);
}

.ui-command-dialog--closed,
.ui-command-dialog[data-state="closed"] {
  opacity: var(--ui-command-dialog-closed-opacity);
}

.ui-command-dialog--with-description,
.ui-command-dialog[data-description="present"] {
  --ui-command-dialog-description-lines: 2;
}

.ui-command-dialog--title-only,
.ui-command-dialog[data-description="absent"] {
  --ui-command-dialog-description-lines: 0;
}

.ui-command-dialog--persistent,
.ui-command-dialog[data-close-on-action="false"] {
  border-top: var(--ui-command-dialog-border-width) solid
    var(--ui-command-dialog-border-color);
}

.ui-command-dialog--disabled,
.ui-command-dialog[data-disabled="true"] {
  opacity: var(--ui-command-dialog-disabled-opacity);
}

.ui-command-dialog--controlled,
.ui-command-dialog[data-open-mode="controlled"] {
  --ui-command-dialog-open-mode: 1;
}

.ui-command-dialog--uncontrolled,
.ui-command-dialog[data-open-mode="uncontrolled"] {
  --ui-command-dialog-open-mode: 0;
}

.ui-command-dialog--custom-command-motion,
.ui-command-dialog[data-command-motion-source="custom"],
.ui-command-dialog[data-custom-command-motion="true"] {
  --ui-command-dialog-custom-command-motion: 1;
}

.ui-command-dialog--custom-overlay-motion,
.ui-command-dialog[data-overlay-motion-source="custom"],
.ui-command-dialog[data-custom-overlay-motion="true"] {
  --ui-command-dialog-custom-overlay-motion: 1;
}

.ui-command-dialog[data-id-source="custom"],
.ui-command-dialog[data-custom-id="true"],
.ui-command-dialog--custom-id {
  --ui-command-dialog-custom-id: 1;
}

.ui-command-dialog[data-title-source="custom"],
.ui-command-dialog[data-custom-title="true"],
.ui-command-dialog--custom-title {
  --ui-command-dialog-custom-title: 1;
}

.ui-command-dialog[data-description-source="custom"],
.ui-command-dialog[data-custom-description="true"],
.ui-command-dialog--custom-description {
  --ui-command-dialog-custom-description: 1;
}

.ui-command-dialog[data-placeholder-source="custom"],
.ui-command-dialog[data-custom-placeholder="true"],
.ui-command-dialog--custom-placeholder {
  --ui-command-dialog-custom-placeholder: 1;
}

.ui-command-dialog[data-empty-label-source="custom"],
.ui-command-dialog[data-custom-empty-label="true"],
.ui-command-dialog--custom-empty-label {
  --ui-command-dialog-custom-empty-label: 1;
}

.ui-command-dialog[data-aria-label-source="custom"],
.ui-command-dialog[data-custom-aria-label="true"],
.ui-command-dialog--custom-aria-label {
  --ui-command-dialog-custom-aria-label: 1;
}

.ui-command-dialog[data-class-source="custom"],
.ui-command-dialog[data-custom-class="true"],
.ui-command-dialog--custom-class {
  --ui-command-dialog-custom-class: 1;
}

.ui-command-dialog[data-action-source="custom"],
.ui-command-dialog[data-custom-action="true"],
.ui-command-dialog--custom-action {
  --ui-command-dialog-custom-action: 1;
}

.ui-command-dialog[data-open-change-source="custom"],
.ui-command-dialog[data-custom-open-change="true"],
.ui-command-dialog--custom-open-change {
  --ui-command-dialog-custom-open-change: 1;
}

.ui-command-dialog[data-default-open-source="custom"],
.ui-command-dialog[data-custom-default-open="true"],
.ui-command-dialog--custom-default-open {
  --ui-command-dialog-custom-default-open: 1;
}

.ui-command-dialog[data-close-on-action-source="custom"],
.ui-command-dialog[data-custom-close-on-action="true"],
.ui-command-dialog--custom-close-on-action {
  --ui-command-dialog-custom-close-on-action: 1;
}

.ui-command-dialog[data-disabled-source="custom"],
.ui-command-dialog[data-custom-disabled="true"],
.ui-command-dialog--custom-disabled {
  --ui-command-dialog-custom-disabled: 1;
}

.ui-command-dialog__modal.ui-modal {
  width: min(
    var(--ui-command-dialog-modal-max-width),
    calc(
      var(--ui-command-dialog-modal-max-inline-viewport) -
        (var(--ui-command-dialog-modal-viewport-inset) * 2)
    )
  );
  padding: 0;
  overflow: hidden;
}

.ui-command-dialog__modal.ui-modal .ui-modal__title,
.ui-command-dialog__modal.ui-modal .ui-modal__description {
  padding-inline: var(--ui-command-dialog-modal-header-padding-inline);
  padding-top: var(--ui-command-dialog-modal-header-padding-top);
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
"#;
