component "ui-field" {
  crate: "ui-field"
  mode: "snapshot"
  exports: [
    "Field",
    "FieldMotion",
    "FieldOrientation",
    "FieldTone",
    "A11yDirection",
    "FieldComponentSchemaVersion",
    "FieldComponentSpec",
    "styles::CSS",
    "group::FieldGroup (feature=field-group)",
    "group::FieldGroupOrientation (feature=field-group)",
    "group::FieldGroupDensity (feature=field-group)",
    "group::GroupComponentSchemaVersion (feature=field-group)",
    "group::GroupComponentSpec (feature=field-group)",
    "group::styles::CSS (feature=field-group)"
  ]

  signature Field(
    orientation?: FieldOrientation,
    tone?: FieldTone,
    is_required?: Option<bool>,
    required?: Option<bool>,
    is_disabled?: Option<bool>,
    disabled?: Option<bool>,
    is_invalid?: Option<bool>,
    invalid?: Option<bool>,
    label?: Option<String>,
    description?: Option<String>,
    error_message?: Option<String>,
    motion?: FieldMotion,
    aria_label?: Option<String>,
    lang?: Option<String>,
    dir?: Option<A11yDirection>,
    class_name?: Option<String>,
    children: Children
  ) -> impl IntoView

  signature FieldGroup(
    orientation?: FieldGroupOrientation,
    density?: FieldGroupDensity,
    is_disabled?: Option<bool>,
    disabled?: Option<bool>,
    is_invalid?: Option<bool>,
    invalid?: Option<bool>,
    id_base?: Option<String>,
    label?: Option<String>,
    description?: Option<String>,
    aria_label?: Option<String>,
    lang?: Option<String>,
    dir?: Option<A11yDirection>,
    class_name?: Option<String>,
    children: Children
  ) -> impl IntoView [feature="field-group"]

  streaming_policy {
    required: false
    fallback: "snapshot"
    owner: "upstream"
  }

  llm_render_modes {
    allowed: ["streaming", "snapshot"]
    default: "snapshot"
  }

  semantic_markers: [
    "data-slot",
    "data-state",
    "data-orientation",
    "data-tone",
    "data-required",
    "data-disabled",
    "data-invalid",
    "data-required-source",
    "data-disabled-source",
    "data-invalid-source",
    "data-motion-source",
    "data-aria-source",
    "data-error-source",
    "data-class-source",
    "data-ui-schema",
    "data-ui-schema-version",
    "data-ui-intent",
    "data-ui-action",
    "data-ui-state",
    "data-ui-source",
    "data-ui-source-required",
    "data-ui-source-disabled",
    "data-ui-source-invalid",
    "data-ui-source-motion",
    "data-ui-source-aria",
    "data-ui-source-error",
    "data-ui-source-class",
    "data-ui-stream-mode",
    "data-ui-stream-support",
    "data-ui-stream-fallback",
    "data-ui-output-mode",
    "data-ui-output-status"
  ]

  agent_contract_schema "ui.field.agent-contract/v1" {
    fields: [
      "intent",
      "action",
      "state",
      "source",
      "source.required",
      "source.disabled",
      "source.invalid",
      "source.motion",
      "source.aria",
      "source.error",
      "source.class",
      "stream.mode",
      "stream.support",
      "stream.fallback",
      "output.mode",
      "output.status"
    ]
  }

  whitelist "render_path" {
    allowed: [
      "logic::resolve_content",
      "logic::resolve_state",
      "logic::resolve_agent_contract",
      "view::Field",
      "group::logic::resolve_agent_contract",
      "group::view::FieldGroup",
      "motion::attach_motion"
    ]
    blocked: ["inner_html", "dangerously_set_inner_html", "<script", "javascript:"]
  }
}
