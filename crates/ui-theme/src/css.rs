pub const BASE_CSS: &str = r#"
:root {
  /* Generated tokens should be appended after this block. */
}
"#;

pub const SAFE_AREA_CSS: &str = r#"
.safe-area {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
"#;
