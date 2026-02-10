pub const CSS: &str = r#"
.ui-toaster {
  pointer-events: none;
}

.ui-toaster__sonner.ui-sonner {
  width: 100%;
}

.ui-toaster[data-motion-source="custom"],
.ui-toaster[data-custom-motion="true"] {
  --ui-toaster-custom-motion: 1;
}

.ui-toaster[data-portal="false"] {
  display: flex;
  width: 100%;
}

.ui-toaster[data-portal="false"][data-position$="left"] {
  justify-content: flex-start;
}

.ui-toaster[data-portal="false"][data-position$="center"] {
  justify-content: center;
}

.ui-toaster[data-portal="false"][data-position$="right"] {
  justify-content: flex-end;
}
"#;
