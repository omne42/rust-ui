use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Button, ButtonVariant, Coachmark, CoachmarkAssetVariant, CoachmarkVariant, OnPress, Snippet,
};

const COACHMARK_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{Coachmark, CoachmarkAssetVariant, CoachmarkVariant};";
const COACHMARK_CONTROLLED_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant, Coachmark, CoachmarkAssetVariant};";

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

    let hello_world_code = Signal::derive(move || {
        r#"<Coachmark title=\"Welcome\".into() default_open=true>
  <div>Tour copy</div>
</Coachmark>"#
            .to_string()
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

    let state_matrix_code = Signal::derive(move || {
        r#"<Coachmark title=\"Help variant\".into() default_open=true current_step=1 total_steps=3 primary_cta=\"Next\".into() asset_variant=CoachmarkAssetVariant::Folder>
  <div>Default help intent with built-in asset.</div>
</Coachmark>
<Coachmark variant=CoachmarkVariant::Info title=\"Info variant\".into() default_open=true primary_cta=\"Understood\".into() asset_src=\"https://picsum.photos/420/260\".into()>
  <div>Info intent with external image source.</div>
</Coachmark>
<Coachmark title=\"Disabled preview\".into() default_open=true is_disabled=true secondary_cta=\"Dismiss\".into()>
  <div>Disabled state keeps semantic markers for testing.</div>
</Coachmark>"#.to_string()
    });

    let streaming_snapshot_code = Signal::derive(move || {
        r#"<Coachmark
  title="Snapshot baseline".into()
  default_open=true
  current_step=1
  total_steps=2
  primary_cta="Next".into()
>
  <div>Snapshot baseline; complete result renders in one pass.</div>
</Coachmark>
// Streaming Optional; fallback=snapshot."#
            .to_string()
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
        let is_disabled = if workbench_disabled.get() {
            "  is_disabled=true\n"
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
            "let (open, set_open) = signal({});\n\n<Coachmark\n  title=\"Workbench coachmark\".into()\n{variant}{is_disabled}  open=Signal::derive(move || open.get())\n  on_open_change=Callback::new(move |next| set_open.set(next))\n{steps}{cta}{asset}{class_name}>\n  <div>Inspect display/config/code/css-test panels together.</div>\n</Coachmark>",
            workbench_open_raw.get()
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/coachmark/src/styles.rs */\n{}",
            ui::coachmark::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = if workbench_info_variant.get() {
            "Info"
        } else {
            "Help"
        };
        let current_step = if workbench_show_steps.get() {
            "Some(2)"
        } else {
            "None"
        };
        let total_steps = if workbench_show_steps.get() {
            "Some(5)"
        } else {
            "None"
        };
        let primary_cta = if workbench_dual_cta.get() {
            "Some(\"Next\")"
        } else {
            "Some(\"Got it\")"
        };
        let secondary_cta = if workbench_dual_cta.get() {
            "Some(\"Back\")"
        } else {
            "None"
        };
        let on_secondary = if workbench_dual_cta.get() {
            "Some(\"OnPress\")"
        } else {
            "None"
        };
        let asset_variant = if workbench_use_image.get() {
            "None"
        } else {
            "Some(Folder)"
        };
        let asset_src = if workbench_use_image.get() {
            "Some(\"https://picsum.photos/420/260\")"
        } else {
            "None"
        };
        let class_name = if workbench_use_custom_class.get() {
            "Some(\"docs-coachmark-state\")"
        } else {
            "None"
        };
        format!(
            "CoachmarkWorkbenchConfig {{\n  variant: {variant},\n  aria_label: Some(\"Coachmark workbench\"),\n  is_disabled: Some({}),\n  disabled: Some({}),\n  placement: BottomStart,\n  motion: CoachmarkMotion::default,\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: Some(\"Callback<bool>\"),\n  title: Some(\"Workbench coachmark\"),\n  class_name: {class_name},\n  current_step: {current_step},\n  total_steps: {total_steps},\n  primary_cta: {primary_cta},\n  secondary_cta: {secondary_cta},\n  on_primary: Some(\"OnPress\"),\n  on_secondary: {on_secondary},\n  shortcut_key: Some(\"K\"),\n  modifier_keys: [\"⌘\"],\n  asset_variant: {asset_variant},\n  asset_label: Some(\"Tour folder\"),\n  asset_src: {asset_src},\n  asset_alt: Some(\"Coachmark image\"),\n  lang: Some(\"en\"),\n  dir: Some(A11yDirection::Ltr),\n  actions: Some(\"ViewFn\"),\n}}",
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_open_raw.get(),
        )
    });

    view! {
        <ComponentPage
            title="Coachmark"
            slug="coachmark"
            group="Overlays"
            description="baseline-compatible Coachmark primitive for guided tours, composed on ContextualHelp/Popover contracts with baseline-level spring overlay motion and optional asset + CTA navigation semantics."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
            >
                <Coachmark title="Welcome".to_string() default_open=true>
                    <div>"Tour copy"</div>
                </Coachmark>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Uncontrolled path uses default_open; controlled path binds open + on_open_change so parent state remains the single source of truth."
                code_signal=controlled_code
                code_imports=COACHMARK_CONTROLLED_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="coachmark-controlled-vs-uncontrolled"
                >
                    <Coachmark title="Uncontrolled baseline".to_string() default_open=true>
                        <div>"Uncontrolled branch keeps internal open state after initialization."</div>
                    </Coachmark>
                    <div class="docs-row" attr:data-slot="coachmark-controlled-actions">
                        <Button
                            variant=ButtonVariant::Secondary
                            attr:data-slot="coachmark-controlled-toggle"
                            on_press=toggle_controlled
                        >
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
                    title="Config + Code + CSS Test Workbench"
                    description="Interactive acceptance surface: edit props/state in settings, inspect preview, then verify code + scoped CSS + actual config in one place."
                    code_signal=workbench_code
                    code_imports=COACHMARK_DOC_IMPORTS.to_string()
                    test_css_source=workbench_test_css_source
                    test_source_path="components/coachmark/src/styles.rs".to_string()
                    test_config_signal=workbench_actual_config
                    controls=move || {
                        view! {
                            <div class="docs-stack docs-stack--tight" attr:data-slot="coachmark-workbench-controls">
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-variant">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_info_variant.get()
                                        on:change=move |ev| set_workbench_info_variant.set(event_target_checked(&ev))
                                    />
                                    <span>"Info variant"</span>
                                </label>
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-disabled">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_disabled.get()
                                        on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                                    />
                                    <span>"Disabled"</span>
                                </label>
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-steps">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_show_steps.get()
                                        on:change=move |ev| set_workbench_show_steps.set(event_target_checked(&ev))
                                    />
                                    <span>"Show step counter"</span>
                                </label>
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-cta">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_dual_cta.get()
                                        on:change=move |ev| set_workbench_dual_cta.set(event_target_checked(&ev))
                                    />
                                    <span>"Dual CTA"</span>
                                </label>
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-asset">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_use_image.get()
                                        on:change=move |ev| set_workbench_use_image.set(event_target_checked(&ev))
                                    />
                                    <span>"Use image asset"</span>
                                </label>
                                <label class="docs-choice-row" attr:data-slot="coachmark-workbench-toggle-class">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || workbench_use_custom_class.get()
                                        on:change=move |ev| set_workbench_use_custom_class.set(event_target_checked(&ev))
                                    />
                                    <span>"Enable custom class source"</span>
                                </label>
                                <Button
                                    variant=ButtonVariant::Secondary
                                    attr:data-slot="coachmark-workbench-toggle-open"
                                    on_press=toggle_workbench_open
                                >
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
                        let is_disabled = workbench_disabled.get();
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
                            <div attr:data-slot="coachmark-workbench-preview">
                                <Coachmark
                                    variant=variant
                                    title="Workbench coachmark".to_string()
                                    aria_label="Coachmark workbench".to_string()
                                    open=workbench_open
                                    default_open=false
                                    on_open_change=on_workbench_open_change
                                    is_disabled=is_disabled
                                    disabled=is_disabled
                                    placement=ui_headless::PopoverPlacement::BottomStart
                                    motion=Default::default()
                                    current_step=current_step
                                    total_steps=total_steps
                                    primary_cta=primary_cta
                                    secondary_cta=secondary_cta
                                    on_primary=on_primary
                                    on_secondary=on_secondary
                                    shortcut_key="K".to_string()
                                    modifier_keys=vec!["⌘".to_string()]
                                    asset_variant=CoachmarkAssetVariant::Folder
                                    asset_label="Tour folder".to_string()
                                    asset_src=asset_src
                                    asset_alt="Coachmark image".to_string()
                                    lang="en".to_string()
                                    dir=ui_headless::A11yDirection::Ltr
                                    class_name=class_name
                                    actions=move || {
                                        view! { <Button variant=ButtonVariant::Secondary>"Restart"</Button> }
                                    }
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Inspect display/config/code/css-test panels together."</div>
                                        <div class="ui-muted">"Open state is controlled so settings are reproducible for regression review."</div>
                                    </div>
                                </Coachmark>
                            </div>
                        }
                    }}
                </Playground>

            <Playground
                title="State Matrix"
                description="Compare Help, Info, and Disabled states after workbench controls."
                code_signal=state_matrix_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="coachmark-state-matrix-after-workbench">
                    <Coachmark
                        title="Help state".to_string()
                        default_open=true
                        current_step=1
                        total_steps=3
                        primary_cta="Next".to_string()
                        asset_variant=CoachmarkAssetVariant::Folder
                    >
                        <div>"Help intent with built-in asset."</div>
                    </Coachmark>
                    <Coachmark
                        variant=CoachmarkVariant::Info
                        title="Info state".to_string()
                        default_open=true
                        primary_cta="Understood".to_string()
                        asset_src="https://picsum.photos/420/260".to_string()
                        asset_alt="Info preview".to_string()
                    >
                        <div>"Info intent with external image."</div>
                    </Coachmark>
                    <Coachmark
                        title="Disabled state".to_string()
                        default_open=true
                        is_disabled=true
                        secondary_cta="Dismiss".to_string()
                    >
                        <div>"Disabled state for regression checks."</div>
                    </Coachmark>
                </div>
            </Playground>

            <Playground
                title="Step + CTA + Asset Variant"
                code_signal=basic_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
            >
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

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-open-mode`, `data-label-source`, `data-class-source`, and content-level `data-asset-source` for baseline-compatible coachmark contracts."
                code_signal=markers_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
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

            <section class="docs-card docs-prose" attr:data-slot="coachmark-defaults-contract">
                <h3>"API + Defaults Contract"</h3>
                <p>
                    "This page keeps docs API names and defaults aligned with "
                    <code>"components/coachmark/src/logic.rs"</code>
                    " and "
                    <code>"components/coachmark/src/view.rs"</code>
                    "."
                </p>
                <ul>
                    <li>
                        <code>"variant=CoachmarkVariant::Help"</code>
                        " (default via variant type default)."
                    </li>
                    <li>
                        <code>"default_open=false"</code>
                        " (from "
                        <code>"resolve_default_open(default_open.unwrap_or(false))"</code>
                        ")."
                    </li>
                    <li>
                        <code>"is_disabled=false"</code>
                        " when both "
                        <code>"is_disabled"</code>
                        " and "
                        <code>"disabled"</code>
                        " are unset (from "
                        <code>"resolve_is_disabled(is_disabled.or(disabled).unwrap_or(false))"</code>
                        ")."
                    </li>
                    <li>
                        "Controlled axis stays "
                        <code>"open + on_open_change"</code>
                        "; uncontrolled axis uses "
                        <code>"default_open"</code>
                        "."
                    </li>
                </ul>
            </section>

            <div attr:data-slot="coachmark-interactive-playground">
            <Playground
                title="Variant Gallery"
                description="Display matrix for common states to compare variant intent, asset source, and disabled behavior side by side."
                code_signal=state_matrix_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="coachmark-state-matrix">
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
                        is_disabled=true
                        secondary_cta="Dismiss".to_string()
                    >
                        <div>"Disabled state keeps semantic markers for testing."</div>
                    </Coachmark>
                </div>
            </Playground>
            </div>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Coachmark is not a long-form reader surface: it stays Snapshot-first and documents Streaming Optional with fallback=snapshot."
                code_signal=streaming_snapshot_code
                code_imports=COACHMARK_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="coachmark-streaming-modes">
                    <Coachmark
                        title="Snapshot baseline".to_string()
                        default_open=true
                        current_step=1
                        total_steps=2
                        primary_cta="Next".to_string()
                    >
                        <div>"Snapshot baseline; complete result renders in one pass."</div>
                    </Coachmark>
                    <div class="ui-muted" attr:data-slot="coachmark-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="coachmark-interactive-spec-linkage">
                <h3>"AI Spec Input -> Preview Output Linkage"</h3>
                <p>
                    "Workbench settings act as spec-like input, and the preview + Actual config panel show normalized output in real time."
                </p>
                <ul>
                    <li>
                        <code>"test_config_signal=workbench_actual_config"</code>
                        " exposes normalized config projection (Spec input snapshot) in the Playground test panel."
                    </li>
                    <li>
                        <code>"CoachmarkWorkbenchConfig { ... }"</code>
                        " reflects current interactive inputs (`variant/open/is_disabled/steps/cta/asset_source/class_source`)."
                    </li>
                    <li>
                        "Rendered preview remains machine-readable via "
                        <code>"data-ui-schema=\"ui.coachmark.agent-contract.v1\""</code>
                        " and related semantic markers."
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" attr:data-slot="coachmark-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul attr:data-slot="coachmark-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-coachmark"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{Coachmark, CoachmarkAssetVariant};\n\n<Coachmark title=\"Welcome\".to_string() default_open=true asset_variant=CoachmarkAssetVariant::Folder>\n  <div>Tour copy</div>\n</Coachmark>".to_string()
                    label="Copy coachmark starter".to_string()
                    copyable=true
                    class_name="docs-coachmark-source-copy".to_string()
                />
                <ul attr:data-slot="coachmark-source-paths">
                    <li><code>"components/coachmark/src/mod.rs"</code></li>
                    <li><code>"components/coachmark/src/logic.rs"</code></li>
                    <li><code>"components/coachmark/src/view.rs"</code></li>
                    <li><code>"components/coachmark/src/styles.rs"</code></li>
                    <li><code>"components/coachmark/src/motion.rs"</code></li>
                    <li><code>"components/coachmark/src/protocol.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
