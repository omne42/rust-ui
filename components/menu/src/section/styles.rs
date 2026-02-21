pub const CSS: &str = r#"
.ui-menu-section {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-menu-section--tone-default,
.ui-menu-section[data-tone="default"] {
  --ui-menu-section-title-color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 88%,
    var(--ui-fg, var(--ui-fallback-fg)) 12%
  );
}

.ui-menu-section--tone-quiet,
.ui-menu-section[data-tone="quiet"] {
  --ui-menu-section-title-color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 96%,
    var(--ui-bg, var(--ui-fallback-bg)) 4%
  );
}

.ui-menu-section__header {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 600;
  color: var(--ui-menu-section-title-color, var(--ui-fallback-fg-muted));
  padding-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding-block: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-menu-section__header[data-sticky="true"] {
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(
    var(--ui-underlay-backdrop-blur, var(--ui-fallback-underlay-backdrop-blur))
  );
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 88%,
    transparent 12%
  );
}

.ui-menu-section__items {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-menu-section__divider {
  border-bottom: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 88%,
      var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 12%
    );
  margin-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-menu-section--empty,
.ui-menu-section[data-empty="true"] {
  opacity: 0.76;
}

.ui-menu-section--disabled,
.ui-menu-section[data-disabled="true"] {
  opacity: 0.52;
}

.ui-menu-section--sticky-heading,
.ui-menu-section[data-sticky-heading="true"] {
  gap: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}

.ui-menu-section--divided,
.ui-menu-section[data-divided="true"] {
  padding-bottom: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-menu-section--custom-class,
.ui-menu-section[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 26%, transparent);
  outline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
}
"#;
