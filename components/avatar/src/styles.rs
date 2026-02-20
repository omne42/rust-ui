pub const CSS: &str = r#"
.ui-avatar {
  width: var(--ui-avatar-size, 32px);
  height: var(--ui-avatar-size, 32px);

  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  border-radius: 9999px;
  overflow: hidden;
  box-sizing: border-box;

  background: var(--ui-bg-muted);
  border: 1px solid var(--ui-border);
  color: var(--ui-fg);

  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-avatar--sm,
.ui-avatar[data-size="sm"] {
  --ui-avatar-size: 24px;
}

.ui-avatar--md,
.ui-avatar[data-size="md"] {
  --ui-avatar-size: 32px;
}

.ui-avatar--lg,
.ui-avatar[data-size="lg"] {
  --ui-avatar-size: 40px;
}

.ui-avatar--image,
.ui-avatar[data-state="image"],
.ui-avatar[data-image="true"] {
  background: var(--ui-bg);
}

.ui-avatar--fallback,
.ui-avatar[data-state="fallback"],
.ui-avatar[data-fallback="true"] {
  background: var(--ui-bg-muted);
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
  box-shadow: inset 0 0 0 1px var(--ui-border);
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
  font-size: calc(var(--ui-avatar-size, 32px) / 2.5);
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--ui-fg);
}
"#;
