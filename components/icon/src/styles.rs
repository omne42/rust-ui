pub const CSS: &str = r#"
.ui-icon {
  --ui-icon-size-sm-token: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  --ui-icon-size-md-token: var(--ui-icon-size-200, var(--ui-fallback-icon-size-200));
  --ui-icon-size-lg-token: var(--ui-icon-size-100, var(--ui-fallback-icon-size-100));
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--ui-fg, var(--ui-fallback-fg));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  flex: 0 0 auto;
}

.ui-icon__glyph {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-icon--size-sm,
.ui-icon[data-size="sm"] {
  width: var(--ui-icon-size-sm-token);
  height: var(--ui-icon-size-sm-token);
  font-size: var(--ui-icon-size-sm-token);
}

.ui-icon--size-md,
.ui-icon[data-size="md"] {
  width: var(--ui-icon-size-md-token);
  height: var(--ui-icon-size-md-token);
  font-size: var(--ui-icon-size-md-token);
}

.ui-icon--size-lg,
.ui-icon[data-size="lg"] {
  width: var(--ui-icon-size-lg-token);
  height: var(--ui-icon-size-lg-token);
  font-size: var(--ui-icon-size-lg-token);
}

.ui-icon--tone-default,
.ui-icon[data-tone="default"] {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-icon--tone-muted,
.ui-icon[data-tone="muted"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-icon--tone-accent,
.ui-icon[data-tone="accent"] {
  color: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 82%,
    var(--ui-fg, var(--ui-fallback-fg)) 18%
  );
}

.ui-icon--tone-danger,
.ui-icon[data-tone="danger"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 80%,
    var(--ui-fg, var(--ui-fallback-fg)) 20%
  );
}

.ui-icon--disabled,
.ui-icon[data-disabled="true"] {
  opacity: 0.6;
}

.ui-icon--decorative,
.ui-icon[data-decorative="true"] {
  pointer-events: none;
}

.ui-icon--custom-class,
.ui-icon[data-custom-class="true"] {
  --ui-icon-custom-class: 1;
}
"#;
