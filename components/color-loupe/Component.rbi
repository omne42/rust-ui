component "ui-color-loupe" {
  crate: "ui-color-loupe"
  mode: "snapshot"
  exports: [
    "ColorLoupe",
    "ColorLoupeOutputState",
    "ColorLoupeState",
    "ColorLoupeStateInput",
    "DEFAULT_ARIA_LABEL",
    "DEFAULT_COLOR"
  ]

  signature ColorLoupe(
    id_base: String,
    color?: Option<String>,
    is_open?: bool,
    is_disabled?: bool,
    x_percent?: Option<f32>,
    y_percent?: Option<f32>,
    aria_label?: Option<String>,
    class_name?: Option<String>,
    output_state?: Option<ColorLoupeOutputState>,
    lang?: Option<String>,
    dir?: Option<A11yDirection>
  ) -> impl IntoView

  streaming_policy {
    required: false
    fallback: "snapshot"
    owner: "upstream"
  }

  semantic_markers: [
    "data-ui-schema",
    "data-output-state",
    "data-state",
    "data-open",
    "data-disabled",
    "data-has-color",
    "data-x",
    "data-y",
    "data-x-bucket",
    "data-y-bucket",
    "data-aria-source",
    "data-custom-class",
    "data-class-source"
  ]

  agent_contract_schema "ui-color-loupe/v1" {
    fields: [
      "intent",
      "action",
      "state",
      "output_state",
      "source.aria",
      "source.class",
      "x_bucket",
      "y_bucket"
    ]
  }
}
