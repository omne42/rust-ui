pub const CSS: &str = r#"
.ui-breadcrumbs {
  display: block;
  color: var(--ui-fg);
}

.ui-breadcrumbs__list {
  list-style-type: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--ui-space-xs);
}

.ui-breadcrumbs__item {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  min-width: 0;
}

.ui-breadcrumbs__link {
  color: var(--ui-accent);
  text-decoration: none;
  font-weight: 500;
  -webkit-tap-highlight-color: transparent;
}

.ui-breadcrumbs__link:hover {
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

.ui-breadcrumbs__link:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-breadcrumbs__current {
  color: var(--ui-fg);
  font-weight: 650;
}

.ui-breadcrumbs__separator {
  color: var(--ui-fg-muted);
  user-select: none;
}
"#;
