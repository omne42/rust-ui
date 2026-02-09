pub const CSS: &str = r#"
.ui-bottom-sheet {
  position: relative;
  display: grid;
  gap: var(--ui-space-md);
  min-width: 0;
  background: var(--ui-bg);
  color: var(--ui-fg);
}

.ui-bottom-sheet--detached,
.ui-bottom-sheet[data-detached="true"] {
  margin-inline: var(--ui-space-sm);
  padding: var(--ui-space-lg);
  border: 1px solid color-mix(in oklab, var(--ui-border) 82%, transparent);
  border-radius: var(--ui-radius-xl);
  box-shadow: var(--ui-shadow-lg, var(--ui-shadow-sm));
}

.ui-bottom-sheet--inset-none,
.ui-bottom-sheet[data-bottom-inset="none"] {
  margin-bottom: 0;
}

.ui-bottom-sheet--inset-sm,
.ui-bottom-sheet[data-bottom-inset="sm"] {
  margin-bottom: 8px;
}

.ui-bottom-sheet--inset-md,
.ui-bottom-sheet[data-bottom-inset="md"] {
  margin-bottom: 16px;
}

.ui-bottom-sheet--inset-lg,
.ui-bottom-sheet[data-bottom-inset="lg"] {
  margin-bottom: 24px;
}

.ui-bottom-sheet--inset-xl,
.ui-bottom-sheet[data-bottom-inset="xl"] {
  margin-bottom: 32px;
}

.ui-bottom-sheet--attached,
.ui-bottom-sheet[data-detached="false"] {
  padding: var(--ui-space-lg) var(--ui-space-lg) calc(var(--ui-space-lg) + env(safe-area-inset-bottom));
}

.ui-bottom-sheet__handle {
  display: flex;
  justify-content: center;
  align-items: center;
}

.ui-bottom-sheet__handle-bar {
  width: 2.75rem;
  height: 0.3125rem;
  border-radius: 9999px;
  background: color-mix(in oklab, var(--ui-border) 68%, var(--ui-fg-muted) 32%);
}

.ui-bottom-sheet__close {
  position: absolute;
  top: var(--ui-space-sm);
  right: var(--ui-space-sm);
  display: inline-flex;
}

.ui-bottom-sheet__header {
  display: grid;
  gap: var(--ui-space-2xs);
  min-width: 0;
}

.ui-bottom-sheet--close-shown .ui-bottom-sheet__header,
.ui-bottom-sheet[data-close-button="shown"] .ui-bottom-sheet__header {
  padding-right: calc(var(--ui-space-xl) + var(--ui-space-sm));
}

.ui-bottom-sheet__title {
  margin: 0;
  font-size: 1rem;
  line-height: 1.35;
  font-weight: 700;
}

.ui-bottom-sheet__description {
  margin: 0;
  font-size: 0.8125rem;
  line-height: 1.45;
  color: var(--ui-fg-muted);
}

.ui-bottom-sheet__body {
  display: grid;
  gap: var(--ui-space-sm);
  min-width: 0;
}

.ui-bottom-sheet__footer {
  display: grid;
  gap: var(--ui-space-xs);
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
  padding-top: var(--ui-space-xs);
  border-top: 1px solid color-mix(in oklab, var(--ui-border) 72%, transparent);
}

.ui-bottom-sheet--custom-class,
.ui-bottom-sheet[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
