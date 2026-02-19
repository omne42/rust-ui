pub const CSS: &str = r#"
.ui-avatar {
  width: var(--ui-avatar-size, 32px);
  height: var(--ui-avatar-size, 32px);

  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  border-radius: 9999px;
  overflow: hidden;
  box-sizing: border-box;

  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  color: var(--ui-fg);

  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-avatar--sm,
.ui-avatar[data-size="sm"] {
  --ui-avatar-size: 24px;
}

.ui-avatar--md,
.ui-avatar[data-size="md"] {
  --ui-avatar-size: 32px;
}

.ui-avatar--lg,
.ui-avatar[data-size="lg"] {
  --ui-avatar-size: 40px;
}

.ui-avatar--image,
.ui-avatar[data-state="image"],
.ui-avatar[data-image="true"] {
  background: var(--ui-bg);
}

.ui-avatar--fallback,
.ui-avatar[data-state="fallback"],
.ui-avatar[data-fallback="true"] {
  background: var(--ui-bg-muted);
}

.ui-avatar--has-src.ui-avatar--image,
.ui-avatar[data-has-src="true"][data-state="image"],
.ui-avatar[data-has-src="true"][data-image="true"] {
  border-color: transparent;
}

.ui-avatar--label-alt,
.ui-avatar[data-label-source="alt"] {
  --ui-avatar-label-source: 1;
}

.ui-avatar--label-name,
.ui-avatar[data-label-source="name"] {
  --ui-avatar-label-source: 2;
}

.ui-avatar--label-fallback,
.ui-avatar[data-label-source="fallback"] {
  --ui-avatar-label-source: 3;
}

.ui-avatar--has-alt[data-fallback="true"],
.ui-avatar[data-has-alt="true"][data-fallback="true"] {
  box-shadow: inset 0 0 0 1px var(--ui-border);
}

.ui-avatar--custom-class,
.ui-avatar[data-custom-class="true"] {
  --ui-avatar-custom-class: 1;
}

.ui-avatar__img {
  width: 100%;
  height: 100%;
  display: block;
  object-fit: cover;
  object-position: center;
}

.ui-avatar__initials {
  font-size: calc(var(--ui-avatar-size, 32px) / 2.5);
  line-height: 1;
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--ui-fg);
}
"#;

#[cfg(feature = "component-avatar_group")]
pub const AVATAR_GROUP_CSS: &str = r#"
.ui-avatar-group {
  --ui-avatar-group-size: 2rem;
  --ui-avatar-group-overlap: 10px;
  --ui-avatar-group-font-size: 0.75rem;
  --ui-avatar-group-overflow-padding: 0.375rem;
  display: inline-flex;
  align-items: center;
  min-height: var(--ui-avatar-group-size);
}

.ui-avatar-group__item {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
}

.ui-avatar-group__avatar {
  border: 2px solid var(--ui-bg);
  border-radius: 9999px;
  box-shadow: var(--ui-shadow-sm);
  background: var(--ui-bg-muted);
}

.ui-avatar-group__item:not(:first-child) {
  margin-left: calc(var(--ui-avatar-group-overlap) * -1);
}

.ui-avatar-group__overflow {
  position: relative;
  z-index: 2;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: var(--ui-avatar-group-size);
  height: var(--ui-avatar-group-size);
  padding-inline: var(--ui-avatar-group-overflow-padding);
  border-radius: 9999px;
  border: 2px solid var(--ui-bg);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  font-size: var(--ui-avatar-group-font-size);
  font-weight: 650;
  line-height: 1;
  box-shadow: var(--ui-shadow-sm);
}

.ui-avatar-group__overflow:not(:first-child) {
  margin-left: calc(var(--ui-avatar-group-overlap) * -1);
}

.ui-avatar-group--size-sm,
.ui-avatar-group[data-size="sm"] {
  --ui-avatar-group-size: 1.5rem;
  --ui-avatar-group-overlap: 8px;
  --ui-avatar-group-font-size: 0.6875rem;
  --ui-avatar-group-overflow-padding: 0.25rem;
}

.ui-avatar-group--size-md,
.ui-avatar-group[data-size="md"] {
  --ui-avatar-group-size: 2rem;
  --ui-avatar-group-overlap: 10px;
  --ui-avatar-group-font-size: 0.75rem;
  --ui-avatar-group-overflow-padding: 0.375rem;
}

.ui-avatar-group--size-lg,
.ui-avatar-group[data-size="lg"] {
  --ui-avatar-group-size: 2.5rem;
  --ui-avatar-group-overlap: 12px;
  --ui-avatar-group-font-size: 0.8125rem;
  --ui-avatar-group-overflow-padding: 0.5rem;
}

.ui-avatar-group--stable,
.ui-avatar-group[data-state="stable"] {
  --ui-avatar-group-state: 0;
}

.ui-avatar-group--overflow,
.ui-avatar-group[data-state="overflow"],
.ui-avatar-group[data-has-overflow="true"] {
  --ui-avatar-group-state: 1;
}

.ui-avatar-group--overflow .ui-avatar-group__overflow,
.ui-avatar-group[data-has-overflow="true"] .ui-avatar-group__overflow,
.ui-avatar-group[data-state="overflow"] .ui-avatar-group__overflow {
  background: color-mix(in oklch, var(--ui-accent-soft) 72%, var(--ui-bg-muted) 28%);
  border-color: color-mix(in oklch, var(--ui-accent) 42%, var(--ui-bg) 58%);
}

.ui-avatar-group--empty,
.ui-avatar-group[data-empty="true"],
.ui-avatar-group[data-state="empty"] {
  opacity: 0.88;
}

.ui-avatar-group--label-source-default,
.ui-avatar-group[data-aria-label-source="default"] {
  --ui-avatar-group-aria-label-source: 0;
}

.ui-avatar-group--label-source-custom,
.ui-avatar-group[data-custom-aria-label="true"],
.ui-avatar-group[data-aria-label-source="custom"] {
  --ui-avatar-group-aria-label-source: 1;
}

.ui-avatar-group--custom-class,
.ui-avatar-group[data-custom-class="true"],
.ui-avatar-group[data-class-source="custom"] {
  --ui-avatar-group-custom-class: 1;
}
"#;
