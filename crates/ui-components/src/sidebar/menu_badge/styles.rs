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
  font-size: 0.75rem;
  line-height: 1.4;
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
