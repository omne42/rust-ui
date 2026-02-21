pub const CSS: &str = r#"
.ui-dropdown {
  display: inline-flex;
}

.ui-dropdown--disabled,
.ui-dropdown[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-dropdown--persistent,
.ui-dropdown[data-keep-open-on-action="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 64%,
      var(--ui-accent, var(--ui-fallback-accent)) 36%
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-dropdown--custom-class,
.ui-dropdown[data-custom-class="true"] {
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-dropdown__trigger {
  min-width: 0;
}

.ui-dropdown[data-motion-source="custom"],
.ui-dropdown[data-custom-motion="true"] {
  --ui-dropdown-custom-motion: 1;
}
"#;
