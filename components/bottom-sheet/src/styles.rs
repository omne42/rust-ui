pub const CSS: &str = r#"
.ui-bottom-sheet {
  position: relative;
  display: grid;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
  min-width: 0;
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-bottom-sheet[data-motion-source="custom"],
.ui-bottom-sheet[data-custom-motion="true"] {
  /* custom motion is state-only; rendering stays token-first */
}

.ui-bottom-sheet--detached,
.ui-bottom-sheet[data-detached="true"] {
  margin-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 82%, transparent);
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}

.ui-bottom-sheet--inset-none,
.ui-bottom-sheet[data-bottom-inset="none"] {
  margin-bottom: 0;
}

.ui-bottom-sheet--inset-sm,
.ui-bottom-sheet[data-bottom-inset="sm"] {
  margin-bottom: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-bottom-sheet--inset-md,
.ui-bottom-sheet[data-bottom-inset="md"] {
  margin-bottom: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-bottom-sheet--inset-lg,
.ui-bottom-sheet[data-bottom-inset="lg"] {
  margin-bottom: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-bottom-sheet--inset-xl,
.ui-bottom-sheet[data-bottom-inset="xl"] {
  margin-bottom: var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)));
}

.ui-bottom-sheet--attached,
.ui-bottom-sheet[data-detached="false"] {
  padding:
    var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)))
    var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)))
    calc(
      var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md))) +
        env(safe-area-inset-bottom)
    );
}

.ui-bottom-sheet__handle {
  display: flex;
  justify-content: center;
  align-items: center;
}

.ui-bottom-sheet__handle-bar {
  width: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md))) +
      var(--ui-space-md, var(--ui-fallback-space-md))
  );
  height: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 68%,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 32%
  );
}

.ui-bottom-sheet__close {
  position: absolute;
  top: var(--ui-space-sm, var(--ui-fallback-space-sm));
  right: var(--ui-space-sm, var(--ui-fallback-space-sm));
  display: inline-flex;
}

.ui-bottom-sheet__header {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  min-width: 0;
}

.ui-bottom-sheet--close-shown .ui-bottom-sheet__header,
.ui-bottom-sheet[data-close-button="shown"] .ui-bottom-sheet__header {
  padding-right: calc(
    var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md))) +
      var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
}

.ui-bottom-sheet__title {
  margin: 0;
  font-size: var(--ui-heading-h5-font-size, var(--ui-fallback-font-size-150));
  line-height: var(--ui-heading-h5-line-height, var(--ui-fallback-line-height-150));
  font-weight: 700;
}

.ui-bottom-sheet__description {
  margin: 0;
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-bottom-sheet__body {
  display: grid;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-width: 0;
}

.ui-bottom-sheet__footer {
  display: grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  min-width: 0;
}

.ui-bottom-sheet--handle-hidden .ui-bottom-sheet__handle,
.ui-bottom-sheet[data-handle="hidden"] .ui-bottom-sheet__handle {
  display: none;
}

.ui-bottom-sheet--close-hidden .ui-bottom-sheet__close,
.ui-bottom-sheet[data-close-button="hidden"] .ui-bottom-sheet__close {
  display: none;
}

.ui-bottom-sheet--title-only .ui-bottom-sheet__description,
.ui-bottom-sheet[data-state="title-only"] .ui-bottom-sheet__description {
  display: none;
}

.ui-bottom-sheet[data-footer="present"] .ui-bottom-sheet__footer {
  padding-top: var(--ui-space-xs, var(--ui-fallback-space-xs));
  border-top: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 72%, transparent);
}

.ui-bottom-sheet--custom-class,
.ui-bottom-sheet[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs)));
}
"#;
