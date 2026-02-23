use super::*;

pub(crate) fn scroll_area() -> AnyView {
    let (marker_orientation, set_marker_orientation) = signal(ScrollAreaOrientation::Vertical);
    let (marker_is_disabled, set_marker_is_disabled) = signal(false);
    let (marker_has_custom_max_height, set_marker_has_custom_max_height) = signal(true);
    let (marker_has_custom_class, set_marker_has_custom_class) = signal(false);
    let (marker_has_custom_aria, set_marker_has_custom_aria) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ScrollArea>
  <div>"Activity feed"</div>
</ScrollArea>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<ScrollArea max_height_px=180>
  <div class="docs-stack docs-stack--tight">
    {(1..=24)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Release note {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ScrollArea
  orientation=ScrollAreaOrientation::Horizontal
  max_height_px=120
  class_name="docs-scroll-area-custom".to_string()
>
  <div class="docs-row">
    {(1..=16)
      .map(|idx| {
        view! { <span class="ui-chip ui-chip--flat docs-scroll-area-chip">{format!("Tag {idx}")}</span> }
      })
      .collect_view()}
  </div>
</ScrollArea>

<ScrollArea
  orientation=ScrollAreaOrientation::Both
  is_disabled=Some(true)
  max_height_px=120
  aria_label="Disabled logs".to_string()
>
  <div class="docs-scroll-area-grid">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Cell {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#.to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (marker_orientation, set_marker_orientation) = signal(ScrollAreaOrientation::Vertical);
let (marker_is_disabled, set_marker_is_disabled) = signal(false);
let (marker_has_custom_max_height, set_marker_has_custom_max_height) = signal(true);
let (marker_has_custom_class, set_marker_has_custom_class) = signal(false);
let (marker_has_custom_aria, set_marker_has_custom_aria) = signal(false);

<ScrollArea
  orientation=marker_orientation.get()
  is_disabled=Some(marker_is_disabled.get())
  max_height_px=if marker_has_custom_max_height.get() { Some(140) } else { None }
  class_name=if marker_has_custom_class.get() { "docs-scroll-area-custom".to_string() } else { "".to_string() }
  aria_label=if marker_has_custom_aria.get() { "Marker logs".to_string() } else { "".to_string() }
>
  <div class="docs-stack docs-stack--tight">
    {(1..=20)
      .map(|idx| {
        view! { <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div> }
      })
      .collect_view()}
  </div>
</ScrollArea>"#.to_string()
    });

    view! {
        <ComponentPage
            title="ScrollArea"
            slug="scroll-area"
            group="Layout"
            description="baseline-compatible scroll container with centralized orientation/max-height/disabled normalization and stable state-marker data contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ScrollArea>
                    <div>"Activity feed"</div>
                </ScrollArea>
            </Playground>

            <Playground title="Vertical + Max Height" code_signal=default_code>
                <ScrollArea max_height_px=180>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=24)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Release note {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollArea>
            </Playground>

            <Playground title="Horizontal + Both + Disabled" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <ScrollArea
                        orientation=ScrollAreaOrientation::Horizontal
                        max_height_px=120
                        class_name="docs-scroll-area-custom".to_string()
                    >
                        <div class="docs-row">
                            {(1..=16)
                                .map(|idx| {
                                    view! {
                                        <span class="ui-chip ui-chip--flat docs-scroll-area-chip">
                                            {format!("Tag {idx}")}
                                        </span>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </ScrollArea>

                    <ScrollArea
                        orientation=ScrollAreaOrientation::Both
                        is_disabled=true
                        max_height_px=120
                        aria_label="Disabled logs".to_string()
                    >
                        <div class="docs-scroll-area-grid">
                            {(1..=20)
                                .map(|idx| {
                                    view! {
                                        <div class="docs-scroll-shadow-item">
                                            {format!("Cell {idx}")}
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </ScrollArea>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                description="Toggle orientation/disabled/max-height/class/aria inputs and inspect live `data-*` + `aria-*` contracts."
                code_signal=marker_code
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        if marker_has_custom_max_height.get() {
                            view! {
                                <ScrollArea
                                    orientation=marker_orientation.get()
                                    is_disabled=marker_is_disabled.get()
                                    max_height_px=140
                                    class_name=if marker_has_custom_class.get() {
                                        "docs-scroll-area-custom".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                    aria_label=if marker_has_custom_aria.get() {
                                        "Marker logs".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        {(1..=20)
                                            .map(|idx| {
                                                view! {
                                                    <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </ScrollArea>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ScrollArea
                                    orientation=marker_orientation.get()
                                    is_disabled=marker_is_disabled.get()
                                    class_name=if marker_has_custom_class.get() {
                                        "docs-scroll-area-custom".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                    aria_label=if marker_has_custom_aria.get() {
                                        "Marker logs".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        {(1..=20)
                                            .map(|idx| {
                                                view! {
                                                    <div class="docs-scroll-shadow-item">{format!("Marker row {idx}")}</div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </ScrollArea>
                            }
                                .into_any()
                        }
                    }}

                    <div class="docs-row" data-slot="scroll-area-marker-controls">
                        <div data-slot="scroll-area-toggle-orientation">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_orientation.update(|value| {
                                        *value = match *value {
                                            ScrollAreaOrientation::Vertical =>
                                                ScrollAreaOrientation::Horizontal,
                                            ScrollAreaOrientation::Horizontal =>
                                                ScrollAreaOrientation::Both,
                                            ScrollAreaOrientation::Both =>
                                                ScrollAreaOrientation::Vertical,
                                        };
                                    })
                                })
                            >
                                {move || format!("Orientation: {}", match marker_orientation.get() {
                                    ScrollAreaOrientation::Vertical => "vertical",
                                    ScrollAreaOrientation::Horizontal => "horizontal",
                                    ScrollAreaOrientation::Both => "both",
                                })}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-disabled">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_is_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_is_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-max-height">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_max_height.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_max_height.get() {
                                    "Use default max height"
                                } else {
                                    "Use custom max height"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-class">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_class.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_class.get() {
                                    "Clear custom class"
                                } else {
                                    "Set custom class"
                                }}
                            </ui::Button>
                        </div>

                        <div data-slot="scroll-area-toggle-aria">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_has_custom_aria.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_has_custom_aria.get() {
                                    "Use default aria label"
                                } else {
                                    "Use custom aria label"
                                }}
                            </ui::Button>
                        </div>
                    </div>

                    <span class="ui-muted" data-slot="scroll-area-marker-summary">
                        "orientation="
                        {move || match marker_orientation.get() {
                            ScrollAreaOrientation::Vertical => "vertical",
                            ScrollAreaOrientation::Horizontal => "horizontal",
                            ScrollAreaOrientation::Both => "both",
                        }}
                        " · disabled="
                        {move || if marker_is_disabled.get() { "true" } else { "false" }}
                        " · max-height="
                        {move || if marker_has_custom_max_height.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                        " · class="
                        {move || if marker_has_custom_class.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                        " · aria="
                        {move || if marker_has_custom_aria.get() {
                            "custom"
                        } else {
                            "default"
                        }}
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="scroll-area-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Snippets are import-ready through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_layout::{ScrollArea, ScrollAreaOrientation};\n\n<ScrollArea\n  orientation=ScrollAreaOrientation::Vertical\n  max_height_px=180\n  aria_label=\"Release feed\".into()\n>\n  <div class=\"docs-stack docs-stack--tight\">\n    <div>\"Release note 1\"</div>\n    <div>\"Release note 2\"</div>\n  </div>\n</ScrollArea>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-scroll-area-source-copy".to_string()
                />
                <ul data-slot="scroll-area-source-paths">
                    <li><code>"crates/ui-layout/src/scroll_area/mod.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/logic.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/view.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/styles.rs"</code></li>
                    <li><code>"crates/ui-layout/src/scroll_area/motion.rs"</code></li>
                </ul>
                <ul data-slot="scroll-area-source-prerequisites">
                    <li><code>"component-scroll_area"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="scroll-area-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="scroll-area-api-rows">
                    <li>
                        <code>"orientation: ScrollAreaOrientation"</code>
                        " default = ScrollAreaOrientation::Vertical"
                    </li>
                    <li>
                        <code>"max_height_px: Option<u32>"</code>
                        " default = None (no custom max-height marker)"
                    </li>
                    <li>
                        <code>"is_disabled: Option<bool>"</code>
                        " None -> default(false), Some(v) -> is-prop"
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " fallback = ui_layout::scroll_area::DEFAULT_ARIA_LABEL"
                    </li>
                    <li>
                        <code>"class_name / lang / dir / motion"</code>
                        " optional style + locale + motion contract inputs"
                    </li>
                    <li>
                        <code>"children: Children"</code>
                        " explicit composition payload"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="scroll-area-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="scroll-area-state-rows">
                    <li>
                        <code>"data-orientation"</code>
                        " = vertical | horizontal | both"
                    </li>
                    <li>
                        <code>"data-disabled / data-disabled-source"</code>
                        " = true? and is-prop | default source provenance"
                    </li>
                    <li>
                        <code>"data-max-height / data-aria-source / data-class-source / data-custom-class"</code>
                        " = default | custom marker set"
                    </li>
                    <li>
                        <code>"data-ui-schema / data-ui-intent / data-ui-action / data-ui-state / data-ui-source / data-ui-output-status"</code>
                        " machine-readable agent contract + snapshot status markers"
                    </li>
                    <li>
                        <code>"tabindex / aria-disabled"</code>
                        " viewport accessibility path = enabled(0,None) | disabled(-1,true)"
                    </li>
                    <li>
                        <code>"controlled/uncontrolled value axis"</code>
                        " N/A for ScrollArea (no value/open state machine)"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
