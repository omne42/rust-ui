pub const CSS: &str = r#"
.ui-sidebar-group {
  --ui-sidebar-group-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-group-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-group-line-height: var(
    --ui-line-height-100,
    var(--ui-fallback-line-height-100)
  );
  --ui-sidebar-group-radius-xs: var(
    --ui-radius-xs,
    var(--ui-radius-sm, var(--ui-fallback-radius-sm))
  );
  --ui-sidebar-group-border-subtle: var(
    --ui-border-subtle,
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 20%, transparent)
  );
  --ui-sidebar-group-accent-solid: var(
    --ui-accent-solid,
    var(--ui-accent, var(--ui-fallback-accent))
  );
  --ui-sidebar-group-hover-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 44%,
    transparent
  );
  --ui-sidebar-group-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  display: grid;
  gap: 0.35rem;
}

.ui-sidebar-group__header {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  align-items: center;
  gap: 0.25rem;
}

.ui-sidebar-group__label,
.ui-sidebar-group__action,
.ui-sidebar-group__toggle {
  border: 0;
  background: transparent;
  color: inherit;
  font: inherit;
  line-height: var(--ui-sidebar-group-line-height);
  border-radius: var(--ui-sidebar-group-radius-xs);
}

.ui-sidebar-group__label {
  display: flex;
  align-items: center;
  min-width: 0;
  padding: 0.35rem 0.4rem;
  text-align: left;
}

.ui-sidebar-group__label span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ui-sidebar-group__action,
.ui-sidebar-group__toggle {
  padding: 0.25rem 0.4rem;
}

.ui-sidebar-group__toggle {
  transform-origin: center;
  transition:
    transform var(--ui-sidebar-group-motion-duration) var(--ui-sidebar-group-motion-easing);
}

.ui-sidebar-group__toggle[data-open="true"] {
  transform: rotate(180deg);
}

.ui-sidebar-group__content {
  display: grid;
  gap: 0.3rem;
  padding-inline-start: 0.2rem;
  border-inline-start: 1px solid var(--ui-sidebar-group-border-subtle);
  transition:
    opacity var(--ui-sidebar-group-motion-duration) var(--ui-sidebar-group-motion-easing),
    transform var(--ui-sidebar-group-motion-duration) var(--ui-sidebar-group-motion-easing);
}

.ui-sidebar-group[data-state="closed"] .ui-sidebar-group__content {
  opacity: 0;
  transform: translateY(-0.25rem);
}

.ui-sidebar-group__label:hover,
.ui-sidebar-group__action:hover,
.ui-sidebar-group__toggle:hover {
  background: var(--ui-sidebar-group-hover-bg);
}

.ui-sidebar-group__label:focus-visible,
.ui-sidebar-group__action:focus-visible,
.ui-sidebar-group__toggle:focus-visible {
  outline: 2px solid var(--ui-sidebar-group-accent-solid);
  outline-offset: 1px;
}

.ui-sidebar-group--label-hidden .ui-sidebar-group__label,
.ui-sidebar-group[data-show-label="false"] .ui-sidebar-group__label {
  display: none;
}

.ui-sidebar-group--action-hidden .ui-sidebar-group__action,
.ui-sidebar-group[data-show-action="false"] .ui-sidebar-group__action {
  display: none;
}

.ui-sidebar-group--disabled,
.ui-sidebar-group[data-disabled="true"] {
  opacity: var(--ui-sidebar-group-disabled-opacity);
}

.ui-sidebar-group--custom-class,
.ui-sidebar-group[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
