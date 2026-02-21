pub const CSS: &str = r#"
.ui-file-trigger {
  --ui-file-trigger-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-file-trigger-disabled-opacity: var(--ui-drop-zone-disabled-opacity, var(--ui-fallback-drop-zone-disabled-opacity));
  --ui-file-trigger-sr-only-size: var(--ui-drop-zone-sr-only-size, var(--ui-fallback-drop-zone-sr-only-size));
  --ui-file-trigger-zero: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none));

  display: inline-flex;
  align-items: center;
  gap: var(--ui-file-trigger-gap);
}

.ui-file-trigger--disabled,
.ui-file-trigger[data-disabled="true"] {
  opacity: var(--ui-file-trigger-disabled-opacity);
}

.ui-file-trigger[data-motion-source="custom"],
.ui-file-trigger--custom-motion,
.ui-file-trigger[data-custom-motion="true"] {
  --ui-file-trigger-custom-motion: 1;
}

.ui-file-trigger__input {
  position: absolute;
  width: var(--ui-file-trigger-sr-only-size);
  height: var(--ui-file-trigger-sr-only-size);
  padding: var(--ui-file-trigger-zero);
  margin: calc(-1 * var(--ui-file-trigger-sr-only-size));
  overflow: hidden;
  clip: rect(
    var(--ui-file-trigger-zero),
    var(--ui-file-trigger-zero),
    var(--ui-file-trigger-zero),
    var(--ui-file-trigger-zero)
  );
  white-space: nowrap;
  border-width: var(--ui-file-trigger-zero);
}
"#;
