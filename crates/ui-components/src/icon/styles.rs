pub const CSS: &str = r#"
.ui-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--ui-fg);
  line-height: 1;
  flex: 0 0 auto;
}

.ui-icon__glyph {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-icon--size-sm,
.ui-icon[data-size="sm"] {
  width: 0.875rem;
  height: 0.875rem;
  font-size: 0.875rem;
}

.ui-icon--size-md,
.ui-icon[data-size="md"] {
  width: 1rem;
  height: 1rem;
  font-size: 1rem;
}

.ui-icon--size-lg,
.ui-icon[data-size="lg"] {
  width: 1.25rem;
  height: 1.25rem;
  font-size: 1.25rem;
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
