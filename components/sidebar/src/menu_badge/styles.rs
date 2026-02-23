pub const CSS: &str = r#"
.ui-sidebar-menu-badge {
  --ui-sidebar-menu-badge-bg: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 52%,
    transparent
  );
  --ui-sidebar-menu-badge-fg-muted: var(
    --ui-fg-muted,
    var(--ui-fallback-fg-muted)
  );
  --ui-sidebar-menu-badge-font-size-100: var(
    --ui-font-size-100,
    var(--ui-fallback-font-size-100)
  );
  --ui-sidebar-menu-badge-line-height-100: var(
    --ui-line-height-100,
    var(--ui-fallback-line-height-100)
  );
  --ui-sidebar-menu-badge-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-inline-start: auto;
  padding-inline: 0.45rem;
  border-radius: 999px;
  background: var(--ui-sidebar-menu-badge-bg);
  color: var(--ui-sidebar-menu-badge-fg-muted);
  font-size: var(--ui-sidebar-menu-badge-font-size-100);
  line-height: var(--ui-sidebar-menu-badge-line-height-100);
  font-variant-numeric: tabular-nums;
}

.ui-sidebar-menu-badge--muted,
.ui-sidebar-menu-badge[data-muted="true"],
.ui-sidebar-menu-badge[data-tone="muted"] {
  opacity: 0.78;
}

.ui-sidebar-menu-badge--disabled,
.ui-sidebar-menu-badge[data-disabled="true"] {
  opacity: var(--ui-sidebar-menu-badge-disabled-opacity);
}

.ui-sidebar-menu-badge--custom-class,
.ui-sidebar-menu-badge[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
