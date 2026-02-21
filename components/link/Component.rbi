component "ui-link" {
  crate: "ui-link"
  mode: "snapshot"
  exports: [
    "Link",
    "styles::CSS"
  ]

  signature Link(
    href: String,
    is_disabled?: Option<bool>,
    target?: Option<&'static str>,
    rel?: Option<String>,
    aria_label?: Option<String>,
    class_name?: Option<String>,
    lang?: Option<String>,
    dir?: Option<A11yDirection>
  ) -> impl IntoView

  streaming_policy {
    required: false
    fallback: "snapshot"
    owner: "upstream"
  }

  semantic_markers: [
    "data-state",
    "data-enabled",
    "data-disabled",
    "data-disabled-source",
    "data-missing-href",
    "data-target",
    "data-external",
    "data-rel",
    "data-aria-label",
    "data-custom-class",
    "data-ui-schema",
    "data-ui-schema-version",
    "data-ui-intent",
    "data-ui-action",
    "data-ui-state",
    "data-ui-source",
    "data-ui-stream-support",
    "data-ui-stream-fallback",
    "data-ui-output-status"
  ]

  agent_contract_schema "ui.link.agent-contract/v1" {
    fields: [
      "intent",
      "action",
      "state",
      "source.disabled",
      "stream.support",
      "stream.fallback",
      "output.status"
    ]
  }
}
