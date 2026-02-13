use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Search, SearchFieldMotion};

pub(super) fn search() -> AnyView {
    let (value, set_value) = signal(String::new());
    let (submitted, set_submitted) = signal(String::new());
    let (clear_count, set_clear_count) = signal(0_usize);

    let on_submit = Callback::new(move |next: String| set_submitted.set(next));
    let on_clear = Callback::new(move |_| set_clear_count.update(|count| *count += 1));

    let (required_value, set_required_value) = signal(String::new());
    let required_invalid = Signal::derive(move || required_value.get().trim().is_empty());

    let (marker_value, set_marker_value) = signal("popover".to_string());
    let (marker_submitted, set_marker_submitted) = signal(String::new());
    let (marker_clear_count, set_marker_clear_count) = signal(0_u32);
    let (marker_invalid, set_marker_invalid) = signal(false);

    let marker_on_submit = Callback::new(move |next: String| set_marker_submitted.set(next));
    let marker_on_clear =
        Callback::new(move |_| set_marker_clear_count.update(|count| *count += 1));

    let basic_code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
let on_submit = Callback::new(|query: String| { /* ... */ });
let on_clear = Callback::new(|()| { /* ... */ });
<Search
  id=\"site-search\".to_string()
  label=\"Search docs\".to_string()
  value=value
  set_value=set_value
  on_submit=on_submit
  on_clear=on_clear
  placeholder=\"Try: overlay\".to_string()
/>"#
        .to_string()
    });

    let validation_code = Signal::derive(move || {
        r#"let required_invalid = Signal::derive(move || value.get().trim().is_empty());
<Search
  id=\"required-search\".to_string()
  label=\"Required query\".to_string()
  value=value
  set_value=set_value
  required=true
  invalid=required_invalid
  error=\"Query is required\".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
let (invalid, set_invalid) = signal(false);
let on_submit = Callback::new(move |_| {
  set_invalid.set(value.get().trim().is_empty());
});
let on_clear = Callback::new(move |_| {
  set_value.set(String::new());
  set_invalid.set(false);
});
let mut marker_motion = SearchFieldMotion::default();
marker_motion.hidden_scale = 0.78;
marker_motion.hover_scale = 1.08;
marker_motion.tap_scale = 0.92;

<Search
  id="docs-search-markers".to_string()
  label="Search runtime docs".to_string()
  value=value
  set_value=set_value
  required=true
  invalid=Signal::derive(move || invalid.get())
  description="Inspect source/state marker contracts".to_string()
  error="Query is required".to_string()
  placeholder="Try: spring".to_string()
  class_name="docs-search-state".to_string()
  on_submit=on_submit
  on_clear=on_clear
  motion=marker_motion
/>"#
        .to_string()
    });

    let marker_motion = SearchFieldMotion {
        hidden_scale: 0.78,
        hover_scale: 1.08,
        tap_scale: 0.92,
        ..SearchFieldMotion::default()
    };

    view! {
        <ComponentPage
            title="Search"
            slug="search"
            group="Forms"
            description="Spectrum-compatible Search alias for upstream naming parity, preserving SearchField accessibility/state contracts and HeroUI-level clear-button spring motion."
        >
            <Playground title="Submit + Clear" code_signal=basic_code>
                <div class="docs-stack">
                    <Search
                        id="docs-search-basic".to_string()
                        label="Search docs".to_string()
                        value=value
                        set_value=set_value
                        on_submit=on_submit
                        on_clear=on_clear
                        placeholder="Try: overlay".to_string()
                        description="Press Enter to submit; Escape to clear.".to_string()
                    />
                    <span class="ui-muted">"submitted: " {move || submitted.get()}</span>
                    <span class="ui-muted">"clear count: " {move || clear_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Required + Invalid" code_signal=validation_code>
                <div class="docs-stack">
                    <Search
                        id="docs-search-required".to_string()
                        label="Required query".to_string()
                        value=required_value
                        set_value=set_required_value
                        required=true
                        invalid=required_invalid
                        error="Query is required".to_string()
                        placeholder="Type a query".to_string()
                    />
                    <span class="ui-muted">
                        "invalid: "
                        {move || required_invalid.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, `data-submit-handler-source`, `data-clear-handler-source`, and `data-motion-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Search
                        id="docs-search-markers".to_string()
                        label="Search runtime docs".to_string()
                        value=marker_value
                        set_value=set_marker_value
                        required=true
                        invalid=Signal::derive(move || marker_invalid.get())
                        description="Inspect source/state marker contracts".to_string()
                        error="Query is required".to_string()
                        placeholder="Try: spring".to_string()
                        class_name="docs-search-state".to_string()
                        on_submit=marker_on_submit
                        on_clear=marker_on_clear
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
                    <span class="ui-muted">"marker submitted: " {move || marker_submitted.get()}</span>
                    <span class="ui-muted">
                        "marker clear count: "
                        {move || marker_clear_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
