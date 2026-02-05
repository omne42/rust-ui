pub const CSS: &str = r#"
.ui-link {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  color: var(--ui-accent);
  text-decoration: none;
  font-weight: 500;
  -webkit-tap-highlight-color: transparent;
}

.ui-link[data-hovered="true"] {
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

.ui-link--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-link[data-disabled="true"] {
  opacity: 0.55;
  cursor: not-allowed;
  text-decoration: none;
  pointer-events: none;
}
"#;
