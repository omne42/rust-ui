pub const CSS: &str = r#"
.ui-dialog {
  position: relative;
  width: min(
    100%,
    calc(
      var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 2
    )
  );
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-dialog--with-description,
.ui-dialog[data-state="with-description"] {
  --ui-dialog-description-lines: 2;
}

.ui-dialog--title-only,
.ui-dialog[data-state="title-only"] {
  --ui-dialog-description-lines: 0;
}

.ui-dialog--with-footer,
.ui-dialog[data-footer="present"] {
  --ui-dialog-has-footer: 1;
}

.ui-dialog--footer-absent,
.ui-dialog[data-footer="absent"] {
  --ui-dialog-has-footer: 0;
}

.ui-dialog--close-shown,
.ui-dialog[data-close-button="shown"] {
  --ui-dialog-close-visible: 1;
}

.ui-dialog--close-hidden,
.ui-dialog[data-close-button="hidden"] {
  --ui-dialog-close-visible: 0;
}

.ui-dialog--size-sm,
.ui-dialog[data-size="sm"] {
  width: min(
    100%,
    calc(
      var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 1.58
    )
  );
}

.ui-dialog--size-md,
.ui-dialog[data-size="md"] {
  width: min(
    100%,
    calc(
      var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 2
    )
  );
}

.ui-dialog--size-lg,
.ui-dialog[data-size="lg"] {
  width: min(
    100%,
    calc(
      var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width)) * 2.67
    )
  );
}

.ui-dialog[data-motion-source="custom"],
.ui-dialog[data-custom-motion="true"],
.ui-dialog--custom-motion {
  --ui-dialog-custom-motion: 1;
}

.ui-dialog[data-size-source="custom"],
.ui-dialog[data-custom-size="true"],
.ui-dialog--custom-size {
  --ui-dialog-custom-size: 1;
}

.ui-dialog[data-id-source="custom"],
.ui-dialog[data-custom-id="true"],
.ui-dialog--custom-id {
  --ui-dialog-custom-id: 1;
}

.ui-dialog[data-title-source="custom"],
.ui-dialog[data-custom-title="true"],
.ui-dialog--custom-title {
  --ui-dialog-custom-title: 1;
}

.ui-dialog[data-description-source="custom"],
.ui-dialog[data-custom-description="true"],
.ui-dialog--custom-description {
  --ui-dialog-custom-description: 1;
}

.ui-dialog[data-close-source="custom"],
.ui-dialog[data-custom-close="true"],
.ui-dialog--custom-close {
  --ui-dialog-custom-close: 1;
}

.ui-dialog[data-exit-source="custom"],
.ui-dialog[data-custom-exit="true"],
.ui-dialog--custom-exit {
  --ui-dialog-custom-exit: 1;
}

.ui-dialog--custom-class,
.ui-dialog[data-custom-class="true"] {
  --ui-dialog-custom-class: 1;
}

.ui-dialog__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-dialog__title[data-slot="dialog-title"] {
  font-size: var(
    --ui-heading-h5-font-size,
    var(--ui-fallback-heading-h5-font-size)
  );
  line-height: var(
    --ui-heading-h5-line-height,
    var(--ui-fallback-heading-h5-line-height)
  );
  font-weight: 700;
  margin: 0;
}

.ui-dialog__description[data-slot="dialog-description"] {
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  margin: 0;
}

.ui-dialog__body[data-slot="dialog-body"] {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-width: 0;
}

.ui-dialog__footer[data-slot="dialog-footer"] {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  flex-wrap: wrap;
}

.ui-dialog__close[data-slot="dialog-close"] {
  position: absolute;
  top: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
  right: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}
"#;
