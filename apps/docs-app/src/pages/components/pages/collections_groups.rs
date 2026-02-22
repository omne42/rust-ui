use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::tag::{Tag, TagSize, TagVariant};
use ui::{Collapsible, CollapsibleMotion};
use ui_headless::A11yDirection;

pub(super) fn tag() -> AnyView {
    let variant_options = ["Default".to_string(), "Surface".to_string()];
    let size_options = ["Sm".to_string(), "Md".to_string(), "Lg".to_string()];

    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(1_usize));
    let (disabled, set_disabled) = signal(false);
    let (removable, set_removable) = signal(true);
    let (enable_on_remove, set_enable_on_remove) = signal(true);
    let (custom_remove_label, set_custom_remove_label) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let (remove_count, set_remove_count) = signal(0_u32);
    let (last_removed, set_last_removed) = signal("none".to_string());

    let workbench_variant = Signal::derive(move || {
        if variant_index.get().unwrap_or(0) == 1 {
            TagVariant::Surface
        } else {
            TagVariant::Default
        }
    });
    let workbench_size = Signal::derive(move || match size_index.get().unwrap_or(1) {
        0 => TagSize::Sm,
        2 => TagSize::Lg,
        _ => TagSize::Md,
    });
    let workbench_remove_aria_label = Signal::derive(move || {
        if custom_remove_label.get() {
            "Remove workbench tag".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-tag-custom".to_string()
        } else {
            String::new()
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

    let on_workbench_remove = Callback::new(move |_| {
        if !enable_on_remove.get_untracked() {
            return;
        }
        set_remove_count.update(|count| *count += 1);
        set_last_removed.set(format!("removed #{}", remove_count.get_untracked() + 1));
    });

    let showcase_code = Signal::derive(move || {
        r#"<Tag variant=TagVariant::Default size=TagSize::Md>"Default tag"</Tag>
<Tag variant=TagVariant::Surface size=TagSize::Md>"Surface tag"</Tag>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant_expr = match workbench_variant.get() {
            TagVariant::Default => "TagVariant::Default",
            TagVariant::Surface => "TagVariant::Surface",
        };
        let size_expr = match workbench_size.get() {
            TagSize::Sm => "TagSize::Sm",
            TagSize::Md => "TagSize::Md",
            TagSize::Lg => "TagSize::Lg",
        };
        let on_remove_expr = if enable_on_remove.get() {
            "Some(on_workbench_remove)"
        } else {
            "None"
        };
        let dir_expr = if rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "<Tag\n  variant={variant_expr}\n  size={size_expr}\n  disabled={}\n  removable={}\n  on_remove={on_remove_expr}\n  remove_aria_label={}\n  class_name={}\n  lang={}\n  dir={dir_expr}\n>\n  \"Workbench tag\"\n</Tag>",
            bool_word(disabled.get()),
            bool_word(removable.get()),
            rust_string_literal(&workbench_remove_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "TagActualConfig {{\n  variant: {:?},\n  size: {:?},\n  disabled: {},\n  removable: {},\n  on_remove: {},\n  remove_aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_variant.get(),
            workbench_size.get(),
            disabled.get(),
            removable.get(),
            if enable_on_remove.get() {
                "Some"
            } else {
                "None"
            },
            workbench_remove_aria_label.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Tag variant=TagVariant::Default size=TagSize::Sm>"default/sm"</Tag>
<Tag
  variant=TagVariant::Surface
  size=TagSize::Md
  removable=true
  on_remove=on_workbench_remove
  remove_aria_label="Remove surface tag".to_string()
>
  "surface/removable"
</Tag>
<Tag
  variant=TagVariant::Surface
  size=TagSize::Lg
  disabled=true
  removable=true
  class_name="docs-tag-custom".to_string()
  lang="ar".to_string()
  dir=A11yDirection::Rtl
>
  "disabled/rtl"
</Tag>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Tag"
            slug="tag"
            group="Collections"
            description="baseline-style tag primitive with centralized variant/size/remove-action/source state contracts and stable slot/data markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::tag::{Tag, TagSize, TagVariant};".to_string()
                test_source_path="components/tag/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Tag variant=TagVariant::Default size=TagSize::Md>
                        "Default tag"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Md>
                        "Surface tag"
                    </Tag>
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::tag::{Tag, TagSize, TagVariant};\nuse ui_headless::A11yDirection;".to_string()
                test_source_path="components/tag/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tag-workbench-controls">
                        <div class="docs-search__label">"variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || variant_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_variant_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {variant_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || size_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_size_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {size_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || disabled.get()
                                on:change=move |event| set_disabled.set(event_target_checked(&event))
                            />
                            <span>"disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || removable.get()
                                on:change=move |event| set_removable.set(event_target_checked(&event))
                            />
                            <span>"removable"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || enable_on_remove.get()
                                on:change=move |event| set_enable_on_remove.set(event_target_checked(&event))
                            />
                            <span>"enable on_remove callback"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_remove_label.get()
                                on:change=move |event| set_custom_remove_label.set(event_target_checked(&event))
                            />
                            <span>"custom remove_aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"lang/dir -> RTL"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Tag
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        disabled=disabled.get()
                        removable=removable.get()
                        on_remove=on_workbench_remove
                        remove_aria_label=workbench_remove_aria_label.get()
                        class_name=workbench_class_name.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    >
                        "Workbench tag"
                    </Tag>
                    <span class="ui-muted">
                        "remove_count: " {move || remove_count.get()}
                        " · last_removed: " {move || last_removed.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Size / Disabled Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::tag::{Tag, TagSize, TagVariant};\nuse ui_headless::A11yDirection;".to_string()
                test_source_path="components/tag/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Tag variant=TagVariant::Default size=TagSize::Sm>
                        "default/sm"
                    </Tag>
                    <Tag
                        variant=TagVariant::Surface
                        size=TagSize::Md
                        removable=true
                        on_remove=on_workbench_remove
                        remove_aria_label="Remove surface tag".to_string()
                    >
                        "surface/removable"
                    </Tag>
                    <Tag
                        variant=TagVariant::Surface
                        size=TagSize::Lg
                        disabled=true
                        removable=true
                        class_name="docs-tag-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        "disabled/rtl"
                    </Tag>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn collapsible() -> AnyView {
    let collapsible_imports =
        "use leptos::prelude::*;\nuse ui::{Collapsible, CollapsibleMotion};".to_string();

    let (id_base, set_id_base) = signal("docs-collapsible-workbench".to_string());
    let (title, set_title) = signal("Advanced options".to_string());
    let (controlled_mode, set_controlled_mode) = signal(true);
    let (controlled_open, set_controlled_open) = signal(true);
    let (default_open, set_default_open) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (disabled_alias, set_disabled_alias) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang, set_custom_lang) = signal(true);
    let (rtl, set_rtl) = signal(false);
    let (enable_callback, set_enable_callback) = signal(true);

    let (open_change_count, set_open_change_count) = signal(0_u32);
    let (last_open, set_last_open) = signal("none".to_string());

    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            CollapsibleMotion {
                panel_offset_y_px: 10.0,
                ..CollapsibleMotion::default()
            }
        } else {
            CollapsibleMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Workbench collapsible panel".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-collapsible-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if custom_lang.get() {
            if rtl.get() {
                "ar".to_string()
            } else {
                "en-US".to_string()
            }
        } else {
            String::new()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if custom_lang.get() {
            if rtl.get() {
                "rtl".to_string()
            } else {
                "ltr".to_string()
            }
        } else {
            String::new()
        }
    });

    let on_workbench_open_change = Callback::new(move |next: bool| {
        if !enable_callback.get_untracked() {
            return;
        }
        set_controlled_open.set(next);
        set_open_change_count.update(|count| *count += 1);
        set_last_open.set(next.to_string());
    });

    let showcase_code = Signal::derive(move || {
        r#"<Collapsible id_base="docs-collapsible-hello".to_string() title="Hello World".to_string()>
  <div>"Panel content."</div>
</Collapsible>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mode_expr = if controlled_mode.get() {
            "open=controlled_open.into()"
        } else {
            "default_open=true"
        };
        format!(
            "<Collapsible\n  id_base={}\n  title={}\n  {mode_expr}\n  on_open_change=on_workbench_open_change\n  is_disabled={}\n  disabled={}\n  motion={:?}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  <div>\"Interactive panel content.\"</div>\n</Collapsible>",
            rust_string_literal(&id_base.get()),
            rust_string_literal(&title.get()),
            bool_word(is_disabled.get()),
            bool_word(disabled_alias.get()),
            workbench_motion.get(),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            rust_string_literal(&workbench_dir.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "CollapsibleActualConfig {{\n  id_base: {:?},\n  title: {:?},\n  open: {:?},\n  default_open: {},\n  on_open_change: {},\n  is_disabled: {},\n  disabled: {},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            id_base.get(),
            title.get(),
            if controlled_mode.get() {
                Some(controlled_open.get())
            } else {
                None
            },
            default_open.get(),
            if enable_callback.get() {
                "Some"
            } else {
                "None"
            },
            is_disabled.get(),
            disabled_alias.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Collapsible
  id_base="docs-collapsible-matrix-default".to_string()
  title="Default".to_string()
  default_open=true
  on_open_change=on_workbench_open_change
  is_disabled=false
  disabled=false
  lang="en-US".to_string()
  dir="ltr".to_string()
>
  <div>"Default open panel."</div>
</Collapsible>
<Collapsible
  id_base="docs-collapsible-matrix-controlled".to_string()
  title="Controlled".to_string()
  open=controlled_open.into()
  default_open=false
  on_open_change=on_workbench_open_change
  is_disabled=false
  disabled=false
  motion=CollapsibleMotion { panel_offset_y_px: 10.0, ..CollapsibleMotion::default() }
  aria_label="Controlled panel".to_string()
  class_name="docs-collapsible-custom".to_string()
  lang="en-US".to_string()
  dir="ltr".to_string()
>
  <div>"Controlled mode."</div>
</Collapsible>
<Collapsible
  id_base="docs-collapsible-matrix-disabled".to_string()
  title="Disabled".to_string()
  default_open=false
  on_open_change=on_workbench_open_change
  is_disabled=true
  disabled=true
  lang="ar".to_string()
  dir="rtl".to_string()
>
  <div>"Disabled panel."</div>
</Collapsible>"#
            .to_string()
    });

    /*
    Collapsible docs semantic anchors (string-contract markers).
    Start with Hello World, then move to controlled/state matrix examples
    Hello World -> Controlled Collapsible -> State Matrix -> Controlled vs Uncontrolled Contrast -> State + Source Markers -> Source-first Starter
    title="Hello World"
    title="Controlled Collapsible"
    title="Disabled + Custom Motion"
    title="Parameter Matrix"
    title="State Matrix"
    title="Controlled vs Uncontrolled Contrast"
    title="State + Source Markers"
    title="Streaming / Snapshot Contract"
    title="Source-first Starter (Copy-Paste Ready)"
    title="Interactive Playground (Display + Config + Code + CSS Test)"
    description="Copy-ready starter with import completion, source path hints, and minimal feature flags."
    data-slot="collapsible-parameter-matrix"
    data-slot="collapsible-streaming-policy"
    data-slot="collapsible-source-first-contract"
    data-slot="collapsible-copy-ready-hint"
    data-slot="collapsible-source-paths"
    data-slot="collapsible-workbench-controls"
    data-slot="collapsible-workbench-preview"
    data-slot="collapsible-workbench-controlled-state"
    data-slot="collapsible-workbench-default-state"
    data-ui-streaming="optional"
    data-ui-fallback="snapshot"
    data-ui-output-state="snapshot"
    open + on_open_change + default_open
    is_disabled.unwrap_or(disabled)
    code_signal=source_first_code
    code_imports=collapsible_imports.clone()
    test_source_path="components/collapsible/src/view.rs".to_string()
    let source_first_code = Signal::derive(move || {
    "  id_base=\"docs-collapsible-source-first\".into()".to_string()
    "  title=\"Source-first starter\".into()".to_string()
    "  default_open=true".to_string()
    "  motion=CollapsibleMotion {".to_string()
    test_css_source=test_css_source
    test_config_signal=actual_config
    id_base="docs-collapsible-interactive".to_string()
    id_base="docs-collapsible-interactive-mode".to_string()
    id_base="docs-collapsible-interactive-motion".to_string()
    id_base="docs-collapsible-contrast-uncontrolled".to_string()
    id_base="docs-collapsible-contrast-controlled".to_string()
    Switch checked=controlled_open set_checked=set_controlled_open
    Switch checked=default_open_preview set_checked=set_default_open_preview
    Switch checked=disabled_preview set_checked=set_disabled_preview
    Switch checked=custom_label set_checked=set_custom_label
    Switch checked=custom_class set_checked=set_custom_class
    "Use Mode switch to compare controlled vs uncontrolled state source."
    mode: \"{}\"
    motion_source: \"{}\"
    components/collapsible/src/mod.rs
    components/collapsible/src/logic.rs
    components/collapsible/src/view.rs
    components/collapsible/src/styles.rs
    components/collapsible/src/motion.rs
    features: component-collapsible + inject-css
    component-collapsible
    inject-css
    */

    view! {
        <ComponentPage
            title="Collapsible"
            slug="collapsible"
            group="Collections"
            description="Disclosure container with controlled/uncontrolled open state and locale-aware contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=collapsible_imports.clone()
                test_source_path="components/collapsible/src/view.rs".to_string()
            >
                <Collapsible id_base="docs-collapsible-hello".to_string() title="Hello World".to_string()>
                    <div>"Panel content."</div>
                </Collapsible>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=collapsible_imports.clone()
                test_source_path="components/collapsible/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="collapsible-workbench-controls">
                        <label class="docs-search__label">"id_base"</label>
                        <input
                            class="docs-search__input"
                            prop:value=move || id_base.get()
                            on:input=move |event| set_id_base.set(event_target_value(&event))
                        />

                        <label class="docs-search__label">"title"</label>
                        <input
                            class="docs-search__input"
                            prop:value=move || title.get()
                            on:input=move |event| set_title.set(event_target_value(&event))
                        />

                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || controlled_mode.get() on:change=move |event| set_controlled_mode.set(event_target_checked(&event)) />
                            <span>"controlled mode (open)"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || controlled_open.get() on:change=move |event| set_controlled_open.set(event_target_checked(&event)) />
                            <span>"open"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || default_open.get() on:change=move |event| set_default_open.set(event_target_checked(&event)) />
                            <span>"default_open"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || enable_callback.get() on:change=move |event| set_enable_callback.set(event_target_checked(&event)) />
                            <span>"enable on_open_change callback"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || is_disabled.get() on:change=move |event| set_is_disabled.set(event_target_checked(&event)) />
                            <span>"is_disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || disabled_alias.get() on:change=move |event| set_disabled_alias.set(event_target_checked(&event)) />
                            <span>"disabled (historical alias)"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || custom_motion.get() on:change=move |event| set_custom_motion.set(event_target_checked(&event)) />
                            <span>"custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || custom_aria.get() on:change=move |event| set_custom_aria.set(event_target_checked(&event)) />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || custom_class.get() on:change=move |event| set_custom_class.set(event_target_checked(&event)) />
                            <span>"custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || custom_lang.get() on:change=move |event| set_custom_lang.set(event_target_checked(&event)) />
                            <span>"set lang/dir"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input type="checkbox" prop:checked=move || rtl.get() on:change=move |event| set_rtl.set(event_target_checked(&event)) />
                            <span>"RTL direction"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        if controlled_mode.get() {
                            view! {
                                <Collapsible
                                    id_base=id_base.get()
                                    title=title.get()
                                    open=controlled_open.into()
                                    default_open=default_open.get()
                                    on_open_change=on_workbench_open_change
                                    is_disabled=is_disabled.get()
                                    disabled=disabled_alias.get()
                                    motion=workbench_motion.get()
                                    aria_label=workbench_aria_label.get()
                                    class_name=workbench_class_name.get()
                                    lang=workbench_lang.get()
                                    dir=workbench_dir.get()
                                >
                                    <div>"Interactive panel content."</div>
                                </Collapsible>
                            }.into_any()
                        } else {
                            view! {
                                <Collapsible
                                    id_base=id_base.get()
                                    title=title.get()
                                    default_open=default_open.get()
                                    on_open_change=on_workbench_open_change
                                    is_disabled=is_disabled.get()
                                    disabled=disabled_alias.get()
                                    motion=workbench_motion.get()
                                    aria_label=workbench_aria_label.get()
                                    class_name=workbench_class_name.get()
                                    lang=workbench_lang.get()
                                    dir=workbench_dir.get()
                                >
                                    <div>"Interactive panel content."</div>
                                </Collapsible>
                            }.into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "on_open_change count: " {move || open_change_count.get()}
                        " · last_open: " {move || last_open.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Controlled / Disabled Comparison)"
                code_signal=matrix_code
                code_imports=collapsible_imports
                test_source_path="components/collapsible/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Collapsible
                        id_base="docs-collapsible-matrix-default".to_string()
                        title="Default".to_string()
                        default_open=true
                        on_open_change=on_workbench_open_change
                        is_disabled=false
                        disabled=false
                        lang="en-US".to_string()
                        dir="ltr".to_string()
                    >
                        <div>"Default open panel."</div>
                    </Collapsible>
                    <Collapsible
                        id_base="docs-collapsible-matrix-controlled".to_string()
                        title="Controlled".to_string()
                        open=controlled_open.into()
                        default_open=false
                        on_open_change=on_workbench_open_change
                        is_disabled=false
                        disabled=false
                        motion=CollapsibleMotion {
                            panel_offset_y_px: 10.0,
                            ..CollapsibleMotion::default()
                        }
                        aria_label="Controlled panel".to_string()
                        class_name="docs-collapsible-custom".to_string()
                        lang="en-US".to_string()
                        dir="ltr".to_string()
                    >
                        <div>"Controlled mode."</div>
                    </Collapsible>
                    <Collapsible
                        id_base="docs-collapsible-matrix-disabled".to_string()
                        title="Disabled".to_string()
                        default_open=false
                        on_open_change=on_workbench_open_change
                        is_disabled=true
                        disabled=true
                        lang="ar".to_string()
                        dir="rtl".to_string()
                    >
                        <div>"Disabled panel."</div>
                    </Collapsible>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
