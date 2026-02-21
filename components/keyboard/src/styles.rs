pub const CSS: &str = r#"
.ui-keyboard {
  --ui-keyboard-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-keyboard-border: var(--ui-border, var(--ui-fallback-border));
  --ui-keyboard-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-keyboard-bg: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 92%,
    var(--ui-bg, var(--ui-fallback-bg)) 8%
  );
  --ui-keyboard-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-keyboard-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-keyboard-min-height: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.625
  );
  --ui-keyboard-min-height-compact: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.5625
  );
  --ui-keyboard-padding-block: calc(var(--ui-space-2xs, var(--ui-fallback-space-2xs)) / 2);
  --ui-keyboard-padding-inline: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) * 0.75);
  --ui-keyboard-padding-block-compact: calc(
    var(--ui-space-2xs, var(--ui-fallback-space-2xs)) / 4
  );
  --ui-keyboard-padding-inline-compact: calc(
    var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2
  );
  --ui-keyboard-font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-keyboard-line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-keyboard-custom-outline: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 24%,
    transparent
  );

  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: var(--ui-keyboard-min-height);
  padding: var(--ui-keyboard-padding-block) var(--ui-keyboard-padding-inline);
  border: var(--ui-keyboard-border-width) solid var(--ui-keyboard-border);
  border-radius: var(--ui-keyboard-radius);
  background: var(--ui-keyboard-bg);
  color: var(--ui-keyboard-fg);
  font-size: var(--ui-keyboard-font-size);
  line-height: var(--ui-keyboard-line-height);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
}

.ui-keyboard--tone-default,
.ui-keyboard[data-tone="default"] {
  color: var(--ui-keyboard-fg);
}

.ui-keyboard--tone-muted,
.ui-keyboard[data-tone="muted"] {
  color: var(--ui-keyboard-fg-muted);
}

.ui-keyboard--compact,
.ui-keyboard[data-compact="true"] {
  min-height: var(--ui-keyboard-min-height-compact);
  padding: var(--ui-keyboard-padding-block-compact) var(--ui-keyboard-padding-inline-compact);
  font-size: var(--ui-keyboard-font-size);
  line-height: var(--ui-keyboard-line-height);
}

.ui-keyboard--custom-class,
.ui-keyboard[data-custom-class="true"] {
  outline: var(--ui-keyboard-border-width) solid var(--ui-keyboard-custom-outline);
}
"#;
