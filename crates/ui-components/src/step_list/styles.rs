pub const CSS: &str = r#"
.ui-step-list {
  --ui-step-list-marker-size: 1.5rem;
  --ui-step-list-gap: var(--ui-space-sm);
  --ui-step-list-marker-bg: color-mix(in oklab, var(--ui-fg-muted) 28%, transparent);
  --ui-step-list-marker-fg: var(--ui-fg-muted);
  --ui-step-list-label: var(--ui-fg);
  --ui-step-list-desc: var(--ui-fg-muted);
  --ui-step-list-connector: color-mix(in oklab, var(--ui-fg-muted) 28%, transparent);
  display: flex;
  gap: var(--ui-step-list-gap);
  margin: 0;
  padding: 0;
}

.ui-step-list--orientation-horizontal,
.ui-step-list[data-orientation="horizontal"] {
  flex-direction: row;
  align-items: stretch;
}

.ui-step-list--orientation-vertical,
.ui-step-list[data-orientation="vertical"] {
  flex-direction: column;
}

.ui-step-list__item {
  position: relative;
  display: flex;
  flex: 1 1 0;
  min-inline-size: 0;
}

.ui-step-list__button {
  appearance: none;
  border: none;
  margin: 0;
  padding: 0;
  inline-size: 100%;
  background: transparent;
  display: inline-flex;
  align-items: flex-start;
  gap: var(--ui-space-sm);
  text-align: start;
  cursor: pointer;
  color: inherit;
}

.ui-step-list__button:focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 84%, transparent);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-step-list__marker {
  inline-size: var(--ui-step-list-marker-size);
  block-size: var(--ui-step-list-marker-size);
  border-radius: 9999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 700;
  flex-shrink: 0;
  background: var(--ui-step-list-marker-bg);
  color: var(--ui-step-list-marker-fg);
}

.ui-step-list__content {
  display: inline-flex;
  flex-direction: column;
  gap: 0.125rem;
  min-inline-size: 0;
}

.ui-step-list__label {
  color: var(--ui-step-list-label);
  font-size: 0.875rem;
  line-height: 1.2;
  font-weight: 600;
}

.ui-step-list__description {
  color: var(--ui-step-list-desc);
  font-size: 0.75rem;
  line-height: 1.2;
}

.ui-step-list__connector {
  position: absolute;
  background: var(--ui-step-list-connector);
  pointer-events: none;
}

.ui-step-list--orientation-horizontal .ui-step-list__connector,
.ui-step-list[data-orientation="horizontal"] .ui-step-list__connector {
  inset-block-start: calc(var(--ui-step-list-marker-size) / 2 - 1px);
  inset-inline-start: calc(var(--ui-step-list-marker-size) + var(--ui-space-sm));
  inset-inline-end: calc(var(--ui-space-sm) * -0.5);
  block-size: 2px;
}

.ui-step-list--orientation-vertical .ui-step-list__connector,
.ui-step-list[data-orientation="vertical"] .ui-step-list__connector {
  inset-inline-start: calc(var(--ui-step-list-marker-size) / 2 - 1px);
  inset-block-start: calc(var(--ui-step-list-marker-size) + var(--ui-space-2xs));
  inset-block-end: calc(var(--ui-space-2xs) * -1);
  inline-size: 2px;
}

.ui-step-list__connector[data-last="true"] {
  display: none;
}

.ui-step-list__item--pending,
.ui-step-list__item[data-status="pending"] {
  --ui-step-list-marker-bg: color-mix(in oklab, var(--ui-fg-muted) 24%, transparent);
  --ui-step-list-marker-fg: var(--ui-fg-muted);
}

.ui-step-list__item--current,
.ui-step-list__item[data-status="current"] {
  --ui-step-list-marker-bg: color-mix(in oklab, var(--ui-accent) 88%, transparent);
  --ui-step-list-marker-fg: var(--ui-bg);
  --ui-step-list-label: var(--ui-accent);
}

.ui-step-list__item--completed,
.ui-step-list__item[data-status="completed"] {
  --ui-step-list-marker-bg: color-mix(in oklab, var(--ui-success) 90%, transparent);
  --ui-step-list-marker-fg: var(--ui-bg);
  --ui-step-list-label: color-mix(in oklab, var(--ui-success) 86%, var(--ui-fg));
}

.ui-step-list__item--disabled,
.ui-step-list__item[data-status="disabled"],
.ui-step-list--disabled .ui-step-list__item,
.ui-step-list[data-disabled="true"] .ui-step-list__item {
  opacity: 0.6;
}

.ui-step-list__item--disabled .ui-step-list__button,
.ui-step-list__item[data-status="disabled"] .ui-step-list__button {
  cursor: not-allowed;
}

.ui-step-list--size-s,
.ui-step-list[data-size="s"] {
  --ui-step-list-marker-size: 1.25rem;
  --ui-step-list-gap: var(--ui-space-xs);
}

.ui-step-list--size-m,
.ui-step-list[data-size="m"] {
  --ui-step-list-marker-size: 1.5rem;
}

.ui-step-list--size-l,
.ui-step-list[data-size="l"] {
  --ui-step-list-marker-size: 1.75rem;
  --ui-step-list-gap: var(--ui-space-md);
}

.ui-step-list--size-xl,
.ui-step-list[data-size="xl"] {
  --ui-step-list-marker-size: 2rem;
  --ui-step-list-gap: var(--ui-space-md);
}

.ui-step-list--emphasized,
.ui-step-list[data-emphasized="true"] {
  --ui-step-list-label: color-mix(in oklab, var(--ui-fg) 90%, var(--ui-accent));
}

.ui-step-list--custom-class,
.ui-step-list[data-custom-class="true"],
.ui-step-list[data-class-source="custom"] {
  --ui-step-list-custom-class: 1;
}

.ui-step-list[data-empty="true"] {
  min-block-size: 2.25rem;
}
"#;
