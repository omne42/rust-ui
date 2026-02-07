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

.ui-avatar--sm {
  --ui-avatar-size: 24px;
}

.ui-avatar--md {
  --ui-avatar-size: 32px;
}

.ui-avatar--lg {
  --ui-avatar-size: 40px;
}

.ui-avatar--has-src.ui-avatar--image {
  border-color: transparent;
}

.ui-avatar--label-fallback {
  background: var(--ui-bg-muted);
}

.ui-avatar--has-alt[data-fallback="true"] {
  box-shadow: inset 0 0 0 1px var(--ui-border);
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
  line-height: 1;
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--ui-fg);
}

.ui-avatar--fallback {
  background: var(--ui-bg-muted);
}
"#;
