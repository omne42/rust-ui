use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
use ui_components::{SegmentedControl, SegmentedControlSize, Switch};

pub(super) fn breadcrumb_primitives() -> AnyView {
    let scenario_options = vec!["basic".to_string(), "overflow".to_string()];
    let (scenario_index, set_scenario_index) = signal(Some(0_usize));
    let (custom_aria_label, set_custom_aria_label) = signal(false);

    let basic_code = Signal::derive(move || {
        let mut snippet = vec!["<Breadcrumb".to_string()];

        if custom_aria_label.get() {
            snippet.push("  aria_label=\"Documentation navigation\".to_string()".to_string());
        }

        snippet.extend([">".to_string(), "  <BreadcrumbList>".to_string()]);

        if scenario_index.get().unwrap_or(0) == 1 {
            snippet.extend([
                "    <BreadcrumbItem><BreadcrumbLink href=\"/\">\"Home\"</BreadcrumbLink></BreadcrumbItem>".to_string(),
                "    <BreadcrumbSeparator />".to_string(),
                "    <BreadcrumbEllipsis />".to_string(),
                "    <BreadcrumbSeparator />".to_string(),
                "    <BreadcrumbItem><BreadcrumbPage>\"Current\"</BreadcrumbPage></BreadcrumbItem>".to_string(),
            ]);
        } else {
            snippet.extend([
                "    <BreadcrumbItem>".to_string(),
                "      <BreadcrumbLink href=\"#/components\">\"Components\"</BreadcrumbLink>"
                    .to_string(),
                "    </BreadcrumbItem>".to_string(),
                "    <BreadcrumbSeparator />".to_string(),
                "    <BreadcrumbItem>".to_string(),
                "      <BreadcrumbPage>\"Breadcrumb\"</BreadcrumbPage>".to_string(),
                "    </BreadcrumbItem>".to_string(),
            ]);
        }

        snippet.extend([
            "  </BreadcrumbList>".to_string(),
            "</Breadcrumb>".to_string(),
        ]);

        snippet.join("\n")
    });

    let overflow_code = Signal::derive(move || {
        r#"<Breadcrumb>
  <BreadcrumbList>
    <BreadcrumbItem><BreadcrumbLink href="/">"Home"</BreadcrumbLink></BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbEllipsis />
    <BreadcrumbSeparator />
    <BreadcrumbItem><BreadcrumbPage>"Current"</BreadcrumbPage></BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>"#
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r##"<Breadcrumb
  aria_label="Documentation navigation".to_string()
  class_name="docs-breadcrumb-state".to_string()
>
  <BreadcrumbList class_name="docs-breadcrumb-list".to_string()>
    <BreadcrumbItem>
      <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator>
      <span>"→"</span>
    </BreadcrumbSeparator>
    <BreadcrumbItem>
      <BreadcrumbLink class_name="docs-breadcrumb-link".to_string()>
        "Collections"
      </BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem>
      <BreadcrumbPage class_name="docs-breadcrumb-page".to_string()>
        "Breadcrumb"
      </BreadcrumbPage>
    </BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="BreadcrumbList"
            slug="breadcrumb-list"
            group="Collections"
            description="Shadcn-compatible breadcrumb primitive family (`Breadcrumb*`) with slot-stable link/page/separator/ellipsis composition contracts and Spectrum-style source/state marker semantics."
        >
            <Playground
                title="Link + Current Page"
                code_signal=basic_code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-breadcrumb-primitive-scenario".to_string()
                            options=scenario_options.clone()
                            selected_index=scenario_index
                            set_selected_index=set_scenario_index
                            size=SegmentedControlSize::Sm
                            aria_label="Breadcrumb primitive scenario".to_string()
                        />

                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    if scenario_index.get().unwrap_or(0) == 1 {
                        if custom_aria_label.get() {
                            view! {
                                <Breadcrumb aria_label="Documentation navigation".to_string()>
                                    <BreadcrumbList>
                                        <BreadcrumbItem>
                                            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                                        </BreadcrumbItem>
                                        <BreadcrumbSeparator />
                                        <BreadcrumbEllipsis />
                                        <BreadcrumbSeparator />
                                        <BreadcrumbItem>
                                            <BreadcrumbPage>
                                                "Current"
                                            </BreadcrumbPage>
                                        </BreadcrumbItem>
                                    </BreadcrumbList>
                                </Breadcrumb>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Breadcrumb>
                                    <BreadcrumbList>
                                        <BreadcrumbItem>
                                            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                                        </BreadcrumbItem>
                                        <BreadcrumbSeparator />
                                        <BreadcrumbEllipsis />
                                        <BreadcrumbSeparator />
                                        <BreadcrumbItem>
                                            <BreadcrumbPage>
                                                "Current"
                                            </BreadcrumbPage>
                                        </BreadcrumbItem>
                                    </BreadcrumbList>
                                </Breadcrumb>
                            }
                                .into_any()
                        }
                    } else if custom_aria_label.get() {
                        view! {
                            <Breadcrumb aria_label="Documentation navigation".to_string()>
                                <BreadcrumbList>
                                    <BreadcrumbItem>
                                        <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
                                    </BreadcrumbItem>
                                    <BreadcrumbSeparator />
                                    <BreadcrumbItem>
                                        <BreadcrumbPage>
                                            "Breadcrumb"
                                        </BreadcrumbPage>
                                    </BreadcrumbItem>
                                </BreadcrumbList>
                            </Breadcrumb>
                        }
                            .into_any()
                    } else {
                        view! {
                            <Breadcrumb>
                                <BreadcrumbList>
                                    <BreadcrumbItem>
                                        <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
                                    </BreadcrumbItem>
                                    <BreadcrumbSeparator />
                                    <BreadcrumbItem>
                                        <BreadcrumbPage>
                                            "Breadcrumb"
                                        </BreadcrumbPage>
                                    </BreadcrumbItem>
                                </BreadcrumbList>
                            </Breadcrumb>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="Ellipsis Overflow" code_signal=overflow_code>
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbEllipsis />
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>
                                "Current"
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root/link/separator markers like `data-state`, `data-aria-source`, `data-class-source`, `data-href-state`, and `data-content-source` for Spectrum-compatible breadcrumb contracts."
                code_signal=markers_code
            >
                <Breadcrumb
                    aria_label="Documentation navigation".to_string()
                    class_name="docs-breadcrumb-state".to_string()
                >
                    <BreadcrumbList class_name="docs-breadcrumb-list".to_string()>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator>
                            <span>"→"</span>
                        </BreadcrumbSeparator>
                        <BreadcrumbItem>
                            <BreadcrumbLink class_name="docs-breadcrumb-link".to_string()>
                                "Collections"
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage class_name="docs-breadcrumb-page".to_string()>
                                "Breadcrumb"
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
