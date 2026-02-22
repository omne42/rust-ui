use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
    SegmentedControl, SegmentedControlSize, Snippet, Switch,
};

pub(super) fn empty() -> AnyView {
    let empty_code_imports =
        "use leptos::prelude::*;\nuse ui::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};".to_string();
    let media_options = vec!["default".to_string(), "icon".to_string()];
    let (workbench_media_index, set_workbench_media_index) = signal(Some(0));
    let (workbench_show_content, set_workbench_show_content) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_slot_classes, set_workbench_custom_slot_classes) = signal(false);

    let workbench_media_variant =
        Signal::derive(move || match workbench_media_index.get().unwrap_or(0) {
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
            "EmptyActualConfig {{\n  media_variant: {media_variant:?},\n  show_content: {show_content},\n  custom_root_class: {custom_class},\n  custom_slot_classes: {custom_slot_classes},\n  marker_expectations: [\"data-slot\", \"data-state\", \"data-class-source\", \"data-variant\", \"data-variant-source\"],\n}}\nEmptyAgentSpecInput {{\n  stream_mode: Snapshot,\n  stream_support: Optional,\n  stream_fallback: Snapshot,\n  output_status: Verified,\n  media_variant: {media_variant:?},\n  show_content: {show_content},\n}}\nPreviewLinkage {{\n  preview_selector: \"[data-slot=empty][data-state=root]\",\n  preview_action: \"render-snapshot\",\n}}"
        )
    });

    let empty_test_css_source = Signal::derive(move || {
        format!(
            "/* components/empty/src/styles.rs */\n{}",
            ui::empty::styles::CSS
        )
    });

    let hello_code = Signal::derive(move || {
        r##"<Empty>
  <EmptyHeader>
    <EmptyTitle>"No results"</EmptyTitle>
  </EmptyHeader>
</Empty>"##
            .to_string()
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

    let parameter_matrix_code = Signal::derive(move || {
        r##"<Empty>
  <EmptyHeader>
    <EmptyMedia>"📦"</EmptyMedia>
    <EmptyTitle>"Default variant"</EmptyTitle>
    <EmptyDescription>"Uses EmptyMediaVariant::Default via logic fallback."</EmptyDescription>
  </EmptyHeader>
</Empty>
<Empty>
  <EmptyHeader>
    <EmptyMedia variant=EmptyMediaVariant::Icon>"📭"</EmptyMedia>
    <EmptyTitle>"Icon variant"</EmptyTitle>
    <EmptyDescription>"Explicit variant parameter path."</EmptyDescription>
  </EmptyHeader>
</Empty>
<Empty class_name="docs-empty-custom".to_string()>
  <EmptyHeader>
    <EmptyTitle>"Custom class_name"</EmptyTitle>
    <EmptyDescription>"Root class source switches to custom."</EmptyDescription>
  </EmptyHeader>
  <EmptyContent>
    <a href="#/components/search">"Open search"</a>
  </EmptyContent>
</Empty>"##
            .to_string()
    });

    let control_mode_code = Signal::derive(move || {
        r##"// Empty is display-only and currently has no controlled state axis.
// The controlled/uncontrolled checklist item is documented as N/A.
<Empty>
  <EmptyHeader>
    <EmptyTitle>"Controlled: N/A"</EmptyTitle>
    <EmptyDescription>"No value/on_value_change/default_value props on Empty*."</EmptyDescription>
  </EmptyHeader>
</Empty>
<Empty>
  <EmptyHeader>
    <EmptyTitle>"Uncontrolled: N/A"</EmptyTitle>
    <EmptyDescription>"Static composition surface; no internal mutable state axis."</EmptyDescription>
  </EmptyHeader>
</Empty>"##
            .to_string()
    });

    let snapshot_streaming_code = Signal::derive(move || {
        r##"<Empty class_name="docs-empty-streaming".to_string()>
  <EmptyHeader>
    <EmptyMedia variant=EmptyMediaVariant::Icon>"📡"</EmptyMedia>
    <EmptyTitle>"Snapshot baseline"</EmptyTitle>
    <EmptyDescription>"Streaming Optional; fallback=snapshot."</EmptyDescription>
  </EmptyHeader>
  <EmptyContent>
    <span class="ui-muted">"Inspect data-ui-stream-support / data-ui-stream-fallback on root."</span>
  </EmptyContent>
</Empty>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"use leptos::prelude::*;
use ui::{Empty, EmptyHeader, EmptyTitle};

<Empty>
  <EmptyHeader>
    <EmptyTitle>"No results"</EmptyTitle>
  </EmptyHeader>
</Empty>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="Empty"
            slug="empty"
            group="Display"
            description="baseline-compatible empty-state composition primitives (`Empty*`) with stable slot contracts for header/media/title/description/content layering."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=empty_code_imports.clone()
                description="最小可用路径：不需要手动接线状态原语，直接组合默认结构即可运行。"
            >
                <Empty>
                    <EmptyHeader>
                        <EmptyTitle>"No results"</EmptyTitle>
                    </EmptyHeader>
                </Empty>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                description="可在线调 media/content/class/source 并实时预览；同面板提供 code + config + AI Spec 输入联动 + scoped css test，作为可重复验收面。"
                code_signal=workbench_code
                code_imports=empty_code_imports.clone()
                test_css_source=empty_test_css_source
                test_source_path="components/empty/src/styles.rs".to_string()
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
                title="Parameter Matrix (variant / class_name / content)"
                code_signal=parameter_matrix_code
                code_imports=empty_code_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Default variant"</span>
                        <Empty>
                            <EmptyHeader>
                                <EmptyMedia>"📦"</EmptyMedia>
                                <EmptyTitle>"Default variant"</EmptyTitle>
                                <EmptyDescription>
                                    "Uses logic fallback for EmptyMediaVariant::Default."
                                </EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    </div>

                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Icon variant"</span>
                        <Empty>
                            <EmptyHeader>
                                <EmptyMedia variant=EmptyMediaVariant::Icon>
                                    "📭"
                                </EmptyMedia>
                                <EmptyTitle>"Icon variant"</EmptyTitle>
                                <EmptyDescription>
                                    "Explicit variant parameter path."
                                </EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    </div>

                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Custom class_name + content"</span>
                        <Empty class_name="docs-empty-custom".to_string()>
                            <EmptyHeader>
                                <EmptyTitle>"Custom class_name"</EmptyTitle>
                                <EmptyDescription>
                                    "Root class source switches to custom."
                                </EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent>
                                <a href="#/components/search">"Open search"</a>
                            </EmptyContent>
                        </Empty>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Header / Action / Source Markers)"
                code_signal=matrix_code
                code_imports=empty_code_imports.clone()
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

            <Playground
                title="Controlled vs Uncontrolled (N/A for Empty)"
                code_signal=control_mode_code
                code_imports=empty_code_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-card">
                        <span class="ui-muted">"Controlled"</span>
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"N/A"</EmptyTitle>
                                <EmptyDescription>
                                    "Empty has no value/on_value_change/default_value state axis."
                                </EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    </div>

                    <div class="docs-card">
                        <span class="ui-muted">"Uncontrolled"</span>
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"N/A"</EmptyTitle>
                                <EmptyDescription>
                                    "Rendering is static composition; no internal mutable state."
                                </EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="Empty 不是正文阅读面：Streaming Optional，且 fallback=snapshot。"
                code_signal=snapshot_streaming_code
                code_imports=empty_code_imports
            >
                <div class="docs-stack">
                    <Empty class_name="docs-empty-streaming".to_string()>
                        <EmptyHeader>
                            <EmptyMedia variant=EmptyMediaVariant::Icon>"📡"</EmptyMedia>
                            <EmptyTitle>"Snapshot baseline"</EmptyTitle>
                            <EmptyDescription>
                                "Streaming Optional; fallback=snapshot."
                            </EmptyDescription>
                        </EmptyHeader>
                        <EmptyContent>
                            <span class="ui-muted">
                                "Inspect data-ui-stream-support / data-ui-stream-fallback on the root."
                            </span>
                        </EmptyContent>
                    </Empty>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="empty-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="empty-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-empty"</code>
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
                    text=source_first_code.get()
                    label="Copy empty starter".to_string()
                    copyable=true
                    class_name="docs-empty-source-copy".to_string()
                />
                <ul data-slot="empty-source-paths">
                    <li><code>"components/empty/src/mod.rs"</code></li>
                    <li><code>"components/empty/src/logic.rs"</code></li>
                    <li><code>"components/empty/src/view.rs"</code></li>
                    <li><code>"components/empty/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
