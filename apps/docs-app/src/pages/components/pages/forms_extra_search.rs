use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::Search;

pub(super) fn search() -> AnyView {
    let (value, set_value) = signal(String::new());
    let (submitted, set_submitted) = signal(String::new());
    let (clear_count, set_clear_count) = signal(0_usize);

    let on_submit = Callback::new(move |next: String| set_submitted.set(next));
    let on_clear = Callback::new(move |_| set_clear_count.update(|count| *count += 1));

    let (required_value, set_required_value) = signal(String::new());
    let required_invalid = Signal::derive(move || required_value.get().trim().is_empty());

    let basic_code = r#"let (value, set_value) = signal(String::new());
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
/>"#;

    let validation_code = r#"let required_invalid = Signal::derive(move || value.get().trim().is_empty());
<Search
  id=\"required-search\".to_string()
  label=\"Required query\".to_string()
  value=value
  set_value=set_value
  required=true
  invalid=required_invalid
  error=\"Query is required\".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Search"
            slug="search"
            group="Forms"
            description="Spectrum-compatible Search alias for upstream naming parity, preserving SearchField accessibility/state contracts and HeroUI-level clear-button spring motion."
        >
            <Playground title="Submit + Clear" code=basic_code>
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

            <Playground title="Required + Invalid" code=validation_code>
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
        </ComponentPage>
    }
    .into_any()
}
