pub const CSS: &str = r#"
.ui-sidenav {
  display: block;
}

.ui-sidenav[data-state="disabled"] {
  opacity: 0.62;
}

.ui-sidenav[data-open-mode="controlled"] {
  --ui-sidenav-open-mode: controlled;
}

.ui-sidenav[data-open-mode="uncontrolled"] {
  --ui-sidenav-open-mode: uncontrolled;
}

.ui-sidenav[data-initial-open="open"] {
  --ui-sidenav-initial-open: open;
}

.ui-sidenav[data-initial-open="closed"] {
  --ui-sidenav-initial-open: closed;
}

.ui-sidenav[data-trigger-mode="visible"] {
  --ui-sidenav-trigger-mode: visible;
}

.ui-sidenav[data-trigger-mode="hidden"] {
  --ui-sidenav-trigger-mode: hidden;
}

.ui-sidenav[data-shortcut-mode="enabled"] {
  --ui-sidenav-shortcut-mode: enabled;
}

.ui-sidenav[data-shortcut-mode="disabled"] {
  --ui-sidenav-shortcut-mode: disabled;
}

.ui-sidenav[data-label-source="custom"] {
  --ui-sidenav-label-source: custom;
}

.ui-sidenav[data-trigger-source="custom"] {
  --ui-sidenav-trigger-source: custom;
}

.ui-sidenav[data-shortcut-source="custom"] {
  --ui-sidenav-shortcut-source: custom;
}

.ui-sidenav[data-class-source="custom"] {
  --ui-sidenav-class-source: custom;
}

.ui-sidenav[data-handler-source="custom"] {
  --ui-sidenav-handler-source: custom;
}

.ui-sidenav--custom-class,
.ui-sidenav[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
