pub const CSS: &str = r#"
.ui-sidebar {
  --ui-sidebar-open-width: 16rem;
  --ui-sidebar-icon-width: 3.5rem;
  --ui-sidebar-transition-ease: cubic-bezier(0.22, 1, 0.36, 1);
  display: grid;
  grid-template-columns: auto auto;
  align-items: stretch;
  gap: 0.35rem;
  min-height: 14rem;
  color: inherit;
}

.ui-sidebar__trigger {
  justify-self: start;
  align-self: start;
  border: 1px solid var(--ui-border-subtle, color-mix(in oklab, currentColor 22%, transparent));
  border-radius: var(--ui-radius-sm, 0.5rem);
  background: var(--ui-bg-surface, color-mix(in oklab, currentColor 4%, transparent));
  color: inherit;
  padding: 0.35rem 0.6rem;
  font: inherit;
  line-height: var(--ui-line-height-100, 16px);
}

.ui-sidebar__trigger:focus-visible,
.ui-sidebar__rail:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  outline-offset: 1px;
}

.ui-sidebar__panel {
  grid-column: 1;
  display: grid;
  gap: 0.625rem;
  width: var(--ui-sidebar-open-width);
  min-width: 0;
  padding: 0.65rem;
  border: 1px solid var(--ui-border-subtle, color-mix(in oklab, currentColor 22%, transparent));
  border-radius: var(--ui-radius-md, 0.75rem);
  background: var(--ui-bg-surface, color-mix(in oklab, currentColor 4%, transparent));
  transition:
    width 220ms var(--ui-sidebar-transition-ease),
    transform 220ms var(--ui-sidebar-transition-ease),
    opacity 180ms ease,
    border-color 180ms ease,
    padding 220ms var(--ui-sidebar-transition-ease);
}

.ui-sidebar__header,
.ui-sidebar__content,
.ui-sidebar__footer {
  display: grid;
  gap: 0.45rem;
}

.ui-sidebar__header,
.ui-sidebar__footer {
  padding: 0.2rem 0.25rem;
}

.ui-sidebar__content {
  min-height: 0;
  overflow: auto;
  padding: 0.25rem;
}

.ui-sidebar__rail {
  grid-column: 2;
  align-self: stretch;
  width: 0.625rem;
  border: 0;
  border-radius: 999px;
  background: color-mix(in oklab, currentColor 12%, transparent);
  cursor: pointer;
  transition: background 160ms ease;
}

.ui-sidebar__rail:hover {
  background: color-mix(in oklab, var(--ui-accent-solid, currentColor) 30%, transparent);
}

.ui-sidebar[data-side="right"] {
  justify-content: end;
}

.ui-sidebar[data-side="right"] .ui-sidebar__panel {
  grid-column: 2;
}

.ui-sidebar[data-side="right"] .ui-sidebar__rail {
  grid-column: 1;
}

.ui-sidebar[data-state="closed"][data-collapsible="offcanvas"] .ui-sidebar__panel {
  width: 0;
  opacity: 0;
  border-color: transparent;
  padding-inline: 0;
  transform: translateX(-0.8rem);
  pointer-events: none;
  overflow: hidden;
}

.ui-sidebar[data-side="right"][data-state="closed"][data-collapsible="offcanvas"] .ui-sidebar__panel {
  transform: translateX(0.8rem);
}

.ui-sidebar[data-state="closed"][data-collapsible="icon"] .ui-sidebar__panel {
  width: var(--ui-sidebar-icon-width);
  overflow: hidden;
}

.ui-sidebar[data-state="closed"][data-collapsible="none"] .ui-sidebar__rail {
  display: none;
}

.ui-sidebar[data-state="closed"][data-collapsible="icon"] .ui-sidebar__header,
.ui-sidebar[data-state="closed"][data-collapsible="icon"] .ui-sidebar__footer {
  opacity: 0;
  pointer-events: none;
}

.ui-sidebar[data-variant="floating"] .ui-sidebar__panel {
  box-shadow: 0 18px 38px -30px color-mix(in oklab, currentColor 45%, transparent);
}

.ui-sidebar[data-variant="inset"] .ui-sidebar__panel {
  background: color-mix(in oklab, var(--ui-bg-canvas, white) 92%, transparent);
}

.ui-sidebar--disabled,
.ui-sidebar[data-disabled="true"] {
  opacity: 0.62;
}

.ui-sidebar--custom-class,
.ui-sidebar[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
