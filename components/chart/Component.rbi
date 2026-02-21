component "ui-chart" {
  crate: "ui-chart"
  mode: "snapshot"
  exports: [
    "Chart",
    "ChartKind",
    "ChartPoint",
    "ChartMotion",
    "DEFAULT_ARIA_LABEL",
    "DEFAULT_ID_BASE",
    "styles::CSS"
  ]

  signature Chart(
    points: Vec<ChartPoint>,
    id_base?: Option<String>,
    kind?: ChartKind,
    active_index?: Option<leptos::prelude::Signal<usize>>,
    default_active_index?: Option<usize>,
    on_active_index_change?: Option<leptos::prelude::Callback<usize>>,
    on_action?: Option<leptos::prelude::Callback<String>>,
    is_disabled?: bool,
    is_show_grid?: bool,
    motion?: ChartMotion,
    aria_label?: Option<String>,
    class_name?: Option<String>,
    lang?: Option<String>,
    dir?: Option<ui_headless::A11yDirection>
  ) -> impl leptos::prelude::IntoView

  streaming_policy {
    required: false
    fallback: "snapshot"
    owner: "upstream"
  }

  semantic_markers: [
    "data-slot",
    "data-kind",
    "data-state",
    "data-empty",
    "data-has-points",
    "data-disabled",
    "data-enabled",
    "data-show-grid",
    "data-controlled",
    "data-uncontrolled",
    "data-active-index",
    "data-active-value-source",
    "data-active-interaction-source",
    "data-class-source",
    "data-custom-class",
    "data-motion-source",
    "data-custom-motion",
    "data-ui-schema",
    "data-ui-schema-version",
    "data-ui-intent",
    "data-ui-action",
    "data-ui-kind",
    "data-ui-state",
    "data-ui-source",
    "data-ui-active-value-source",
    "data-ui-interaction-source",
    "data-ui-motion-source",
    "data-ui-stream-support",
    "data-ui-stream-fallback",
    "data-ui-stream-mode",
    "data-ui-output-status",
    "data-ui-config-policy"
  ]

  agent_contract_schema "ui.chart.agent-contract/v1" {
    fields: [
      "kind",
      "state",
      "active_index",
      "source.active_value",
      "source.interaction",
      "source.motion",
      "output.status",
      "stream.fallback"
    ]
  }
}
