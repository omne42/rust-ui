pub const CSS: &str = r#"
.ui-avatar {
  width: var(--ui-avatar-size, var(--ui-fallback-avatar-size-md));
  height: var(--ui-avatar-size, var(--ui-fallback-avatar-size-md));

  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  border-radius: var(--ui-avatar-radius, var(--ui-fallback-avatar-radius, var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))));
  overflow: hidden;
  box-sizing: border-box;

  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));

  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-avatar--sm,
.ui-avatar[data-size="sm"] {
  --ui-avatar-size: var(--ui-avatar-size-sm, var(--ui-fallback-avatar-size-sm));
}

.ui-avatar--md,
.ui-avatar[data-size="md"] {
  --ui-avatar-size: var(--ui-avatar-size-md, var(--ui-fallback-avatar-size-md));
}

.ui-avatar--lg,
.ui-avatar[data-size="lg"] {
  --ui-avatar-size: var(--ui-avatar-size-lg, var(--ui-fallback-avatar-size-lg));
}

.ui-avatar--image,
.ui-avatar[data-state="image"],
.ui-avatar[data-image="true"] {
  background: var(--ui-bg, var(--ui-fallback-bg));
}

.ui-avatar--fallback,
.ui-avatar[data-state="fallback"],
.ui-avatar[data-fallback="true"] {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
}

.ui-avatar--has-src.ui-avatar--image,
.ui-avatar[data-has-src="true"][data-state="image"],
.ui-avatar[data-has-src="true"][data-image="true"] {
  border-color: transparent;
}

.ui-avatar--label-alt,
.ui-avatar[data-label-source="alt"] {
  --ui-avatar-label-source: 1;
}

.ui-avatar--label-name,
.ui-avatar[data-label-source="name"] {
  --ui-avatar-label-source: 2;
}

.ui-avatar--label-fallback,
.ui-avatar[data-label-source="fallback"] {
  --ui-avatar-label-source: 3;
}

.ui-avatar--has-alt[data-fallback="true"],
.ui-avatar[data-has-alt="true"][data-fallback="true"] {
  box-shadow: inset 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width)) var(--ui-border, var(--ui-fallback-border));
}

.ui-avatar--custom-class,
.ui-avatar[data-custom-class="true"] {
  --ui-avatar-custom-class: 1;
}

.ui-avatar__img {
  width: 100%;
  height: 100%;
  display: block;
  object-fit: cover;
  object-position: center;
}

.ui-avatar__initials {
  font-size: calc(var(--ui-avatar-size, var(--ui-fallback-avatar-size-md)) / 2.5);
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--ui-fg, var(--ui-fallback-fg));
}
"#;
