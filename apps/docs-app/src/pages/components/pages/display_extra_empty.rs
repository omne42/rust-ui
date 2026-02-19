use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
    SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn empty() -> AnyView {
    let media_options = vec!["default".to_string(), "icon".to_string()];
    let (workbench_media_index, set_workbench_media_index) = signal(Some(1));
    let (workbench_show_content, set_workbench_show_content) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_slot_classes, set_workbench_custom_slot_classes) = signal(false);

    let workbench_media_variant =
        Signal::derive(move || match workbench_media_index.get().unwrap_or(1) {
            0 => EmptyMediaVariant::Default,
            _ => EmptyMediaVariant::Icon,
        });

    let workbench_code = Signal::derive(move || {
        let media_variant = workbench_media_variant.get();
        let show_content = workbench_show_content.get();
        let custom_class = workbench_custom_class.get();
        let custom_slot_classes = workbench_custom_slot_classes.get();

        let mut out = vec!["<Empty".to_string()];
        if custom_class {
            out.push("  class_name=\"docs-empty-custom\".into()".to_string());
        }
        out.push(">".to_string());
        if custom_slot_classes {
            out.push("  <EmptyHeader class_name=\"docs-empty-header\".into()>".to_string());
            out.push(format!(
                "    <EmptyMedia variant=EmptyMediaVariant::{media_variant:?} class_name=\"docs-empty-media\".into()>\"📦\"</EmptyMedia>"
            ));
            out.push("    <EmptyTitle class_name=\"docs-empty-title\".into()>\"No results\"</EmptyTitle>".to_string());
            out.push("    <EmptyDescription class_name=\"docs-empty-description\".into()>\"Try adjusting filters.\"</EmptyDescription>".to_string());
            out.push("  </EmptyHeader>".to_string());
        } else {
            out.push("  <EmptyHeader>".to_string());
            out.push(format!(
                "    <EmptyMedia variant=EmptyMediaVariant::{media_variant:?}>\"📦\"</EmptyMedia>"
            ));
            out.push("    <EmptyTitle>\"No results\"</EmptyTitle>".to_string());
            out.push(
                "    <EmptyDescription>\"Try adjusting filters.\"</EmptyDescription>".to_string(),
            );
            out.push("  </EmptyHeader>".to_string());
        }
        if show_content {
            if custom_slot_classes {
                out.push("  <EmptyContent class_name=\"docs-empty-content\".into()>".to_string());
            } else {
                out.push("  <EmptyContent>".to_string());
            }
            out.push("    <a href=\"#/components/search\">\"Open search\"</a>".to_string());
            out.push("  </EmptyContent>".to_string());
        }
        out.push("</Empty>".to_string());
        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let media_variant = workbench_media_variant.get();
        let show_content = workbench_show_content.get();
        let custom_class = workbench_custom_class.get();
        let custom_slot_classes = workbench_custom_slot_classes.get();

        format!(
            "EmptyActualConfig {{\n  media_variant: {media_variant:?},\n  show_content: {show_content},\n  custom_root_class: {custom_class},\n  custom_slot_classes: {custom_slot_classes},\n  marker_expectations: [\"data-slot\", \"data-state\", \"data-class-source\", \"data-variant\", \"data-variant-source\"],\n}}"
        )
    });

    let empty_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/empty/styles.rs */\n{}",
            ui_components::empty::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<Empty>
  <EmptyHeader>
    <EmptyMedia variant=EmptyMediaVariant::Icon>"📭"</EmptyMedia>
    <EmptyTitle>"No messages"</EmptyTitle>
    <EmptyDescription>"You're all caught up."</EmptyDescription>
  </EmptyHeader>
</Empty>
<Empty class_name="docs-empty-custom".to_string()>
  <EmptyHeader>
    <EmptyTitle>"No deployments"</EmptyTitle>
    <EmptyDescription>"Create your first release to populate this list."</EmptyDescription>
  </EmptyHeader>
  <EmptyContent>
    <a href="#/components/button">"Create deployment"</a>
  </EmptyContent>
</Empty>
<Empty class_name="docs-empty-state".to_string()>
  <EmptyHeader class_name="docs-empty-header".to_string()>
    <EmptyMedia variant=EmptyMediaVariant::Icon class_name="docs-empty-media".to_string()>"📦"</EmptyMedia>
    <EmptyTitle class_name="docs-empty-title".to_string()>"No results"</EmptyTitle>
    <EmptyDescription class_name="docs-empty-description".to_string()>"Try adjusting filters."</EmptyDescription>
  </EmptyHeader>
  <EmptyContent class_name="docs-empty-content".to_string()>
    <a href="#/components/search">"Open search"</a>
  </EmptyContent>
</Empty>"##.to_string()
    });

    view! {
        <ComponentPage
            title="Empty"
            slug="empty"
            group="Display"
            description="baseline-compatible empty-state composition primitives (`Empty*`) with stable slot contracts for header/media/title/description/content layering."
        >
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                description="可调 media/content/class/source，并在同一面板查看 code + config + scoped css test。"
                code_signal=workbench_code
                test_css_source=empty_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/empty/styles.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Media Variant"</div>
                            <SegmentedControl
                                id_base="docs-empty-media-variant".to_string()
                                options=media_options.clone()
                                selected_index=workbench_media_index
                                set_selected_index=set_workbench_media_index
                                size=SegmentedControlSize::Sm
                                aria_label="Empty media variant".to_string()
                            />
                            <Switch checked=workbench_show_content set_checked=set_workbench_show_content>
                                "Show content action"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom root class"
                            </Switch>
                            <Switch
                                checked=workbench_custom_slot_classes
                                set_checked=set_workbench_custom_slot_classes
                            >
                                "Custom slot classes"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let media_variant = workbench_media_variant.get();
                    let show_content = workbench_show_content.get();
                    let custom_class = if workbench_custom_class.get() {
                        "docs-empty-custom".to_string()
                    } else {
                        "".to_string()
                    };
                    let use_custom_slots = workbench_custom_slot_classes.get();

                    view! {
                        <Empty class_name=custom_class>
                            <EmptyHeader class_name=if use_custom_slots {
                                "docs-empty-header".to_string()
                            } else {
                                "".to_string()
                            }>
                                <EmptyMedia
                                    variant=media_variant
                                    class_name=if use_custom_slots {
                                        "docs-empty-media".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                >
                                    "📦"
                                </EmptyMedia>
                                <EmptyTitle class_name=if use_custom_slots {
                                    "docs-empty-title".to_string()
                                } else {
                                    "".to_string()
                                }>
                                    "No results"
                                </EmptyTitle>
                                <EmptyDescription class_name=if use_custom_slots {
                                    "docs-empty-description".to_string()
                                } else {
                                    "".to_string()
                                }>
                                    "Try adjusting filters."
                                </EmptyDescription>
                            </EmptyHeader>

                            <Show when=move || show_content>
                                <EmptyContent class_name=if use_custom_slots {
                                    "docs-empty-content".to_string()
                                } else {
                                    "".to_string()
                                }>
                                    <a href="#/components/search">"Open search"</a>
                                </EmptyContent>
                            </Show>
                        </Empty>
                    }
                }}
            </Playground>

            <Playground
                title="Comparison Matrix (Header / Action / Source Markers)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Header + Icon"</span>
                        <Empty>
                            <EmptyHeader>
                                <EmptyMedia variant=EmptyMediaVariant::Icon>
                                    "📭"
                                </EmptyMedia>
                                <EmptyTitle>"No messages"</EmptyTitle>
                                <EmptyDescription>"You're all caught up."</EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    </div>

                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Content Action"</span>
                        <Empty class_name="docs-empty-custom".to_string()>
                            <EmptyHeader>
                                <EmptyTitle>"No deployments"</EmptyTitle>
                                <EmptyDescription>
                                    "Create your first release to populate this list."
                                </EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent>
                                <a href="#/components/button">"Create deployment"</a>
                            </EmptyContent>
                        </Empty>
                    </div>

                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"State + Source Markers"</span>
                        <Empty class_name="docs-empty-state".to_string()>
                            <EmptyHeader class_name="docs-empty-header".to_string()>
                                <EmptyMedia
                                    variant=EmptyMediaVariant::Icon
                                    class_name="docs-empty-media".to_string()
                                >
                                    "📦"
                                </EmptyMedia>
                                <EmptyTitle class_name="docs-empty-title".to_string()>
                                    "No results"
                                </EmptyTitle>
                                <EmptyDescription class_name="docs-empty-description".to_string()>
                                    "Try adjusting filters."
                                </EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent class_name="docs-empty-content".to_string()>
                                <a href="#/components/search">"Open search"</a>
                            </EmptyContent>
                        </Empty>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
