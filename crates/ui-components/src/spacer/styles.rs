pub const CSS: &str = r#"
.ui-spacer {
  display: block;
  flex: 0 0 auto;
}

.ui-spacer--axis-vertical {
  width: 1px;
}

.ui-spacer--axis-horizontal {
  height: 1px;
}

.ui-spacer--axis-vertical.ui-spacer--size-xs {
  height: var(--ui-space-xs);
}

.ui-spacer--axis-vertical.ui-spacer--size-sm {
  height: var(--ui-space-sm);
}

.ui-spacer--axis-vertical.ui-spacer--size-md {
  height: var(--ui-space-md);
}

.ui-spacer--axis-vertical.ui-spacer--size-lg {
  height: var(--ui-space-lg);
}

.ui-spacer--axis-vertical.ui-spacer--size-xl {
  height: calc(var(--ui-space-lg) * 2);
}

.ui-spacer--axis-horizontal.ui-spacer--size-xs {
  width: var(--ui-space-xs);
}

.ui-spacer--axis-horizontal.ui-spacer--size-sm {
  width: var(--ui-space-sm);
}

.ui-spacer--axis-horizontal.ui-spacer--size-md {
  width: var(--ui-space-md);
}

.ui-spacer--axis-horizontal.ui-spacer--size-lg {
  width: var(--ui-space-lg);
}

.ui-spacer--axis-horizontal.ui-spacer--size-xl {
  width: calc(var(--ui-space-lg) * 2);
}
"#;
