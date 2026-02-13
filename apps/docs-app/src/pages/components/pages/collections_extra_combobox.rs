use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{ComboBoxMotion, Combobox};

pub(super) fn combobox() -> AnyView {
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
    ];

    let items_basic = items.clone();
    let items_state = items.clone();
    let items_markers = items.clone();

    let (selected_basic, set_selected_basic) = signal(Some(1_usize));
    let (selected_state, set_selected_state) = signal(Some(2_usize));
    let (selected_markers, set_selected_markers) = signal(Some(0_usize));

    let (invalid, set_invalid) = signal(false);
    let (marker_invalid, set_marker_invalid) = signal(false);

    let basic_code = Signal::derive(move || {
        let selected = selected_basic.get();
        let selected_literal = selected
            .map(|value| format!("Some({value}_usize)"))
            .unwrap_or_else(|| "None".to_string());

        vec![
            format!("let (selected, set_selected) = signal({selected_literal});"),
            String::new(),
            "<Combobox".to_string(),
            "  id_base=\"lang\".to_string()".to_string(),
            "  label=\"Language\".to_string()".to_string(),
            "  items=items".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  description=\"Pick one runtime language\".to_string()".to_string(),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let state_code = Signal::derive(move || {
        let selected = selected_state.get();
        let selected_literal = selected
            .map(|value| format!("Some({value}_usize)"))
            .unwrap_or_else(|| "None".to_string());

        let mut snippet = vec![
            format!("let (selected, set_selected) = signal({selected_literal});"),
            String::new(),
            "<Combobox".to_string(),
            "  id_base=\"lang-state\".to_string()".to_string(),
            "  label=\"Stateful language\".to_string()".to_string(),
            "  items=items".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  disabled_indices=vec![3]".to_string(),
        ];

        if invalid.get() {
            snippet.push("  invalid=Signal::derive(|| true)".to_string());
            snippet.push("  error=\"Language is required\".to_string()".to_string());
        }

        snippet.push("/>".to_string());
        snippet.join("\n")
    });

    let markers_code = Signal::derive(move || {
        let selected = selected_markers.get();
        let selected_literal = selected
            .map(|value| format!("Some({value}_usize)"))
            .unwrap_or_else(|| "None".to_string());

        let mut snippet = vec![
            "let mut marker_motion = ComboBoxMotion::default();".to_string(),
            "marker_motion.popover.offset_y_px = 10.0;".to_string(),
            "marker_motion.highlight.spring.stiffness = 260.0;".to_string(),
            format!("let (selected, set_selected) = signal({selected_literal});"),
            String::new(),
            "<Combobox".to_string(),
            "  id_base=\"lang-markers\".to_string()".to_string(),
            "  label=\"Technology stack\".to_string()".to_string(),
            "  items=items".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  required=Signal::derive(|| true)".to_string(),
            "  disabled_indices=vec![3]".to_string(),
            "  description=\"Inspect source/state marker contracts\".to_string()".to_string(),
            "  placeholder=\"Type to filter\".to_string()".to_string(),
            "  class_name=\"docs-combobox-state\".to_string()".to_string(),
            "  motion=marker_motion".to_string(),
        ];

        if marker_invalid.get() {
            snippet.push("  invalid=Signal::derive(|| true)".to_string());
            snippet.push("  error=\"Selection is required\".to_string()".to_string());
        }

        snippet.push("/>".to_string());
        snippet.join("\n")
    });

    let marker_motion = {
        let mut motion = ComboBoxMotion::default();
        motion.popover.offset_y_px = 10.0;
        motion.highlight.spring.stiffness = 260.0;
        motion
    };

    view! {
        <ComponentPage
            title="Combobox"
            slug="combobox"
            group="Collections"
            description="Spectrum-compatible combobox alias for upstream naming parity, preserving ComboBox accessibility, state contracts, and HeroUI-level panel/highlight motion."
        >
            <Playground title="Basic Selection" code_signal=basic_code>
                <div class="docs-stack">
                    <Combobox
                        id_base="docs-combobox-basic".to_string()
                        label="Language".to_string()
                        items=items_basic
                        selected_index=selected_basic
                        set_selected_index=set_selected_basic
                        description="Pick one runtime language".to_string()
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected_basic.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Invalid + Disabled Option" code_signal=state_code>
                <div class="docs-stack">
                    <Combobox
                        id_base="docs-combobox-state".to_string()
                        label="Stateful language".to_string()
                        items=items_state
                        selected_index=selected_state
                        set_selected_index=set_selected_state
                        disabled_indices=vec![3]
                        invalid=Signal::derive(move || invalid.get())
                        error="Language is required".to_string()
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                    >
                        {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                    </ui_components::Button>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-selection`, `data-options`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-motion-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Combobox
                        id_base="docs-combobox-markers".to_string()
                        label="Technology stack".to_string()
                        items=items_markers
                        selected_index=selected_markers
                        set_selected_index=set_selected_markers
                        required=Signal::derive(|| true)
                        invalid=Signal::derive(move || marker_invalid.get())
                        disabled_indices=vec![3]
                        description="Inspect source/state marker contracts".to_string()
                        error="Selection is required".to_string()
                        placeholder="Type to filter".to_string()
                        class_name="docs-combobox-state".to_string()
                        motion=marker_motion
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| {
                            set_marker_invalid.update(|value| *value = !*value)
                        })
                    >
                        {move || {
                            if marker_invalid.get() {
                                "Clear marker invalid"
                            } else {
                                "Mark marker invalid"
                            }
                        }}
                    </ui_components::Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
