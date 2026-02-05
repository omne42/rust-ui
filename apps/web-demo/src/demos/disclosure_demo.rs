use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{Accordion, AccordionSelectionMode, Disclosure};

#[component]
pub fn DisclosureDemo() -> impl IntoView {
    let (disclosure_open, set_disclosure_open) = signal(false);
    let on_disclosure_open_change = Callback::new(move |next: bool| set_disclosure_open.set(next));

    let (accordion_open, set_accordion_open) = signal({
        let mut open = BTreeSet::new();
        open.insert(0);
        open
    });

    let accordion_labels = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Notes".to_string(),
    ];

    view! {
        <section id="disclosure" class="demo-card">
            <h2>"Disclosure / Accordion"</h2>
            <p>"Disclosure is a single expandable region; Accordion composes multiple disclosures with keyboard roving."</p>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Disclosure"</div>
                    <Disclosure
                        id_base="demo-disclosure".to_string()
                        label="More information".to_string()
                        open=disclosure_open.into()
                        on_open_change=on_disclosure_open_change
                    >
                        <div class="demo-stack">
                            <div>"This panel uses `role=region` + `aria-labelledby`."</div>
                            <div>"Toggle it with click, Enter, or Space."</div>
                        </div>
                    </Disclosure>
                    <div class="demo-kv">{move || format!("open: {}", disclosure_open.get())}</div>
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"Accordion"</div>
                    <Accordion
                        labels=accordion_labels
                        id_base="demo-accordion".to_string()
                        open_indices=accordion_open
                        set_open_indices=set_accordion_open
                        selection_mode=AccordionSelectionMode::Multiple
                    >
                        <div class="demo-stack">
                            <div>"Accordion panel 1."</div>
                            <div>"Multi-select mode keeps multiple panels open."</div>
                        </div>
                        <div class="demo-stack">
                            <div>"Accordion panel 2."</div>
                            <div>"Arrow keys move focus between triggers."</div>
                        </div>
                        <div class="demo-stack">
                            <div>"Accordion panel 3."</div>
                            <div>"Uses tokens-driven styling."</div>
                        </div>
                    </Accordion>
                </div>
            </div>
        </section>
    }
}
