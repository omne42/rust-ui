pub const CSS: &str = r#"
.ui-spacer {
  display: block;
  flex: 0 0 auto;
  --ui-spacer-size: var(--ui-space-md);
}

.ui-spacer--size-xs,
.ui-spacer[data-size="xs"] {
  --ui-spacer-size: var(--ui-space-xs);
}

.ui-spacer--size-sm,
.ui-spacer[data-size="sm"] {
  --ui-spacer-size: var(--ui-space-sm);
}

.ui-spacer--size-md,
.ui-spacer[data-size="md"] {
  --ui-spacer-size: var(--ui-space-md);
}

.ui-spacer--size-lg,
.ui-spacer[data-size="lg"] {
  --ui-spacer-size: var(--ui-space-lg);
}

.ui-spacer--size-xl,
.ui-spacer[data-size="xl"] {
  --ui-spacer-size: calc(var(--ui-space-lg) * 2);
}

.ui-spacer--axis-vertical,
.ui-spacer[data-axis="vertical"],
.ui-spacer[data-state="vertical"],
.ui-spacer[data-vertical="true"] {
  width: 1px;
  height: var(--ui-spacer-size);
}

.ui-spacer--axis-horizontal,
.ui-spacer[data-axis="horizontal"],
.ui-spacer[data-state="horizontal"],
.ui-spacer[data-horizontal="true"] {
  height: 1px;
  width: var(--ui-spacer-size);
}
"#;
