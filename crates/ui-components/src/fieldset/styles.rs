pub const CSS: &str = r#"
.ui-fieldset {
  display: grid;
  min-width: 0;
  margin: 0;
  padding: var(--ui-space-sm) 0 0;
  border: none;
  gap: var(--ui-space-sm);
  color: var(--ui-fg);
}

.ui-fieldset[data-motion-source="custom"],
.ui-fieldset[data-custom-motion="true"] {
  --ui-fieldset-custom-motion: 1;
}

.ui-fieldset--orientation-vertical,
.ui-fieldset[data-orientation="vertical"] {
  grid-template-columns: minmax(0, 1fr);
  align-items: start;
}

.ui-fieldset--orientation-horizontal,
.ui-fieldset[data-orientation="horizontal"] {
  grid-template-columns: minmax(8rem, 14rem) minmax(0, 1fr);
  align-items: start;
  column-gap: var(--ui-space-md);
}

.ui-fieldset--tone-default,
.ui-fieldset[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-fieldset--tone-muted,
.ui-fieldset[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-fieldset--required .ui-fieldset__legend,
.ui-fieldset[data-required="true"] .ui-fieldset__legend {
  font-weight: 600;
}

.ui-fieldset--disabled,
.ui-fieldset[data-disabled="true"] {
  opacity: 0.72;
}

.ui-fieldset--invalid .ui-fieldset__group,
.ui-fieldset[data-invalid="true"] .ui-fieldset__group {
  outline: 1px solid color-mix(in oklab, var(--ui-danger) 44%, transparent);
  border-radius: var(--ui-radius-sm);
}

.ui-fieldset__legend {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-2xs, 4px);
  min-width: 0;
  margin: 0;
  padding: 0;
  font-size: 0.875rem;
}

.ui-fieldset--orientation-horizontal .ui-fieldset__legend,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__legend {
  justify-content: flex-end;
}

.ui-fieldset__required-indicator {
  color: color-mix(in oklab, var(--ui-danger) 78%, var(--ui-fg) 22%);
}

.ui-fieldset__group {
  min-width: 0;
  display: grid;
  gap: var(--ui-space-xs);
}

.ui-fieldset__actions {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-fieldset--orientation-horizontal .ui-fieldset__actions,
.ui-fieldset--orientation-horizontal .ui-fieldset__description,
.ui-fieldset--orientation-horizontal .ui-fieldset__error,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__actions,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__description,
.ui-fieldset[data-orientation="horizontal"] .ui-fieldset__error {
  grid-column: 2;
}

.ui-fieldset__description,
.ui-fieldset__error {
  margin: 0;
  font-size: 0.75rem;
  line-height: 1.35;
  transition:
    opacity var(--ui-fieldset-motion-duration, 170ms) ease,
    transform var(--ui-fieldset-motion-duration, 170ms) ease;
  transform: translateY(var(--ui-fieldset-motion-distance, 0px));
  opacity: 1;
}

.ui-fieldset__description {
  color: var(--ui-fg-muted);
}

.ui-fieldset__error {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-fieldset--custom-class,
.ui-fieldset[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
