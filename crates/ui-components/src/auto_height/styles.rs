pub const CSS: &str = r#"
.ui-auto-height {
  overflow: hidden;
  height: var(--ui-auto-height-height, auto);
  --ui-auto-height-motion-kind: default;
  --ui-auto-height-class-kind: base;
}

.ui-auto-height--animated,
.ui-auto-height[data-state="animated"],
.ui-auto-height[data-animated="true"] {
  will-change: height;
}

.ui-auto-height--static,
.ui-auto-height[data-state="static"],
.ui-auto-height[data-static="true"] {
  will-change: auto;
}

.ui-auto-height[data-overflow-hidden="true"] {
  overflow: hidden;
}

.ui-auto-height--custom-motion,
.ui-auto-height[data-custom-motion="true"] {
  --ui-auto-height-motion-kind: custom;
}

.ui-auto-height--custom-class,
.ui-auto-height[data-custom-class="true"] {
  --ui-auto-height-class-kind: custom;
}

.ui-auto-height__content {
  width: 100%;
  min-width: 0;
}
"#;
