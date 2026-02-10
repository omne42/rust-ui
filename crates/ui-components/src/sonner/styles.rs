pub const CSS: &str = r#"
.ui-sonner {
  --ui-sonner-offset: 16px;
  pointer-events: none;
}

.ui-sonner__viewport.ui-toast-viewport {
  top: auto;
  right: auto;
  bottom: auto;
  left: auto;
}

.ui-sonner[data-motion-source="custom"],
.ui-sonner[data-custom-motion="true"] {
  --ui-sonner-custom-motion: 1;
}

.ui-sonner__viewport--top-left.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  left: var(--ui-sonner-offset);
}

.ui-sonner__viewport--top-center.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  left: 50%;
  transform: translateX(-50%);
}

.ui-sonner__viewport--top-right.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  right: var(--ui-sonner-offset);
}

.ui-sonner__viewport--bottom-left.ui-toast-viewport {
  bottom: var(--ui-sonner-offset);
  left: var(--ui-sonner-offset);
}

.ui-sonner__viewport--bottom-center.ui-toast-viewport {
  bottom: var(--ui-sonner-offset);
  left: 50%;
  transform: translateX(-50%);
}

.ui-sonner__viewport--bottom-right.ui-toast-viewport {
  right: var(--ui-sonner-offset);
  bottom: var(--ui-sonner-offset);
}

.ui-sonner[data-portal="false"] {
  width: 100%;
  display: flex;
}

.ui-sonner[data-portal="false"][data-position$="left"] {
  justify-content: flex-start;
}

.ui-sonner[data-portal="false"][data-position$="center"] {
  justify-content: center;
}

.ui-sonner[data-portal="false"][data-position$="right"] {
  justify-content: flex-end;
}

.ui-sonner[data-portal="false"] .ui-sonner__viewport.ui-toast-viewport {
  position: relative;
  transform: none;
  max-width: min(100%, 420px);
}
"#;
