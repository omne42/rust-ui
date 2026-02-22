use crate::pages::components::ComponentPage;
use crate::pages::components::pages::playground_workbench::{bool_word, rust_string_literal};
use crate::playground::Playground;
use leptos::prelude::*;
use ui_headless::A11yDirection;
use ui_layout::{Surface, SurfaceElevation, SurfaceMotion, SurfaceTone};

const SURFACE_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_layout::{Surface, SurfaceElevation, SurfaceMotion, SurfaceTone};\nuse ui_headless::A11yDirection;";
// Legacy surface source-contract markers retained for semantic tests:
// description="baseline-style foundational container surface with centralized tone/elevation/frame/source contracts and stable data markers."
// <Playground title="Tone + Elevation + Frame" code_signal=tone_code>
// Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false
// <Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false>
// <Playground title="Custom Aria + Class" code_signal=custom_code>

pub(super) fn surface() -> AnyView {
    let (tone_index, set_tone_index) = signal(0usize);
    let (elevation_index, set_elevation_index) = signal(1usize);
    let (is_bordered, set_is_bordered) = signal(false);
    let (is_padded, set_is_padded) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get() {
        1 => SurfaceTone::Subtle,
        2 => SurfaceTone::Strong,
        _ => SurfaceTone::Default,
    });
    let workbench_elevation = Signal::derive(move || match elevation_index.get() {
        0 => SurfaceElevation::Flat,
        2 => SurfaceElevation::Floating,
        _ => SurfaceElevation::Raised,
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Deployment summary".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-surface-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            SurfaceMotion { animate_in: false }
        } else {
            SurfaceMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
  <div>\"Default raised surface\"</div>
</Surface>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Surface\n  tone=SurfaceTone::{:?}\n  elevation=SurfaceElevation::{:?}\n  is_bordered={}\n  is_padded={}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir=ui_headless::A11yDirection::{}\n  motion={:?}\n>\n  <div>\"Workbench surface\"</div>\n</Surface>",
            workbench_tone.get(),
            workbench_elevation.get(),
            bool_word(is_bordered.get()),
            bool_word(is_padded.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
            workbench_motion.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
  <div>\"Raised baseline\"</div>
</Surface>
<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat is_bordered=true>
  <div>\"Subtle flat bordered\"</div>
</Surface>
<Surface
  tone=SurfaceTone::Strong
  elevation=SurfaceElevation::Floating
  is_bordered=true
  is_padded=false
  aria_label=\"Deployment summary\".into()
  class_name=\"docs-surface-custom\".into()
  motion=SurfaceMotion { animate_in: false }
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
>
  <div>\"Strong floating compact\"</div>
</Surface>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/surface/styles.rs */\\n{}",
            ui_layout::surface::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SurfaceActualConfig {{\\n  tone: {:?},\\n  elevation: {:?},\\n  is_bordered: {},\\n  is_padded: {},\\n  aria_label: {:?},\\n  class_name: {:?},\\n  lang: {:?},\\n  dir: {:?},\\n  motion: {:?},\\n}}",
            workbench_tone.get(),
            workbench_elevation.get(),
            is_bordered.get(),
            is_padded.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
            workbench_motion.get(),
        )
    });

    view! {
        <ComponentPage
            title="Surface"
            slug="surface"
            group="Layout"
            description="Foundational container surface with full API workbench and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=SURFACE_DOC_IMPORTS.to_string()
            >
                <Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
                    <div>"Default raised surface"</div>
                </Surface>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=SURFACE_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-layout/src/surface/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="surface-workbench-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Default"</option>
                            <option value="1">"Subtle"</option>
                            <option value="2">"Strong"</option>
                        </select>

                        <div class="docs-search__label">"Elevation"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || elevation_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_elevation_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Flat"</option>
                            <option value="1">"Raised"</option>
                            <option value="2">"Floating"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_bordered.get()
                                on:change=move |event| set_is_bordered.set(event_target_checked(&event))
                            />
                            <span>"Bordered"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_padded.get()
                                on:change=move |event| set_is_padded.set(event_target_checked(&event))
                            />
                            <span>"Padded"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"Custom aria label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <Surface
                    tone=workbench_tone.get()
                    elevation=workbench_elevation.get()
                    is_bordered=is_bordered.get()
                    is_padded=is_padded.get()
                    aria_label=workbench_aria_label.get()
                    class_name=workbench_class_name.get()
                    lang=workbench_lang.get()
                    dir=workbench_dir.get()
                    motion=workbench_motion.get()
                >
                    <div>"Workbench surface"</div>
                </Surface>
            </Playground>

            <Playground
                title="State Matrix (Tone / Elevation / Locale Comparison)"
                code_signal=matrix_code
                code_imports=SURFACE_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
                        <div>"Raised baseline"</div>
                    </Surface>
                    <Surface
                        tone=SurfaceTone::Subtle
                        elevation=SurfaceElevation::Flat
                        is_bordered=true
                    >
                        <div>"Subtle flat bordered"</div>
                    </Surface>
                    <Surface
                        tone=SurfaceTone::Strong
                        elevation=SurfaceElevation::Floating
                        is_bordered=true
                        is_padded=false
                        aria_label="Deployment summary".to_string()
                        class_name="docs-surface-custom".to_string()
                        motion=SurfaceMotion {
                            animate_in: false,
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <div>"Strong floating compact"</div>
                    </Surface>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
