pub const CSS: &str = r#"
.ui-modal {
  display: grid;
  gap: 12px;
  width: min(36rem, calc(100vw - 2rem));
  max-width: 100%;
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

.ui-modal__title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  line-height: 1.2;
}

.ui-modal__description {
  margin: 0;
  line-height: 1.4;
  color: var(--ui-fg-muted);
}

.ui-modal__body {
  display: grid;
  gap: 12px;
}
"#;
