pub const CSS: &str = r#"
.ui-coachmark {
  display: inline-flex;
}

.ui-coachmark--state-disabled,
.ui-coachmark[data-state="disabled"] {
  opacity: 0.72;
}

.ui-coachmark--state-enabled,
.ui-coachmark[data-state="enabled"] {
  opacity: 1;
}

.ui-coachmark--with-asset .ui-coachmark__content,
.ui-coachmark[data-asset="present"] .ui-coachmark__content {
  display: grid;
  gap: var(--ui-space-sm);
}

.ui-coachmark__asset {
  inline-size: 100%;
}

.ui-coachmark__body {
  display: grid;
  gap: var(--ui-space-xs);
  color: var(--ui-fg-muted);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-coachmark__footer {
  display: grid;
  gap: var(--ui-space-sm);
}

.ui-coachmark__steps {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-coachmark__actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: var(--ui-space-xs);
}

.ui-coachmark[data-cta="none"] .ui-coachmark__actions {
  justify-content: flex-start;
}

.ui-coachmark__button {
  min-inline-size: 5.25rem;
}

.ui-coachmark__button--primary,
.ui-coachmark[data-cta="single"] .ui-coachmark__button {
  font-weight: 600;
}

.ui-coachmark__actions-extra {
  display: inline-flex;
  align-items: center;
}

.ui-coachmark--variant-help,
.ui-coachmark[data-variant="help"] {
  --ui-coachmark-accent: color-mix(in oklch, var(--ui-fg) 32%, var(--ui-accent));
}

.ui-coachmark--variant-info,
.ui-coachmark[data-variant="info"] {
  --ui-coachmark-accent: var(--ui-accent);
}

.ui-coachmark .ui-contextual-help__heading {
  color: var(--ui-coachmark-accent, var(--ui-fg));
}

.ui-coachmark--custom-class,
.ui-coachmark[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-coachmark[data-motion-source="custom"],
.ui-coachmark[data-custom-motion="true"] {
  --ui-coachmark-custom-motion: 1;
}

@media (forced-colors: active) {
  .ui-coachmark,
  .ui-coachmark * {
    forced-color-adjust: auto;
  }
}
"#;
