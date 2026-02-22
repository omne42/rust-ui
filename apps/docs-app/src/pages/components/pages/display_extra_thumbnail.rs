use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Thumbnail, ThumbnailMotion, ThumbnailSize};
use ui_headless::A11yDirection;

const THUMBNAIL_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Thumbnail, ThumbnailMotion, ThumbnailSize};\nuse ui_headless::A11yDirection;";

pub(super) fn thumbnail() -> AnyView {
    let (size_index, set_size_index) = signal(1usize);
    let (has_background, set_has_background) = signal(false);
    let (cover, set_cover) = signal(true);
    let (layer, set_layer) = signal(false);
    let (selected, set_selected) = signal(false);
    let (focused, set_focused) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_size = Signal::derive(move || match size_index.get() {
        0 => ThumbnailSize::Size100,
        2 => ThumbnailSize::Size900,
        _ => ThumbnailSize::Size500,
    });
    let workbench_background = Signal::derive(move || {
        if has_background.get() {
            "#0f172a".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ThumbnailMotion {
                active_scale: 1.08,
                active_ring_opacity: 0.9,
                ..ThumbnailMotion::default()
            }
        } else {
            ThumbnailMotion::default()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-thumbnail-custom".to_string()
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

    let showcase_code = Signal::derive(move || {
        r#"<Thumbnail>
  <img src=\"https://picsum.photos/240/160\" alt=\"Thumbnail sample\" />
</Thumbnail>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Thumbnail\n  size=ThumbnailSize::{:?}\n  background={}\n  cover={}\n  layer={}\n  selected={}\n  focused={}\n  motion={:?}\n  class_name={}\n  lang={}\n  dir=A11yDirection::{}\n>\n  <img src=\"https://picsum.photos/500/300\" alt=\"Workbench sample\" />\n</Thumbnail>",
            workbench_size.get(),
            rust_string_literal(&workbench_background.get()),
            bool_word(cover.get()),
            bool_word(layer.get()),
            bool_word(selected.get()),
            bool_word(focused.get()),
            workbench_motion.get(),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<Thumbnail size=ThumbnailSize::Size100>
  <img src=\"https://picsum.photos/180/120\" alt=\"Small landscape\" />
</Thumbnail>
<Thumbnail
  size=ThumbnailSize::Size500
  background=\"#0f172a\".into()
  cover=true
  layer=true
  selected=true
  focused=true
  class_name=\"docs-thumbnail-custom\".into()
  motion=ThumbnailMotion { active_scale: 1.08, active_ring_opacity: 0.9, ..ThumbnailMotion::default() }
  lang=\"en-US\".into()
  dir=A11yDirection::Ltr
>
  <img src=\"https://picsum.photos/500/300\" alt=\"Selected cover\" />
</Thumbnail>
<Thumbnail
  size=ThumbnailSize::Size900
  background=\"#111827\".into()
  cover=false
  layer=false
  selected=false
  focused=true
  motion=ThumbnailMotion::disabled()
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
>
  <img src=\"https://picsum.photos/500/120\" alt=\"Wide rtl\" />
</Thumbnail>"##
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/thumbnail/src/styles.rs */\\n{}",
            ui::thumbnail::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ThumbnailActualConfig {{\\n  size: {:?},\\n  background: {:?},\\n  cover: {},\\n  layer: {},\\n  selected: {},\\n  focused: {},\\n  motion: {:?},\\n  class_name: {:?},\\n  lang: {:?},\\n  dir: {:?},\\n}}",
            workbench_size.get(),
            workbench_background.get(),
            cover.get(),
            layer.get(),
            selected.get(),
            focused.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="Thumbnail"
            slug="thumbnail"
            group="Display"
            description="Media thumbnail primitive with full API workbench and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=THUMBNAIL_DOC_IMPORTS.to_string()
            >
                <Thumbnail>
                    <img src="https://picsum.photos/240/160" alt="Thumbnail sample" />
                </Thumbnail>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=THUMBNAIL_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="components/thumbnail/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="thumbnail-workbench-controls">
                        <div class="docs-search__label">"Size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || size_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_size_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"100"</option>
                            <option value="1">"500"</option>
                            <option value="2">"900"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || has_background.get()
                                on:change=move |event| set_has_background.set(event_target_checked(&event))
                            />
                            <span>"Background"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || cover.get()
                                on:change=move |event| set_cover.set(event_target_checked(&event))
                            />
                            <span>"Cover"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || layer.get()
                                on:change=move |event| set_layer.set(event_target_checked(&event))
                            />
                            <span>"Layer"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || selected.get()
                                on:change=move |event| set_selected.set(event_target_checked(&event))
                            />
                            <span>"Selected"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || focused.get()
                                on:change=move |event| set_focused.set(event_target_checked(&event))
                            />
                            <span>"Focused"</span>
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
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
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
                <Thumbnail
                    size=workbench_size.get()
                    background=workbench_background.get()
                    cover=cover.get()
                    layer=layer.get()
                    selected=selected.get()
                    focused=focused.get()
                    motion=workbench_motion.get()
                    class_name=workbench_class_name.get()
                    lang=workbench_lang.get()
                    dir=workbench_dir.get()
                >
                    <img src="https://picsum.photos/500/300" alt="Workbench sample" />
                </Thumbnail>
            </Playground>

            <Playground
                title="State Matrix (Size / Selection / Motion Comparison)"
                code_signal=matrix_code
                code_imports=THUMBNAIL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Thumbnail size=ThumbnailSize::Size100>
                        <img src="https://picsum.photos/180/120" alt="Small landscape" />
                    </Thumbnail>
                    <Thumbnail
                        size=ThumbnailSize::Size500
                        background="#0f172a".to_string()
                        cover=true
                        layer=true
                        selected=true
                        focused=true
                        class_name="docs-thumbnail-custom".to_string()
                        motion=ThumbnailMotion {
                            active_scale: 1.08,
                            active_ring_opacity: 0.9,
                            ..ThumbnailMotion::default()
                        }
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <img src="https://picsum.photos/500/300" alt="Selected cover" />
                    </Thumbnail>
                    <Thumbnail
                        size=ThumbnailSize::Size900
                        background="#111827".to_string()
                        cover=false
                        layer=false
                        selected=false
                        focused=true
                        motion=ThumbnailMotion::disabled()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <img src="https://picsum.photos/500/120" alt="Wide rtl" />
                    </Thumbnail>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
