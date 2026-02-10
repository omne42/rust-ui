pub const CSS: &str = r#"
.ui-modal {
  display: grid;
  gap: 12px;
  width: min(36rem, calc(100vw - 2rem));
  max-width: 100%;
}

.ui-modal[data-motion-source="custom"],
.ui-modal[data-custom-motion="true"],
.ui-modal--custom-motion {
  --ui-modal-custom-motion: 1;
}

.ui-modal[data-id-source="custom"] {
  --ui-modal-id-source: custom;
}

.ui-modal[data-title-source="custom"] {
  --ui-modal-title-source: custom;
}

.ui-modal[data-description-source="custom"] {
  --ui-modal-description-source: custom;
}

.ui-modal[data-class-source="custom"],
.ui-modal--custom-class {
  --ui-modal-class-source: custom;
}

.ui-modal[data-exit-source="custom"],
.ui-modal[data-custom-exit="true"],
.ui-modal--custom-exit {
  --ui-modal-exit-source: custom;
}

.ui-modal--with-description,
.ui-modal[data-state="with-description"],
.ui-modal[data-description="present"] {
  gap: 10px;
}

.ui-modal--title-only,
.ui-modal[data-state="title-only"],
.ui-modal[data-description="absent"] {
  gap: 14px;
}

.ui-modal--custom-class,
.ui-modal[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-modal__title,
.ui-modal__title[data-slot="modal-title"] {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  line-height: 1.2;
}

.ui-modal__title[data-title-source="custom"] {
  --ui-modal-title: custom;
}

.ui-modal__description,
.ui-modal__description[data-slot="modal-description"] {
  margin: 0;
  line-height: 1.4;
  color: var(--ui-fg-muted);
}

.ui-modal__description[data-description-source="custom"] {
  --ui-modal-description: custom;
}

.ui-modal__body,
.ui-modal__body[data-slot="modal-body"] {
  display: grid;
  gap: 12px;
}
"#;
