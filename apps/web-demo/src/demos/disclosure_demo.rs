use leptos::prelude::*;
use ui::{Accordion, AccordionItem, AccordionSelectionMode, Disclosure, open_set};

#[component]
pub fn DisclosureDemo() -> impl IntoView {
    let (disclosure_open, set_disclosure_open) = signal(false);
    let on_disclosure_open_change = Callback::new(move |next: bool| set_disclosure_open.set(next));

    let (accordion_open, set_accordion_open) = signal(open_set([0]));
    let item_0_open = Signal::derive(move || accordion_open.get().contains(&0));
    let item_1_open = Signal::derive(move || accordion_open.get().contains(&1));
    let item_2_open = Signal::derive(move || accordion_open.get().contains(&2));
    let on_item_0_open_change = Callback::new(move |is_open: bool| {
        set_accordion_open.update(|open| {
            if is_open {
                open.insert(0);
            } else {
                open.remove(&0);
            }
        });
    });
    let on_item_1_open_change = Callback::new(move |is_open: bool| {
        set_accordion_open.update(|open| {
            if is_open {
                open.insert(1);
            } else {
                open.remove(&1);
            }
        });
    });
    let on_item_2_open_change = Callback::new(move |is_open: bool| {
        set_accordion_open.update(|open| {
            if is_open {
                open.insert(2);
            } else {
                open.remove(&2);
            }
        });
    });

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
                        id_base="demo-accordion".to_string()
                        selection_mode=AccordionSelectionMode::Multiple
                    >
                        <AccordionItem label="Overview" key=0 open=item_0_open on_open_change=on_item_0_open_change>
                            <div class="demo-stack">
                                <div>"Accordion panel 1."</div>
                                <div>"Multi-select mode keeps multiple panels open."</div>
                            </div>
                        </AccordionItem>
                        <AccordionItem label="Details" key=1 open=item_1_open on_open_change=on_item_1_open_change>
                            <div class="demo-stack">
                                <div>"Accordion panel 2."</div>
                                <div>"Arrow keys move focus between triggers."</div>
                            </div>
                        </AccordionItem>
                        <AccordionItem label="Notes" key=2 open=item_2_open on_open_change=on_item_2_open_change>
                            <div class="demo-stack">
                                <div>"Accordion panel 3."</div>
                                <div>"Uses tokens-driven styling."</div>
                            </div>
                        </AccordionItem>
                    </Accordion>
                    <div class="demo-kv">
                        {move || {
                            let open = accordion_open.get();
                            let open = open.iter().copied().collect::<Vec<_>>();
                            format!("open: {open:?}")
                        }}
                    </div>
                </div>
            </div>
        </section>
    }
}
