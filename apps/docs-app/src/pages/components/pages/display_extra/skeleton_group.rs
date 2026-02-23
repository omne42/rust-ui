use super::*;

pub(crate) fn skeleton_group() -> AnyView {
    let variant_options = [
        "Shimmer".to_string(),
        "Pulse".to_string(),
        "None".to_string(),
    ];
    let layout_options = ["Vertical".to_string(), "Horizontal".to_string()];
    let density_options = ["Comfortable".to_string(), "Compact".to_string()];

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_layout_index, set_workbench_layout_index) = signal(Some(0_usize));
    let (workbench_density_index, set_workbench_density_index) = signal(Some(0_usize));
    let (workbench_is_loading, set_workbench_is_loading) = signal(true);
    let (workbench_is_skeleton_only, set_workbench_is_skeleton_only) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => SkeletonGroupVariant::Pulse,
            2 => SkeletonGroupVariant::None,
            _ => SkeletonGroupVariant::Shimmer,
        });
    let workbench_layout =
        Signal::derive(move || match workbench_layout_index.get().unwrap_or(0) {
            1 => SkeletonGroupLayout::Horizontal,
            _ => SkeletonGroupLayout::Vertical,
        });
    let workbench_density =
        Signal::derive(move || match workbench_density_index.get().unwrap_or(0) {
            1 => SkeletonGroupDensity::Compact,
            _ => SkeletonGroupDensity::Comfortable,
        });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench skeleton group".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-skeleton-group-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<SkeletonGroup is_loading=true variant=SkeletonGroupVariant::Shimmer layout=SkeletonGroupLayout::Vertical density=SkeletonGroupDensity::Comfortable>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
</SkeletonGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant_expr = match workbench_variant.get() {
            SkeletonGroupVariant::Shimmer => "SkeletonGroupVariant::Shimmer",
            SkeletonGroupVariant::Pulse => "SkeletonGroupVariant::Pulse",
            SkeletonGroupVariant::None => "SkeletonGroupVariant::None",
        };
        let layout_expr = match workbench_layout.get() {
            SkeletonGroupLayout::Vertical => "SkeletonGroupLayout::Vertical",
            SkeletonGroupLayout::Horizontal => "SkeletonGroupLayout::Horizontal",
        };
        let density_expr = match workbench_density.get() {
            SkeletonGroupDensity::Comfortable => "SkeletonGroupDensity::Comfortable",
            SkeletonGroupDensity::Compact => "SkeletonGroupDensity::Compact",
        };

        format!(
            "<SkeletonGroup\n  is_loading={}\n  is_skeleton_only={}\n  variant={variant_expr}\n  layout={layout_expr}\n  density={density_expr}\n  aria_label={}\n  class_name={}\n>\n  <Skeleton variant=SkeletonVariant::Rect class_name=\"docs-skeleton-line\".to_string() />\n  <Skeleton variant=SkeletonVariant::Rect class_name=\"docs-skeleton-line docs-skeleton-line--short\".to_string() />\n</SkeletonGroup>",
            bool_word(workbench_is_loading.get()),
            bool_word(workbench_is_skeleton_only.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SkeletonGroupActualConfig {{\n  is_loading: {},\n  is_skeleton_only: {},\n  variant: {:?},\n  layout: {:?},\n  density: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_is_loading.get(),
            workbench_is_skeleton_only.get(),
            workbench_variant.get(),
            workbench_layout.get(),
            workbench_density.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SkeletonGroup
  is_loading=true
  is_skeleton_only=false
  variant=SkeletonGroupVariant::Shimmer
  layout=SkeletonGroupLayout::Vertical
  density=SkeletonGroupDensity::Comfortable
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>
<SkeletonGroup
  is_loading=true
  is_skeleton_only=false
  variant=SkeletonGroupVariant::Pulse
  layout=SkeletonGroupLayout::Horizontal
  density=SkeletonGroupDensity::Compact
  aria_label="Profile placeholders".to_string()
  class_name="docs-skeleton-group-custom".to_string()
>
  <Skeleton variant=SkeletonVariant::Circle is_shimmer=false class_name="docs-skeleton-avatar".to_string() />
  <Skeleton variant=SkeletonVariant::Rect is_shimmer=false class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>
<SkeletonGroup
  is_loading=false
  is_skeleton_only=true
  variant=SkeletonGroupVariant::None
  layout=SkeletonGroupLayout::Vertical
  density=SkeletonGroupDensity::Comfortable
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SkeletonGroup"
            slug="skeleton-group"
            group="Display"
            description="baseline-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{Skeleton, SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant, SkeletonVariant};".to_string()
                test_source_path="crates/ui/src/skeleton/group/view.rs".to_string()
            >
                <SkeletonGroup
                    is_loading=true
                    variant=SkeletonGroupVariant::Shimmer
                    layout=SkeletonGroupLayout::Vertical
                    density=SkeletonGroupDensity::Comfortable
                >
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        class_name="docs-skeleton-line".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                    />
                </SkeletonGroup>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{Skeleton, SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant, SkeletonVariant};".to_string()
                test_source_path="crates/ui/src/skeleton/group/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="skeleton-group-workbench-controls">
                        <div class="docs-search__label">"variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_variant_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_variant_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {variant_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"layout"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_layout_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_layout_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {layout_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"density"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_density_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_density_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {density_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_loading.get()
                                on:change=move |event| set_workbench_is_loading.set(event_target_checked(&event))
                            />
                            <span>"is_loading"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_skeleton_only.get()
                                on:change=move |event| set_workbench_is_skeleton_only.set(event_target_checked(&event))
                            />
                            <span>"is_skeleton_only"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <SkeletonGroup
                        is_loading=workbench_is_loading.get()
                        is_skeleton_only=workbench_is_skeleton_only.get()
                        variant=workbench_variant.get()
                        layout=workbench_layout.get()
                        density=workbench_density.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>
                    <span class="ui-muted">
                        "state: loading="
                        {move || workbench_is_loading.get().to_string()}
                        ", skeleton_only="
                        {move || workbench_is_skeleton_only.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Loading / Layout / Hidden Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Skeleton, SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant, SkeletonVariant};".to_string()
                test_source_path="crates/ui/src/skeleton/group/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <SkeletonGroup
                        is_loading=true
                        is_skeleton_only=false
                        variant=SkeletonGroupVariant::Shimmer
                        layout=SkeletonGroupLayout::Vertical
                        density=SkeletonGroupDensity::Comfortable
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                    </SkeletonGroup>
                    <SkeletonGroup
                        is_loading=true
                        is_skeleton_only=false
                        variant=SkeletonGroupVariant::Pulse
                        layout=SkeletonGroupLayout::Horizontal
                        density=SkeletonGroupDensity::Compact
                        aria_label="Profile placeholders".to_string()
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Circle
                            is_shimmer=false
                            class_name="docs-skeleton-avatar".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            is_shimmer=false
                            class_name="docs-skeleton-line".to_string()
                        />
                    </SkeletonGroup>
                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=true
                        variant=SkeletonGroupVariant::None
                        layout=SkeletonGroupLayout::Vertical
                        density=SkeletonGroupDensity::Comfortable
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                    </SkeletonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
