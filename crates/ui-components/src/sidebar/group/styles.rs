pub const CSS: &str = r#"
.ui-sidebar-group {
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
  line-height: 1.2;
  border-radius: var(--ui-radius-xs, 0.375rem);
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
  transition: transform 160ms ease;
}

.ui-sidebar-group__toggle[data-open="true"] {
  transform: rotate(180deg);
}

.ui-sidebar-group__content {
  display: grid;
  gap: 0.3rem;
  padding-inline-start: 0.2rem;
  border-inline-start: 1px solid
    var(--ui-border-subtle, color-mix(in oklab, currentColor 20%, transparent));
  transition: opacity 160ms ease, transform 160ms ease;
}

.ui-sidebar-group[data-state="closed"] .ui-sidebar-group__content {
  opacity: 0;
  transform: translateY(-0.25rem);
}

.ui-sidebar-group__label:hover,
.ui-sidebar-group__action:hover,
.ui-sidebar-group__toggle:hover {
  background: color-mix(in oklab, currentColor 10%, transparent);
}

.ui-sidebar-group__label:focus-visible,
.ui-sidebar-group__action:focus-visible,
.ui-sidebar-group__toggle:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
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
  opacity: 0.62;
}

.ui-sidebar-group--custom-class,
.ui-sidebar-group[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
