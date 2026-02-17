use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ButtonMotion, ButtonSize, ButtonVariant, IconButton, OnPress, SegmentedControl,
    SegmentedControlSize, Switch,
};

pub(super) fn icon_button() -> AnyView {
    let _ = super::actions::icon_button as fn() -> AnyView;
    let (press_count, set_press_count) = signal(0_usize);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let variant_options = vec![
        "default".to_string(),
        "secondary".to_string(),
        "ghost".to_string(),
        "outline".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Secondary,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Outline,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::IconXs,
        1 => ButtonSize::IconS,
        2 => ButtonSize::Icon,
        3 => ButtonSize::IconL,
        _ => ButtonSize::IconXl,
    });

    let (disabled, set_disabled) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec![
            "<IconButton".to_string(),
            if custom_aria_label {
                "  aria_label=\"Inspect icon trigger\".to_string()".to_string()
            } else {
                "  aria_label=\"Search\".to_string()".to_string()
            },
        ];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::Icon {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  <span aria-hidden=\"true\">\"⌕\"</span>".to_string(),
            "</IconButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let (marker_count, set_marker_count) = signal(0_usize);
    let marker_press: OnPress = Callback::new(move |_| {
        set_marker_count.update(|count| *count += 1);
    });

    let states_code = Signal::derive(move || {
        r#"<IconButton aria_label="Search".to_string() size=ButtonSize::IconSm>
  <span aria-hidden="true">"⌕"</span>
</IconButton>
<IconButton aria_label="Search".to_string() size=ButtonSize::Icon>
  <span aria-hidden="true">"⌕"</span>
</IconButton>
<IconButton aria_label="Search".to_string() size=ButtonSize::IconLg disabled=true>
  <span aria-hidden="true">"⌕"</span>
</IconButton>"#
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<IconButton
  aria_label="Inspect icon trigger".to_string()
  variant=ButtonVariant::Secondary
  size=ButtonSize::Lg
  motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }
  class_name="docs-icon-button-state".to_string()
  on_press=Callback::new(move |_| { /* marker */ })
>
  <span aria-hidden="true">"⌕"</span>
</IconButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="IconButton"
            slug="icon-button"
            group="Actions"
            description="baseline-compatible IconButton alias with centralized aria/size/handler source contracts and baseline-level motion behavior via Button composition."
        >
            <Playground
                title="on_press + variants"
                code_signal=code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-icon-button-variant-extra".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-icon-button-size-extra".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconButton size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        {move || {
                            let variant = variant.get();
                            let size = size.get();
                            let disabled = disabled.get();

                            if custom_aria_label.get() {
                                view! {
                                    <IconButton
                                        aria_label="Inspect icon trigger".to_string()
                                        variant=variant
                                        size=size
                                        disabled=disabled
                                        on_press=on_press
                                    >
                                        <span aria-hidden="true">"⌕"</span>
                                    </IconButton>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <IconButton
                                        aria_label="Search".to_string()
                                        variant=variant
                                        size=size
                                        disabled=disabled
                                        on_press=on_press
                                    >
                                        <span aria-hidden="true">"⌕"</span>
                                    </IconButton>
                                }
                                    .into_any()
                            }
                        }}
                    </div>
                    <span class="ui-muted">
                        "presses: " {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Size + disabled matrix" code_signal=states_code>
                <div class="docs-row">
                    <IconButton aria_label="Search small".to_string() size=ButtonSize::IconSm>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton aria_label="Search default".to_string() size=ButtonSize::Icon>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton aria_label="Search large".to_string() size=ButtonSize::IconLg>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton
                        aria_label="Close disabled".to_string()
                        variant=ButtonVariant::Ghost
                        disabled=true
                    >
                        <span aria-hidden="true">"✕"</span>
                    </IconButton>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-size-mode`, `data-handler-source`, `data-label-source`, `data-class-source`, and `data-motion-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <IconButton
                        aria_label="Inspect icon trigger".to_string()
                        variant=ButtonVariant::Secondary
                        size=ButtonSize::Lg
                        motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }
                        class_name="docs-icon-button-state".to_string()
                        on_press=marker_press
                    >
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <span class="ui-muted">"presses: " {move || marker_count.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
