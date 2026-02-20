pub const CSS: &str = r#"
.ui-sidebar-menu-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-inline-start: auto;
  padding-inline: 0.45rem;
  border-radius: 999px;
  background: color-mix(in oklab, currentColor 12%, transparent);
  color: var(--ui-fg-muted, color-mix(in oklab, currentColor 70%, transparent));
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-variant-numeric: tabular-nums;
}

.ui-sidebar-menu-badge--muted,
.ui-sidebar-menu-badge[data-muted="true"],
.ui-sidebar-menu-badge[data-tone="muted"] {
  opacity: 0.78;
}

.ui-sidebar-menu-badge--disabled,
.ui-sidebar-menu-badge[data-disabled="true"] {
  opacity: 0.52;
}

.ui-sidebar-menu-badge--custom-class,
.ui-sidebar-menu-badge[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
