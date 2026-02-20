pub const CSS: &str = r#"
.ui-icon {
  --ui-icon-size-sm-token: var(--ui-font-size-150, 14px);
  --ui-icon-size-md-token: var(--ui-font-size-200, 16px);
  --ui-icon-size-lg-token: var(--ui-icon-size-100, 20px);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--ui-fg);
  line-height: var(--ui-line-height-100, 16px);
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
  color: var(--ui-fg);
}

.ui-icon--tone-muted,
.ui-icon[data-tone="muted"] {
  color: var(--ui-fg-muted);
}

.ui-icon--tone-accent,
.ui-icon[data-tone="accent"] {
  color: color-mix(in oklab, var(--ui-accent) 82%, var(--ui-fg) 18%);
}

.ui-icon--tone-danger,
.ui-icon[data-tone="danger"] {
  color: color-mix(in oklab, var(--ui-danger) 80%, var(--ui-fg) 20%);
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
