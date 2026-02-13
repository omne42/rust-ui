pub const CSS: &str = r#"
.ui-color-area {
  --ui-color-area-motion-duration: 180ms;

  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  min-inline-size: min(100%, 20rem);
  transition: opacity var(--ui-color-area-motion-duration) ease;
}

.ui-color-area__label {
  color: var(--ui-fg-muted);
  font-size: 0.75rem;
  font-weight: 600;
  line-height: 1.2;
}

.ui-color-area__preview {
  inline-size: 1.75rem;
  block-size: 1.75rem;
}

.ui-color-area__grid {
  display: inline-flex;
  flex-direction: column;
  gap: 1px;
  border: 1px solid color-mix(in oklab, var(--ui-fg-muted) 28%, transparent);
  border-radius: var(--ui-radius-sm);
  padding: 1px;
  background:
    linear-gradient(to top, color-mix(in oklab, black 22%, transparent), transparent),
    linear-gradient(to right, white, color-mix(in oklab, var(--ui-accent) 88%, transparent));
}

.ui-color-area__row {
  display: inline-flex;
  gap: 1px;
}

.ui-color-area__cell {
  appearance: none;
  border: none;
  padding: 0;
  margin: 0;
  inline-size: 0.9rem;
  block-size: 0.9rem;
  background: transparent;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.ui-color-area__thumb {
  inline-size: 0.45rem;
  block-size: 0.45rem;
  border-radius: 9999px;
  background: transparent;
  box-shadow: 0 0 0 1px transparent;
}

.ui-color-area__cell[data-selected="true"] .ui-color-area__thumb,
.ui-color-area__cell[aria-selected="true"] .ui-color-area__thumb {
  background: var(--ui-bg);
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-fg) 82%, transparent);
}

.ui-color-area__cell:focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 84%, transparent);
  outline-offset: 1px;
}

.ui-color-area__axes {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: var(--ui-space-xs) var(--ui-space-sm);
  align-items: center;
}

.ui-color-area__axis-label {
  color: var(--ui-fg-muted);
  font-size: 0.75rem;
}

.ui-color-area__axis-input {
  inline-size: 100%;
}

.ui-color-area--with-preview,
.ui-color-area[data-has-preview="true"] {
  --ui-color-area-with-preview: 1;
}

.ui-color-area--disabled,
.ui-color-area[data-disabled="true"] {
  opacity: 0.62;
}

.ui-color-area--disabled .ui-color-area__cell,
.ui-color-area[data-disabled="true"] .ui-color-area__cell,
.ui-color-area--disabled .ui-color-area__axis-input,
.ui-color-area[data-disabled="true"] .ui-color-area__axis-input {
  cursor: not-allowed;
}

.ui-color-area--custom-class,
.ui-color-area[data-custom-class="true"],
.ui-color-area[data-class-source="custom"] {
  --ui-color-area-custom-class: 1;
}

@media (prefers-reduced-motion: reduce) {
  .ui-color-area {
    --ui-color-area-motion-duration: 1ms;
  }
}
"#;
