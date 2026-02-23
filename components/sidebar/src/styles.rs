pub const CSS: &str = r#"
.ui-sidebar {
  --ui-sidebar-space-3xs: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
  --ui-sidebar-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-sidebar-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-sidebar-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-sidebar-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-sidebar-space-xl: var(--ui-space-xl, var(--ui-fallback-space-xl));
  --ui-sidebar-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-sidebar-radius-md: var(--ui-radius-md, var(--ui-fallback-radius-md));
  --ui-sidebar-radius-full: var(--ui-radius-full, var(--ui-fallback-radius-full));
  --ui-sidebar-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-sidebar-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-sidebar-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-sidebar-bg-muted: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  --ui-sidebar-border: var(--ui-border, var(--ui-fallback-border));
  --ui-sidebar-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-sidebar-focus-ring: var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  --ui-sidebar-disabled-opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
  --ui-sidebar-open-width: calc(var(--ui-sidebar-space-xl) * 4);
  --ui-sidebar-icon-width: calc(var(--ui-sidebar-space-xl) - var(--ui-sidebar-space-2xs));
  --ui-sidebar-trigger-pad-y: calc(
    var(--ui-sidebar-space-2xs) - (var(--ui-sidebar-space-3xs) / 2)
  );
  --ui-sidebar-trigger-pad-x: calc(
    var(--ui-sidebar-space-sm) + (var(--ui-sidebar-space-3xs) / 2)
  );
  --ui-sidebar-panel-gap: calc(var(--ui-sidebar-space-sm) + (var(--ui-sidebar-space-3xs) / 2));
  --ui-sidebar-panel-pad: calc(var(--ui-sidebar-space-sm) + (var(--ui-sidebar-space-3xs) / 2));
  --ui-sidebar-section-gap: calc(var(--ui-sidebar-space-sm) - (var(--ui-sidebar-space-3xs) / 2));
  --ui-sidebar-header-pad-b: calc(
    var(--ui-sidebar-space-2xs) - (var(--ui-sidebar-space-3xs) / 2)
  );
  --ui-sidebar-header-pad-i: var(--ui-sidebar-space-2xs);
  --ui-sidebar-content-pad: var(--ui-sidebar-space-2xs);
  --ui-sidebar-gap: var(--ui-sidebar-space-2xs);
  --ui-sidebar-rail-width: calc(var(--ui-sidebar-space-sm) - (var(--ui-sidebar-space-3xs) / 2));
  --ui-sidebar-focus-outline-offset: var(--ui-sidebar-space-3xs);
  --ui-sidebar-min-height: calc((var(--ui-sidebar-space-xl) * 3) + var(--ui-sidebar-space-md));
  --ui-sidebar-panel-surface: color-mix(
    in oklab,
    var(--ui-sidebar-bg) 94%,
    var(--ui-sidebar-bg-muted) 6%
  );
  --ui-sidebar-panel-inset-surface: color-mix(
    in oklab,
    var(--ui-sidebar-bg) 90%,
    var(--ui-sidebar-bg-muted) 10%
  );
  --ui-sidebar-border-subtle: color-mix(in oklab, var(--ui-sidebar-border) 28%, transparent);
  --ui-sidebar-accent-outline: color-mix(
    in oklab,
    var(--ui-sidebar-focus-ring) 68%,
    transparent
  );
  --ui-sidebar-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-sidebar-motion-reduced-duration: 1ms;
  --ui-sidebar-motion-runtime-duration: var(--ui-sidebar-motion-duration);
  --ui-sidebar-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-sidebar-motion-shift-distance: var(--ui-sidebar-space-md);
  display: grid;
  grid-template-columns: auto auto;
  align-items: stretch;
  gap: var(--ui-sidebar-gap);
  min-height: var(--ui-sidebar-min-height);
  color: inherit;
}

.ui-sidebar__trigger {
  justify-self: start;
  align-self: start;
  border: var(--ui-sidebar-border-width) solid var(--ui-sidebar-border-subtle);
  border-radius: var(--ui-sidebar-radius-sm);
  background: var(--ui-sidebar-panel-surface);
  color: inherit;
  padding: var(--ui-sidebar-trigger-pad-y) var(--ui-sidebar-trigger-pad-x);
  font: inherit;
  line-height: var(--ui-sidebar-line-height-100);
}

.ui-sidebar__trigger:focus-visible,
.ui-sidebar__rail:focus-visible {
  outline: var(--ui-sidebar-border-width) solid var(--ui-sidebar-accent-outline);
  outline-offset: var(--ui-sidebar-focus-outline-offset);
}

.ui-sidebar__panel {
  grid-column: 1;
  display: grid;
  gap: var(--ui-sidebar-panel-gap);
  width: var(--ui-sidebar-open-width);
  min-width: 0;
  padding: var(--ui-sidebar-panel-pad);
  border: var(--ui-sidebar-border-width) solid var(--ui-sidebar-border-subtle);
  border-radius: var(--ui-sidebar-radius-md);
  background: var(--ui-sidebar-panel-surface);
  transition:
    width var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing),
    transform var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing),
    opacity var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing),
    border-color var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing),
    padding var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing);
}

.ui-sidebar__header,
.ui-sidebar__content,
.ui-sidebar__footer {
  display: grid;
  gap: var(--ui-sidebar-section-gap);
}

.ui-sidebar__header,
.ui-sidebar__footer {
  padding: var(--ui-sidebar-header-pad-b) var(--ui-sidebar-header-pad-i);
}

.ui-sidebar__content {
  min-height: 0;
  overflow: auto;
  padding: var(--ui-sidebar-content-pad);
}

.ui-sidebar__rail {
  grid-column: 2;
  align-self: stretch;
  width: var(--ui-sidebar-rail-width);
  border: 0;
  border-radius: var(--ui-sidebar-radius-full);
  background: color-mix(in oklab, var(--ui-sidebar-border) 40%, transparent);
  cursor: pointer;
  transition:
    background var(--ui-sidebar-motion-runtime-duration) var(--ui-sidebar-motion-easing);
}

.ui-sidebar__rail:hover {
  background: color-mix(in oklab, var(--ui-sidebar-accent) 30%, transparent);
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
  transform: translateX(calc(-1 * var(--ui-sidebar-motion-shift-distance)));
  pointer-events: none;
  overflow: hidden;
}

.ui-sidebar[data-side="right"][data-state="closed"][data-collapsible="offcanvas"] .ui-sidebar__panel {
  transform: translateX(var(--ui-sidebar-motion-shift-distance));
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
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}

.ui-sidebar[data-variant="inset"] .ui-sidebar__panel {
  background: var(--ui-sidebar-panel-inset-surface);
}

.ui-sidebar--disabled,
.ui-sidebar[data-disabled="true"] {
  opacity: var(--ui-sidebar-disabled-opacity);
}

.ui-sidebar--custom-class,
.ui-sidebar[data-custom-class="true"] {
  border-radius: inherit;
}

@media (prefers-reduced-motion: reduce) {
  .ui-sidebar {
    --ui-sidebar-motion-runtime-duration: var(--ui-sidebar-motion-reduced-duration);
  }
}
"#;
