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

.ui-link--enabled,
.ui-link[data-state="enabled"],
.ui-link[data-enabled="true"] {
  color: var(--ui-accent);
}

.ui-link--external,
.ui-link[data-external="true"],
.ui-link[data-target="blank"] {
  text-underline-offset: 2px;
}

.ui-link[data-hovered="true"] {
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

.ui-link--focus-visible,
.ui-link[data-focus-visible="true"] {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-link--disabled,
.ui-link[data-state="disabled"],
.ui-link[data-disabled="true"] {
  opacity: 0.55;
  cursor: not-allowed;
  text-decoration: none;
  pointer-events: none;
}

.ui-link--missing-href,
.ui-link[data-state="missing-href"],
.ui-link[data-missing-href="true"] {
  opacity: 0.65;
  cursor: default;
  text-decoration: none;
  pointer-events: none;
}

.ui-link--rel-provided,
.ui-link[data-rel="provided"] {
  --ui-link-rel-source: 1;
}

.ui-link--with-aria-label,
.ui-link[data-aria-label="custom"] {
  --ui-link-aria-label-source: 1;
}

.ui-link--custom-class,
.ui-link[data-custom-class="true"] {
  --ui-link-custom-class: 1;
}
"#;
