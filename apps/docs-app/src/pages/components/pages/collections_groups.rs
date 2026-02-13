use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::tag::{Tag, TagSize, TagVariant};
use ui_components::{Collapsible, CollapsibleMotion};

pub(super) fn tag() -> AnyView {
    let (remove_count, set_remove_count) = signal(0_u32);

    let on_remove_alpha = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let on_remove_beta = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let matrix_code = Signal::derive(move || {
        [
            "<Tag variant=TagVariant::Default size=TagSize::Sm>\"Rust\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Default>\"Leptos\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Surface>\"HeroUI parity\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Surface size=TagSize::Lg>\"Spectrum contracts\"</Tag>"
                .to_string(),
        ]
        .join("\n")
    });

    let states_code = Signal::derive(move || {
        vec![
            format!("let (remove_count, set_remove_count) = signal({}_u32);", remove_count.get()),
            "let on_remove_alpha = Callback::new(move |_| set_remove_count.update(|count| *count += 1));"
                .to_string(),
            "let on_remove_beta = Callback::new(move |_| set_remove_count.update(|count| *count += 1));"
                .to_string(),
            String::new(),
            "<Tag".to_string(),
            "  variant=TagVariant::Surface".to_string(),
            "  removable=true".to_string(),
            "  on_remove=on_remove_alpha".to_string(),
            "  remove_aria_label=\"Remove alpha release\".to_string()".to_string(),
            ">".to_string(),
            "  \"alpha\"".to_string(),
            "</Tag>".to_string(),
            "<Tag removable=true on_remove=on_remove_beta class_name=\"docs-tag-custom\".to_string()>"
                .to_string(),
            "  \"beta\"".to_string(),
            "</Tag>".to_string(),
            "<Tag disabled=true removable=true>\"disabled\"</Tag>".to_string(),
        ]
        .join("\n")
    });

    view! {
        <ComponentPage
            title="Tag"
            slug="tag"
            group="Collections"
            description="Spectrum/HeroUI-style tag primitive with centralized variant/size/remove-action/source state contracts and stable slot/data markers."
        >
            <Playground title="Variant + Size Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Tag variant=TagVariant::Default size=TagSize::Sm>
                        "Rust"
                    </Tag>
                    <Tag variant=TagVariant::Default size=TagSize::Md>
                        "Leptos"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Md>
                        "HeroUI parity"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Lg>
                        "Spectrum contracts"
                    </Tag>
                </div>
            </Playground>

            <Playground title="Removable + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Tag
                            variant=TagVariant::Surface
                            size=TagSize::Md
                            removable=true
                            on_remove=on_remove_alpha
                            remove_aria_label="Remove alpha release".to_string()
                        >
                            "alpha"
                        </Tag>
                        <Tag
                            variant=TagVariant::Default
                            size=TagSize::Md
                            removable=true
                            on_remove=on_remove_beta
                            class_name="docs-tag-custom".to_string()
                        >
                            "beta"
                        </Tag>
                        <Tag variant=TagVariant::Default size=TagSize::Md disabled=true removable=true>
                            "disabled"
                        </Tag>
                    </div>
                    <span class="ui-muted">
                        "remove count: " {move || remove_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn collapsible() -> AnyView {
    let (open, set_open) = signal(true);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));

    let basic_code = Signal::derive(move || {
        vec![
            format!("let (open, set_open) = signal({});", open.get()),
            "let on_open_change = Callback::new(move |next: bool| set_open.set(next));".to_string(),
            String::new(),
            "<Collapsible".to_string(),
            "  id_base=\"docs-collapsible\".to_string()".to_string(),
            "  title=\"Advanced options\".to_string()".to_string(),
            "  open=open.into()".to_string(),
            "  on_open_change=on_open_change".to_string(),
            ">".to_string(),
            "  <div>\"Panel content with disclosure-level semantics.\"</div>".to_string(),
            "</Collapsible>".to_string(),
        ]
        .join("\n")
    });

    let states_code = Signal::derive(move || {
        vec![
            "<Collapsible".to_string(),
            "  id_base=\"docs-collapsible-disabled\".to_string()".to_string(),
            "  title=\"Disabled section\".to_string()".to_string(),
            "  default_open=false".to_string(),
            "  disabled=true".to_string(),
            "  class_name=\"docs-collapsible-custom\".to_string()".to_string(),
            "  motion=CollapsibleMotion {".to_string(),
            "    panel_offset_y_px: 6.0,".to_string(),
            "    ..CollapsibleMotion::default()".to_string(),
            "  }".to_string(),
            ">".to_string(),
            "  <div>\"This content is intentionally not reachable while disabled.\"</div>"
                .to_string(),
            "</Collapsible>".to_string(),
        ]
        .join("\n")
    });

    let markers_code = Signal::derive(move || {
        vec![
            "<Collapsible".to_string(),
            "  id_base=\"docs-collapsible-markers\".to_string()".to_string(),
            "  title=\"Advanced settings\".to_string()".to_string(),
            "  aria_label=\"Advanced settings panel\".to_string()".to_string(),
            "  default_open=true".to_string(),
            "  class_name=\"docs-collapsible-state\".to_string()".to_string(),
            "  motion=CollapsibleMotion {".to_string(),
            "    panel_offset_y_px: 8.0,".to_string(),
            "    ..CollapsibleMotion::default()".to_string(),
            "  }".to_string(),
            ">".to_string(),
            "  <div>\"Inspect root/trigger/panel marker contracts.\"</div>".to_string(),
            "</Collapsible>".to_string(),
        ]
        .join("\n")
    });

    let custom_motion = CollapsibleMotion {
        panel_offset_y_px: 6.0,
        ..CollapsibleMotion::default()
    };

    let marker_motion = CollapsibleMotion {
        panel_offset_y_px: 8.0,
        ..CollapsibleMotion::default()
    };

    view! {
        <ComponentPage
            title="Collapsible"
            slug="collapsible"
            group="Collections"
            description="Shadcn-compatible collapsible primitive built on Disclosure semantics with HeroUI-level spring panel motion and stable state/source contracts."
        >
            <Playground title="Controlled Collapsible" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <Collapsible
                        id_base="docs-collapsible".to_string()
                        title="Advanced options".to_string()
                        open=open.into()
                        on_open_change=on_open_change
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Panel content with disclosure-level semantics."</div>
                            <div class="ui-muted">"Escape/keyboard behavior follows the trigger press contract."</div>
                        </div>
                    </Collapsible>
                    <span class="ui-muted">"open: " {move || open.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Motion" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Collapsible
                        id_base="docs-collapsible-disabled".to_string()
                        title="Disabled section".to_string()
                        default_open=false
                        disabled=true
                        class_name="docs-collapsible-custom".to_string()
                        motion=custom_motion
                    >
                        <div>"This content is intentionally not reachable while disabled."</div>
                    </Collapsible>
                    <span class="ui-muted">"disabled: true"</span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-open-mode`, `data-label-source`, `data-class-source`, `data-motion-source`, and `data-custom-motion` across collapsible root/trigger/panel contracts."
                code_signal=markers_code
            >
                <Collapsible
                    id_base="docs-collapsible-markers".to_string()
                    title="Advanced settings".to_string()
                    aria_label="Advanced settings panel".to_string()
                    default_open=true
                    class_name="docs-collapsible-state".to_string()
                    motion=marker_motion
                >
                    <div class="docs-stack docs-stack--tight">
                        <div>"Inspect root/trigger/panel marker contracts."</div>
                        <div class="ui-muted">"Open mode, label source, class source, and motion source are explicit."</div>
                    </div>
                </Collapsible>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
