pub const CSS: &str = r#"
.ui-tree {
  display: grid;
  gap: var(--ui-space-3xs);
  width: min(100%, 28rem);
  padding: var(--ui-space-xs);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
  transform: scale(var(--ui-tree-motion-scale, 1));
  opacity: var(--ui-tree-motion-opacity, 1);
}

.ui-tree[data-motion-source="custom"],
.ui-tree[data-custom-motion="true"] {
  --ui-tree-custom-motion: 1;
}

.ui-tree--tone-default,
.ui-tree[data-tone="default"] {
  background: var(--ui-bg);
}

.ui-tree--tone-quiet,
.ui-tree[data-tone="quiet"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 70%, var(--ui-bg) 30%);
}

.ui-tree--tone-strong,
.ui-tree[data-tone="strong"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 28%, var(--ui-bg) 72%);
  border-color: color-mix(in oklab, var(--ui-accent) 34%, var(--ui-border) 66%);
}

.ui-tree--density-comfortable .ui-tree__item,
.ui-tree[data-density="comfortable"] .ui-tree__item {
  min-height: 2rem;
  padding-inline: var(--ui-space-xs);
}

.ui-tree--density-compact .ui-tree__item,
.ui-tree[data-density="compact"] .ui-tree__item {
  min-height: 1.6rem;
  padding-inline: var(--ui-space-2xs);
}

.ui-tree--disabled,
.ui-tree[data-disabled="true"] {
  opacity: 0.7;
}

.ui-tree--has-selection,
.ui-tree[data-has-selection="true"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-accent) 18%, transparent);
}

.ui-tree--custom-class,
.ui-tree[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 22%, transparent);
}

.ui-tree__list {
  display: grid;
  gap: var(--ui-space-3xs);
  margin: 0;
  padding: 0;
  list-style-type: none;
}

.ui-tree__item {
  display: grid;
  grid-template-columns: auto 1fr;
  align-items: center;
  gap: var(--ui-space-2xs);
  width: 100%;
  border: 1px solid color-mix(in oklab, var(--ui-border) 70%, transparent);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
  color: var(--ui-fg);
  text-align: left;
}

.ui-tree__item--selected,
.ui-tree__item[data-selected="true"] {
  border-color: color-mix(in oklab, var(--ui-accent) 52%, var(--ui-border) 48%);
  background: color-mix(in oklab, var(--ui-accent-soft) 56%, var(--ui-bg) 44%);
}

.ui-tree__item--disabled,
.ui-tree__item[data-disabled="true"] {
  color: var(--ui-fg-muted);
  cursor: not-allowed;
}

.ui-tree__item--branch .ui-tree__chevron {
  font-size: 0.7rem;
}

.ui-tree__item--leaf .ui-tree__chevron {
  opacity: 0.62;
}

.ui-tree__item--depth-0 {
  padding-inline-start: var(--ui-space-xs);
}

.ui-tree__item--depth-1 {
  padding-inline-start: calc(var(--ui-space-xs) + var(--ui-space-md));
}

.ui-tree__item--depth-2 {
  padding-inline-start: calc(var(--ui-space-xs) + var(--ui-space-xl));
}

.ui-tree__item--depth-3 {
  padding-inline-start: calc(var(--ui-space-xs) + var(--ui-space-2xl));
}

.ui-tree__item--depth-4 {
  padding-inline-start: calc(var(--ui-space-xs) + var(--ui-space-3xl));
}

.ui-tree__item--depth-5-plus {
  padding-inline-start: calc(var(--ui-space-xs) + var(--ui-space-4xl));
}

.ui-tree__label {
  min-width: 0;
  font-size: 0.86rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
"#;
