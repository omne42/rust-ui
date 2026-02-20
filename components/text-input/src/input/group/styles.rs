pub const CSS: &str = r#"
.ui-input-group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  width: min(100%, 28rem);
  color: var(--ui-fg);
}

.ui-input-group__control {
  display: flex;
  align-items: stretch;
  min-height: 36px;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  overflow: hidden;
}

.ui-input-group__addon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2.25rem;
  padding: 0 var(--ui-space-sm);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  color: var(--ui-fg-muted);
  background: color-mix(in oklch, var(--ui-bg-muted), var(--ui-bg) 22%);
  border-inline-end: 1px solid color-mix(in oklch, var(--ui-border), transparent 24%);
}

.ui-input-group__addon--end {
  border-inline-end: 0;
  border-inline-start: 1px solid color-mix(in oklch, var(--ui-border), transparent 24%);
}

.ui-input-group__field {
  flex: 1 1 auto;
  min-width: 0;
  display: flex;
  align-items: stretch;
}

.ui-input-group__field > * {
  flex: 1 1 auto;
  min-width: 0;
}

.ui-input-group__field > .ui-input {
  gap: 0;
}

.ui-input-group__field > .ui-input .ui-input__control {
  height: 100%;
  border: 0;
  border-radius: 0;
  background: transparent;
  padding-inline: var(--ui-space-sm);
}

.ui-input-group__field > .ui-input.ui-input--focus-visible .ui-input__control {
  outline: 0;
}

.ui-input-group--invalid .ui-input-group__control,
.ui-input-group[data-invalid="true"] .ui-input-group__control {
  border-color: var(--ui-danger);
}

.ui-input-group--state-disabled,
.ui-input-group[data-state="disabled"],
.ui-input-group[data-disabled="true"] {
  opacity: 0.62;
}

.ui-input-group--state-disabled .ui-input-group__control,
.ui-input-group[data-disabled="true"] .ui-input-group__control {
  background: var(--ui-bg-muted);
}

.ui-input-group--state-disabled .ui-input-group__addon,
.ui-input-group[data-disabled="true"] .ui-input-group__addon {
  cursor: not-allowed;
}

.ui-input-group--detached .ui-input-group__control,
.ui-input-group[data-attachment="detached"] .ui-input-group__control {
  gap: var(--ui-space-xs);
  border: 0;
  background: transparent;
  overflow: visible;
}

.ui-input-group--detached .ui-input-group__addon,
.ui-input-group[data-attachment="detached"] .ui-input-group__addon {
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg-muted);
}

.ui-input-group--detached .ui-input-group__field > .ui-input .ui-input__control,
.ui-input-group[data-attachment="detached"] .ui-input-group__field > .ui-input .ui-input__control {
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
}

.ui-input-group--label-custom,
.ui-input-group[data-label-source="custom"] {
  color: color-mix(in oklch, var(--ui-fg), var(--ui-accent) 8%);
}

.ui-input-group--custom-class,
.ui-input-group[data-custom-class="true"] {
  isolation: isolate;
}
"#;
