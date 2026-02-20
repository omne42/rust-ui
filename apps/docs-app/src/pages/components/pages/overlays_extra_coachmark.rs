use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Button, ButtonVariant, Coachmark, CoachmarkAssetVariant, CoachmarkVariant, OnPress,
};

pub(super) fn coachmark() -> AnyView {
    let (last_action, set_last_action) = signal("none".to_string());

    let on_primary: OnPress = Callback::new(move |_| set_last_action.set("primary".to_string()));
    let on_secondary: OnPress =
        Callback::new(move |_| set_last_action.set("secondary".to_string()));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_controlled_open_change =
        Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let toggle_controlled: OnPress = Callback::new(move |_| {
        set_controlled_open_raw.update(|open| *open = !*open);
    });

    let basic_code = Signal::derive(move || {
        r#"<Coachmark
  title=\"Welcome to the tour\".into()
  default_open=true
  current_step=2
  total_steps=5
  primary_cta=\"Next\".into()
  secondary_cta=\"Back\".into()
  asset_variant=CoachmarkAssetVariant::Folder
>
  <div>Tour copy</div>
</Coachmark>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<Coachmark
  title=\"Keyboard shortcuts\".into()
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  primary_cta=\"Got it\".into()
  shortcut_key=\"K\".into()
  modifier_keys=vec![\"⌘\".into()]
  asset_src=\"https://picsum.photos/420/260\".into()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<Coachmark
  title=\"Shortcuts\".into()
  aria_label=\"Coachmark help\".into()
  current_step=2
  total_steps=6
  primary_cta=\"Next\".into()
  secondary_cta=\"Back\".into()
  shortcut_key=\"K\".into()
  modifier_keys=vec![\"⌘\".into()]
  asset_variant=CoachmarkAssetVariant::Folder
  class_name=\"docs-coachmark-state\".into()
>
  <div>Inspect data-state/source markers on root + content.</div>
</Coachmark>"#
            .to_string()
    });

    let display_code = Signal::derive(move || {
        r#"<Coachmark title=\"Help variant\".into() default_open=true current_step=1 total_steps=3 primary_cta=\"Next\".into() asset_variant=CoachmarkAssetVariant::Folder>
  <div>Default help intent with built-in asset.</div>
</Coachmark>
<Coachmark variant=CoachmarkVariant::Info title=\"Info variant\".into() default_open=true primary_cta=\"Understood\".into() asset_src=\"https://picsum.photos/420/260\".into()>
  <div>Info intent with external image source.</div>
</Coachmark>
<Coachmark title=\"Disabled preview\".into() default_open=true disabled=true secondary_cta=\"Dismiss\".into()>
  <div>Disabled state keeps semantic markers for testing.</div>
</Coachmark>"#.to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(true);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let toggle_workbench_open: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.update(|open| *open = !*open);
    });

    let (workbench_info_variant, set_workbench_info_variant) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_steps, set_workbench_show_steps) = signal(true);
    let (workbench_dual_cta, set_workbench_dual_cta) = signal(true);
    let (workbench_use_image, set_workbench_use_image) = signal(false);
    let (workbench_use_custom_class, set_workbench_use_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_info_variant.get() {
            "  variant=CoachmarkVariant::Info\n"
        } else {
            ""
        };
        let disabled = if workbench_disabled.get() {
            "  disabled=true\n"
        } else {
            ""
        };
        let steps = if workbench_show_steps.get() {
            "  current_step=2\n  total_steps=5\n"
        } else {
            ""
        };
        let cta = if workbench_dual_cta.get() {
            "  primary_cta=\"Next\".into()\n  secondary_cta=\"Back\".into()\n"
        } else {
            "  primary_cta=\"Got it\".into()\n"
        };
        let asset = if workbench_use_image.get() {
            "  asset_src=\"https://picsum.photos/420/260\".into()\n"
        } else {
            "  asset_variant=CoachmarkAssetVariant::Folder\n"
        };
        let class_name = if workbench_use_custom_class.get() {
            "  class_name=\"docs-coachmark-state\".into()\n"
        } else {
            ""
        };

        format!(
            "let (open, set_open) = signal({});\n\n<Coachmark\n  title=\"Workbench coachmark\".into()\n{variant}{disabled}  open=Signal::derive(move || open.get())\n  on_open_change=Callback::new(move |next| set_open.set(next))\n{steps}{cta}{asset}{class_name}>\n  <div>Inspect display/config/code/css-test panels together.</div>\n</Coachmark>",
            workbench_open_raw.get()
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/coachmark/src/styles.rs */\n{}",
            ui_components::coachmark::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = if workbench_info_variant.get() {
            CoachmarkVariant::Info
        } else {
            CoachmarkVariant::Help
        };
        let asset_source = if workbench_use_image.get() {
            "image"
        } else {
            "variant"
        };
        let cta_mode = if workbench_dual_cta.get() {
            "dual"
        } else {
            "single"
        };
        let steps = if workbench_show_steps.get() {
            "present"
        } else {
            "absent"
        };
        let class_source = if workbench_use_custom_class.get() {
            "custom"
        } else {
            "default"
        };

        format!(
            "CoachmarkWorkbenchConfig {{\n  variant: {variant:?},\n  open: {},\n  disabled: {},\n  steps: \"{steps}\",\n  cta: \"{cta_mode}\",\n  asset_source: \"{asset_source}\",\n  class_source: \"{class_source}\",\n}}",
            workbench_open_raw.get(),
            workbench_disabled.get(),
        )
    });

    view! {
        <ComponentPage
            title="Coachmark"
            slug="coachmark"
            group="Overlays"
            description="baseline-compatible Coachmark primitive for guided tours, composed on ContextualHelp/Popover contracts with baseline-level spring overlay motion and optional asset + CTA navigation semantics."
        >
            <Playground title="Step + CTA + Asset Variant" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <Coachmark
                        title="Welcome to the tour".to_string()
                        default_open=true
                        current_step=2
                        total_steps=5
                        primary_cta="Next".to_string()
                        secondary_cta="Back".to_string()
                        on_primary=on_primary
                        on_secondary=on_secondary
                        asset_variant=CoachmarkAssetVariant::Folder
                        asset_label="Tour folder".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Discover navigation and command surfaces in this guided step."</div>
                            <div class="ui-muted">"Uses contextual popover semantics with footer CTA controls."</div>
                        </div>
                    </Coachmark>
                    <span class="ui-muted">"last action: " {move || last_action.get()}</span>
                </div>
            </Playground>

            <Playground title="Controlled + Image Asset + Actions" code_signal=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=toggle_controlled>
                            "Toggle controlled coachmark"
                        </Button>
                        <span class="ui-muted">"open: " {move || controlled_open_raw.get()}</span>
                    </div>

                    <Coachmark
                        title="Keyboard shortcuts".to_string()
                        open=controlled_open
                        on_open_change=on_controlled_open_change
                        primary_cta="Got it".to_string()
                        secondary_cta="Skip".to_string()
                        shortcut_key="K".to_string()
                        modifier_keys=vec!["⌘".to_string()]
                        asset_src="https://picsum.photos/420/260".to_string()
                        asset_alt="Dashboard preview".to_string()
                        actions=move || {
                            view! {
                                <Button variant=ButtonVariant::Secondary>
                                    "Restart"
                                </Button>
                            }
                        }
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Press ⌘ + K to quickly open command search from anywhere."</div>
                            <div class="ui-muted">"Controlled mode keeps parent state as source of truth."</div>
                        </div>
                    </Coachmark>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-open-mode`, `data-label-source`, `data-class-source`, and content-level `data-asset-source` for baseline-compatible coachmark contracts."
                code_signal=markers_code
            >
                <Coachmark
                    title="Shortcuts".to_string()
                    aria_label="Coachmark help".to_string()
                    current_step=2
                    total_steps=6
                    primary_cta="Next".to_string()
                    secondary_cta="Back".to_string()
                    shortcut_key="K".to_string()
                    modifier_keys=vec!["⌘".to_string()]
                    asset_variant=CoachmarkAssetVariant::Folder
                    class_name="docs-coachmark-state".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <div>"Inspect data-state/source markers on root + content."</div>
                        <div class="ui-muted">"Aria label + class source + asset source contracts are explicit."</div>
                    </div>
                </Coachmark>
            </Playground>

            <Playground
                title="Display Comparisons (Help / Info / Disabled)"
                description="Display matrix for common states to compare variant intent, asset source, and disabled behavior side by side."
                code_signal=display_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Coachmark
                        title="Help variant".to_string()
                        default_open=true
                        current_step=1
                        total_steps=3
                        primary_cta="Next".to_string()
                        asset_variant=CoachmarkAssetVariant::Folder
                    >
                        <div>"Default help intent with built-in asset."</div>
                    </Coachmark>
                    <Coachmark
                        variant=CoachmarkVariant::Info
                        title="Info variant".to_string()
                        default_open=true
                        primary_cta="Understood".to_string()
                        asset_src="https://picsum.photos/420/260".to_string()
                        asset_alt="Info preview".to_string()
                    >
                        <div>"Info intent with external image source."</div>
                    </Coachmark>
                    <Coachmark
                        title="Disabled preview".to_string()
                        default_open=true
                        disabled=true
                        secondary_cta="Dismiss".to_string()
                    >
                        <div>"Disabled state keeps semantic markers for testing."</div>
                    </Coachmark>
                </div>
            </Playground>

            <Playground
                title="Config + Code + CSS Test Workbench"
                description="Config panel drives a single live instance; use Show code and Show test to inspect copy-ready code and scoped CSS."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/coachmark/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_info_variant.get()
                                    on:change=move |ev| set_workbench_info_variant.set(event_target_checked(&ev))
                                />
                                <span>"Info variant"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_disabled.get()
                                    on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                                />
                                <span>"Disabled"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_show_steps.get()
                                    on:change=move |ev| set_workbench_show_steps.set(event_target_checked(&ev))
                                />
                                <span>"Show step counter"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_dual_cta.get()
                                    on:change=move |ev| set_workbench_dual_cta.set(event_target_checked(&ev))
                                />
                                <span>"Dual CTA"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_use_image.get()
                                    on:change=move |ev| set_workbench_use_image.set(event_target_checked(&ev))
                                />
                                <span>"Use image asset"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_use_custom_class.get()
                                    on:change=move |ev| set_workbench_use_custom_class.set(event_target_checked(&ev))
                                />
                                <span>"Enable custom class source"</span>
                            </label>
                            <Button variant=ButtonVariant::Secondary on_press=toggle_workbench_open>
                                "Toggle open"
                            </Button>
                        </div>
                    }
                }
            >
                {move || {
                    let variant = if workbench_info_variant.get() {
                        CoachmarkVariant::Info
                    } else {
                        CoachmarkVariant::Help
                    };
                    let disabled = workbench_disabled.get();
                    let (current_step, total_steps) = if workbench_show_steps.get() {
                        (2, 5)
                    } else {
                        (0, 1)
                    };
                    let primary_cta = if workbench_dual_cta.get() {
                        "Next".to_string()
                    } else {
                        "Got it".to_string()
                    };
                    let secondary_cta = if workbench_dual_cta.get() {
                        "Back".to_string()
                    } else {
                        String::new()
                    };
                    let asset_src = if workbench_use_image.get() {
                        "https://picsum.photos/420/260".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_use_custom_class.get() {
                        "docs-coachmark-state".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <Coachmark
                            variant=variant
                            title="Workbench coachmark".to_string()
                            open=workbench_open
                            on_open_change=on_workbench_open_change
                            disabled=disabled
                            current_step=current_step
                            total_steps=total_steps
                            primary_cta=primary_cta
                            secondary_cta=secondary_cta
                            asset_variant=CoachmarkAssetVariant::Folder
                            asset_src=asset_src
                            class_name=class_name
                        >
                            <div class="docs-stack docs-stack--tight">
                                <div>"Inspect display/config/code/css-test panels together."</div>
                                <div class="ui-muted">"Open state is controlled so settings are reproducible for regression review."</div>
                            </div>
                        </Coachmark>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
