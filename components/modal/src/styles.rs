pub const CSS: &str = r#"
.ui-modal {
  display: grid;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
  width: min(
    calc(
      var(
          --ui-overlay-panel-min-width,
          var(--ui-fallback-overlay-panel-min-width)
        ) + var(--ui-space-lg, var(--ui-fallback-space-lg)) * 6
    ),
    calc(
      100vw
        - var(
            --ui-overlay-viewport-inset,
            var(--ui-fallback-overlay-viewport-inset)
          ) * 2
    )
  );
  max-width: 100%;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-modal[data-motion-source="custom"],
.ui-modal[data-custom-motion="true"],
.ui-modal--custom-motion {
  --ui-modal-custom-motion: 1;
}

.ui-modal--custom-id,
.ui-modal[data-id-source="custom"],
.ui-modal[data-custom-id="true"] {
  --ui-modal-id-source: custom;
}

.ui-modal--custom-title,
.ui-modal[data-title-source="custom"],
.ui-modal[data-custom-title="true"] {
  --ui-modal-title-source: custom;
}

.ui-modal--custom-description,
.ui-modal[data-description-source="custom"],
.ui-modal[data-custom-description="true"] {
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
  gap: calc(
    var(--ui-space-md, var(--ui-fallback-space-md))
      - var(--ui-space-3xs, var(--ui-fallback-space-3xs))
  );
}

.ui-modal--title-only,
.ui-modal[data-state="title-only"],
.ui-modal[data-description="absent"] {
  gap: calc(
    var(--ui-space-md, var(--ui-fallback-space-md))
      + var(--ui-space-3xs, var(--ui-fallback-space-3xs))
  );
}

.ui-modal:focus-within {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-fg, var(--ui-fallback-fg)) 24%,
      transparent
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border-radius: calc(
    var(--ui-radius-lg, var(--ui-fallback-radius-lg))
      - var(--ui-space-3xs, var(--ui-fallback-space-3xs))
  );
}

.ui-modal--custom-class,
.ui-modal[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-modal__title,
.ui-modal__title[data-slot="modal-title"] {
  margin: 0;
  font-size: var(
    --ui-heading-h5-font-size,
    var(--ui-fallback-heading-h5-font-size)
  );
  font-weight: 600;
  line-height: var(
    --ui-heading-h5-line-height,
    var(--ui-fallback-heading-h5-line-height)
  );
}

.ui-modal__title[data-title-source="custom"] {
  --ui-modal-title: custom;
}

.ui-modal__description,
.ui-modal__description[data-slot="modal-description"] {
  margin: 0;
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-modal__description[data-description-source="custom"] {
  --ui-modal-description: custom;
}

.ui-modal__body,
.ui-modal__body[data-slot="modal-body"] {
  display: grid;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-modal__body :where(button, [role="button"], a[href]) {
  transition:
    box-shadow
      var(
        --ui-text-field-motion-duration,
        var(--ui-fallback-text-field-motion-duration)
      )
      ease,
    transform
      var(
        --ui-text-field-motion-duration,
        var(--ui-fallback-text-field-motion-duration)
      )
      ease;
}

.ui-modal__body :where(button, [role="button"], a[href]):hover {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}

.ui-modal__body :where(button, [role="button"], a[href]):active {
  transform: translateY(
    var(--ui-border-width, var(--ui-fallback-border-width))
  );
}

.ui-modal__body :where(button, [role="button"], a[href]):focus-visible {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-fg, var(--ui-fallback-fg)) 32%,
      transparent
    );
  outline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}
"#;
