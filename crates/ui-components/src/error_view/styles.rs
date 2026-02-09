pub const CSS: &str = r#"
.ui-error-view {
  --ui-error-view-opacity: 1;
  --ui-error-view-translate-y: 0px;
  --ui-error-view-scale: 1;

  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  gap: var(--ui-space-xs);
  align-items: start;
  min-width: 0;
  margin-top: var(--ui-space-2xs);
  padding: var(--ui-space-xs) var(--ui-space-sm);
  border-radius: var(--ui-radius-md);
  border: 1px solid transparent;
  background: transparent;
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
  opacity: var(--ui-error-view-opacity);
  transform: translateY(var(--ui-error-view-translate-y)) scale(var(--ui-error-view-scale));
  transform-origin: top left;
  max-height: 12rem;
  overflow: hidden;
  will-change: transform, opacity;
}

.ui-error-view--tone-negative,
.ui-error-view[data-tone="negative"] {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-error-view--tone-neutral,
.ui-error-view[data-tone="neutral"] {
  color: var(--ui-fg-muted);
}

.ui-error-view--visible,
.ui-error-view[data-state="visible"] {
  pointer-events: auto;
}

.ui-error-view--hidden,
.ui-error-view[data-state="hidden"],
.ui-error-view[data-hidden="true"] {
  max-height: 0;
  margin-top: 0;
  padding-top: 0;
  padding-bottom: 0;
  border-width: 0;
  pointer-events: none;
}

.ui-error-view--compact,
.ui-error-view[data-compact="true"] {
  gap: var(--ui-space-2xs);
  padding: var(--ui-space-2xs) var(--ui-space-xs);
  font-size: 0.75rem;
}

.ui-error-view--bordered,
.ui-error-view[data-bordered="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 26%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-danger) 8%, var(--ui-bg-muted));
}

.ui-error-view__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
  margin-top: 0.0625rem;
}

.ui-error-view__content {
  min-width: 0;
}

.ui-error-view__text {
  margin: 0;
  min-width: 0;
  font-size: 0.75rem;
  line-height: 1.4;
}

.ui-error-view__actions {
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--ui-space-2xs);
}

.ui-error-view--with-actions,
.ui-error-view[data-actions="true"] {
  grid-template-columns: auto minmax(0, 1fr) auto;
}

.ui-error-view--with-icon,
.ui-error-view[data-icon="true"] {
  grid-template-columns: auto minmax(0, 1fr) auto;
}

.ui-error-view--with-children,
.ui-error-view[data-content="children"] {
  line-height: 1.35;
}

.ui-error-view--custom-class,
.ui-error-view[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
