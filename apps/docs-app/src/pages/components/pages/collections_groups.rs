use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::tag::{Tag, TagSize, TagVariant};
use ui_components::{
    Collapsible, CollapsibleMotion, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn tag() -> AnyView {
    let (remove_count, set_remove_count) = signal(0_u32);

    let on_remove_alpha = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let on_remove_beta = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let hello_world_code = Signal::derive(|| {
        "<Tag>\"Hello Tag\"</Tag>\n<Tag variant=TagVariant::Surface>\"Surface\"</Tag>".to_string()
    });

    let matrix_code = Signal::derive(move || {
        [
            "<Tag variant=TagVariant::Default size=TagSize::Sm>\"Rust\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Default>\"Leptos\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Surface>\"Naming parity\"</Tag>".to_string(),
            "<Tag variant=TagVariant::Surface size=TagSize::Lg>\"baseline contracts\"</Tag>"
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
            description="baseline-style tag primitive with centralized variant/size/remove-action/source state contracts and stable slot/data markers."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                test_source_path="crates/ui-components/src/tag/view.rs".to_string()
            >
                <div class="docs-row">
                    <Tag>"Hello Tag"</Tag>
                    <Tag variant=TagVariant::Surface>"Surface"</Tag>
                </div>
            </Playground>

            <Playground
                title="Variant + Size Matrix"
                code_signal=matrix_code
                test_source_path="crates/ui-components/src/tag/view.rs".to_string()
            >
                <div class="docs-row">
                    <Tag variant=TagVariant::Default size=TagSize::Sm>
                        "Rust"
                    </Tag>
                    <Tag variant=TagVariant::Default size=TagSize::Md>
                        "Leptos"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Md>
                        "Naming parity"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Lg>
                        "baseline contracts"
                    </Tag>
                </div>
            </Playground>

            <Playground
                title="Removable + Disabled + Custom Class"
                code_signal=states_code
                test_source_path="crates/ui-components/src/tag/view.rs".to_string()
            >
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
        [
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

    let mode_options = vec!["Uncontrolled".to_string(), "Controlled".to_string()];
    let motion_options = vec!["Default".to_string(), "Custom".to_string()];
    let (mode_index, set_mode_index) = signal(Some(0_usize));
    let (motion_index, set_motion_index) = signal(Some(0_usize));
    let (controlled_open, set_controlled_open) = signal(true);
    let (default_open_preview, set_default_open_preview) = signal(true);
    let (disabled_preview, set_disabled_preview) = signal(false);
    let (custom_label, set_custom_label) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let on_interactive_open_change = Callback::new(move |next: bool| set_controlled_open.set(next));

    let is_controlled = Signal::derive(move || mode_index.get().unwrap_or(0) == 1);
    let use_custom_motion = Signal::derive(move || motion_index.get().unwrap_or(0) == 1);
    let interactive_motion = Signal::derive(move || {
        if use_custom_motion.get() {
            CollapsibleMotion {
                panel_offset_y_px: 10.0,
                ..CollapsibleMotion::default()
            }
        } else {
            CollapsibleMotion::default()
        }
    });

    let interactive_code = Signal::derive(move || {
        let controlled = is_controlled.get();
        let motion_custom = use_custom_motion.get();
        let controlled_open = controlled_open.get();
        let default_open_preview = default_open_preview.get();
        let disabled_preview = disabled_preview.get();
        let custom_label = custom_label.get();
        let custom_class = custom_class.get();

        let mut lines = vec![
            "<Collapsible".to_string(),
            "  id_base=\"docs-collapsible-interactive\".to_string()".to_string(),
            "  title=\"Interactive collapsible\".to_string()".to_string(),
        ];

        if controlled {
            lines.push(format!(
                "  open=Signal::derive(|| {controlled_open}).into()"
            ));
            lines.push(
                "  on_open_change=Callback::new(move |next: bool| set_open.set(next))".to_string(),
            );
        } else {
            lines.push(format!("  default_open={default_open_preview}"));
        }

        if disabled_preview {
            lines.push("  disabled=true".to_string());
        }
        if custom_label {
            lines.push("  aria_label=\"Interactive collapsible panel\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-collapsible-custom\".to_string()".to_string());
        }
        if motion_custom {
            lines.push("  motion=CollapsibleMotion { panel_offset_y_px: 10.0, ..CollapsibleMotion::default() }".to_string());
        }

        lines.extend([
            ">".to_string(),
            "  <div>\"Interactive panel content.\"</div>".to_string(),
            "</Collapsible>".to_string(),
        ]);
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/collapsible/styles.rs */\n{}",
            ui_components::collapsible::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let controlled = is_controlled.get();
        let motion_custom = use_custom_motion.get();
        let controlled_open = controlled_open.get();
        let default_open_preview = default_open_preview.get();
        let disabled_preview = disabled_preview.get();
        let custom_label = custom_label.get();
        let custom_class = custom_class.get();

        format!(
            "CollapsibleActualConfig {{\n  mode: \"{}\",\n  controlled_open: {},\n  default_open: {},\n  disabled: {},\n  custom_label: {},\n  custom_class: {},\n  motion_source: \"{}\",\n  panel_offset_y_px: {},\n}}",
            if controlled {
                "controlled"
            } else {
                "uncontrolled"
            },
            controlled_open,
            default_open_preview,
            disabled_preview,
            custom_label,
            custom_class,
            if motion_custom { "custom" } else { "default" },
            if motion_custom {
                10.0
            } else {
                CollapsibleMotion::default().panel_offset_y_px
            },
        )
    });

    view! {
        <ComponentPage
            title="Collapsible"
            slug="collapsible"
            group="Collections"
            description="baseline-compatible collapsible primitive built on Disclosure semantics with baseline-level spring panel motion and stable state/source contracts."
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

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="crates/ui-components/src/collapsible/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Mode"</div>
                        <SegmentedControl
                            id_base="docs-collapsible-interactive-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="Collapsible open mode".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-collapsible-interactive-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="Collapsible motion source".to_string()
                        />

                        <Switch checked=controlled_open set_checked=set_controlled_open>
                            "Controlled open"
                        </Switch>
                        <Switch checked=default_open_preview set_checked=set_default_open_preview>
                            "Default open"
                        </Switch>
                        <Switch checked=disabled_preview set_checked=set_disabled_preview>
                            "Disabled"
                        </Switch>
                        <Switch checked=custom_label set_checked=set_custom_label>
                            "Custom aria-label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let controlled = is_controlled.get();
                    let motion = interactive_motion.get();
                    let disabled_preview = disabled_preview.get();
                    let default_open_preview = default_open_preview.get();
                    let custom_label = custom_label.get();
                    let custom_class = custom_class.get();
                    let aria_label = if custom_label {
                        "Interactive collapsible panel".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-collapsible-custom".to_string()
                    } else {
                        String::new()
                    };

                    if controlled {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <Collapsible
                                    id_base="docs-collapsible-interactive".to_string()
                                    title="Interactive collapsible".to_string()
                                    open=controlled_open.into()
                                    on_open_change=on_interactive_open_change
                                    disabled=disabled_preview
                                    motion=motion
                                    aria_label=aria_label.clone()
                                    class_name=class_name.clone()
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Interactive panel content."</div>
                                        <div class="ui-muted">
                                            "Use Mode switch to compare controlled vs uncontrolled state source."
                                        </div>
                                    </div>
                                </Collapsible>
                                <span class="ui-muted">
                                    "controlled open: " {move || controlled_open.get().to_string()}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <Collapsible
                                    id_base="docs-collapsible-interactive".to_string()
                                    title="Interactive collapsible".to_string()
                                    default_open=default_open_preview
                                    disabled=disabled_preview
                                    motion=motion
                                    aria_label=aria_label
                                    class_name=class_name
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Interactive panel content."</div>
                                        <div class="ui-muted">
                                            "Use Mode switch to compare controlled vs uncontrolled state source."
                                        </div>
                                    </div>
                                </Collapsible>
                                <span class="ui-muted">
                                    "default open: " {default_open_preview.to_string()}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
