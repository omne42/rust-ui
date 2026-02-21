use crate::logic::{self, ChartKind, ChartPoint};
use crate::motion;
use leptos::{ev, html, prelude::*};
use ui_headless::{
    A11yDirection, ChartContract, ChartKeyAction, ChartOptions, CommonStrings, use_chart,
    use_controllable_state, use_ui_i18n, use_ui_id_provider,
};

type ChartPoints = std::sync::Arc<[ChartPoint]>;
type ChartStateSignal = Signal<logic::ChartState>;
type ChartSemanticsSignal = Signal<ChartContract>;
const CHART_PLOT_VIEWBOX: &str = "0 0 100 56";
const CHART_GRID_LINE_CLASS: &str = "ui-chart__grid-line";
const CHART_GRID_LINES: [(&str, &str, &str, &str); 4] = [
    ("8", "8", "92", "8"),
    ("8", "22", "92", "22"),
    ("8", "36", "92", "36"),
    ("8", "52", "92", "52"),
];

struct ChartPlotRenderInput {
    kind: ChartKind,
    point_count: usize,
    is_disabled: bool,
    id_base: StoredValue<String>,
    points: StoredValue<ChartPoints>,
    domain: StoredValue<logic::ChartDomain>,
    indices: StoredValue<Vec<usize>>,
    state: ChartStateSignal,
    semantics: ChartSemanticsSignal,
    apply_headless_action: Callback<ChartKeyAction>,
    set_active_interaction_source: WriteSignal<logic::ChartInteractionSource>,
    polyline_points: StoredValue<String>,
}

struct ChartLegendRenderInput {
    legend_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    point_count: usize,
    is_disabled: bool,
    id_base: StoredValue<String>,
    points: StoredValue<ChartPoints>,
    indices: StoredValue<Vec<usize>>,
    state: ChartStateSignal,
    semantics: ChartSemanticsSignal,
    apply_headless_action: Callback<ChartKeyAction>,
    set_active_interaction_source: WriteSignal<logic::ChartInteractionSource>,
}

fn render_chart_grid_lines() -> impl IntoView {
    CHART_GRID_LINES
        .into_iter()
        .map(|(x1, y1, x2, y2)| {
            view! {
                <line class=CHART_GRID_LINE_CLASS x1=x1 y1=y1 x2=x2 y2=y2></line>
            }
        })
        .collect_view()
}

fn render_chart_plot(input: ChartPlotRenderInput) -> impl IntoView {
    let ChartPlotRenderInput {
        kind,
        point_count,
        is_disabled,
        id_base,
        points,
        domain,
        indices,
        state,
        semantics,
        apply_headless_action,
        set_active_interaction_source,
        polyline_points,
    } = input;

    view! {
        <div class="ui-chart__plot-wrap" data-slot="chart-plot-wrap">
            <svg
                class="ui-chart__plot"
                data-slot="chart-plot"
                viewBox=CHART_PLOT_VIEWBOX
                role="img"
                aria-label=move || semantics.get().attrs.aria_label
            >
                <Show when=move || state.get().show_grid>
                    <g class="ui-chart__grid" data-slot="chart-grid">
                        {render_chart_grid_lines()}
                    </g>
                </Show>

                <Show when=move || state.get().kind == ChartKind::Line>
                    <polyline
                        class="ui-chart__line"
                        data-slot="chart-line"
                        points=polyline_points.get_value()
                    ></polyline>
                </Show>

                {move || {
                    let bar_width = logic::bar_width(point_count);
                    indices
                        .get_value()
                        .iter()
                        .copied()
                        .map(|index| {
                            let point = points.get_value()[index].clone();
                            let x = logic::point_x(index, point_count);
                            let y = logic::point_y(point.value, domain.get_value());
                            let is_active = move || state.get().active_index == index;

                            let on_enter = move |_| {
                                let action = semantics
                                    .get_untracked()
                                    .handlers
                                    .on_pointer_enter(index, point_count, is_disabled);
                                if !matches!(action, ChartKeyAction::Noop) {
                                    set_active_interaction_source
                                        .set(logic::ChartInteractionSource::Pointer);
                                }
                                apply_headless_action.run(action);
                            };

                            let on_click = move |_| {
                                let action = semantics
                                    .get_untracked()
                                    .handlers
                                    .on_click(index, point_count, is_disabled);
                                if !matches!(action, ChartKeyAction::Noop) {
                                    set_active_interaction_source
                                        .set(logic::ChartInteractionSource::Pointer);
                                }
                                apply_headless_action.run(action);
                            };

                            let on_key_down = move |event: ev::KeyboardEvent| {
                                let action = semantics.get_untracked().handlers.on_key_down(
                                    &event.key(),
                                    index,
                                    point_count,
                                    is_disabled,
                                );
                                if !matches!(action, ChartKeyAction::Noop) {
                                    set_active_interaction_source
                                        .set(logic::ChartInteractionSource::Keyboard);
                                    event.prevent_default();
                                }
                                apply_headless_action.run(action);
                            };

                            let point_aria_label = semantics
                                .get_untracked()
                                .handlers
                                .point_aria_label(&point.label, point.value);

                            if kind == ChartKind::Bar {
                                let rect_x = x - bar_width / 2.0;
                                let rect_h = (52.0 - y).max(0.5);
                                view! {
                                    <rect
                                        class="ui-chart__bar"
                                        data-slot="chart-bar"
                                        data-index=index
                                        data-active=move || is_active().then_some("true")
                                        id=move || format!("{}-plot-{index}", id_base.get_value())
                                        x=rect_x
                                        y=y
                                        width=bar_width
                                        height=rect_h
                                        tabindex=if is_disabled { -1 } else { 0 }
                                        role="button"
                                        aria-label=point_aria_label.clone()
                                        aria-disabled=is_disabled.then_some("true")
                                        on:pointerenter=on_enter
                                        on:click=on_click
                                        on:keydown=on_key_down
                                    ></rect>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <circle
                                        class="ui-chart__dot"
                                        data-slot="chart-dot"
                                        data-index=index
                                        data-active=move || is_active().then_some("true")
                                        id=move || format!("{}-plot-{index}", id_base.get_value())
                                        cx=x
                                        cy=y
                                        r=move || if is_active() { 2.8 } else { 2.0 }
                                        tabindex=if is_disabled { -1 } else { 0 }
                                        role="button"
                                        aria-label=point_aria_label.clone()
                                        aria-disabled=is_disabled.then_some("true")
                                        on:pointerenter=on_enter
                                        on:click=on_click
                                        on:keydown=on_key_down
                                    ></circle>
                                }
                                .into_any()
                            }
                        })
                        .collect_view()
                }}
            </svg>
        </div>
    }
}

fn render_chart_legend(input: ChartLegendRenderInput) -> impl IntoView {
    let ChartLegendRenderInput {
        legend_ref,
        highlight_ref,
        point_count,
        is_disabled,
        id_base,
        points,
        indices,
        state,
        semantics,
        apply_headless_action,
        set_active_interaction_source,
    } = input;

    view! {
        <div class="ui-chart__legend" data-slot="chart-legend" node_ref=legend_ref>
            <div class="ui-chart__legend-highlight" data-slot="chart-legend-highlight" node_ref=highlight_ref></div>
            {move || {
                indices
                    .get_value()
                    .iter()
                    .copied()
                    .map(|index| {
                        let point = points.get_value()[index].clone();
                        let point_id = StoredValue::new(point.id);
                        let point_label = StoredValue::new(point.label);
                        let point_value = StoredValue::new(point.value);

                        let on_focus = move |_: ev::FocusEvent| {
                            let action = semantics
                                .get_untracked()
                                .handlers
                                .on_focus(index, point_count, is_disabled);
                            if !matches!(action, ChartKeyAction::Noop) {
                                set_active_interaction_source
                                    .set(logic::ChartInteractionSource::Focus);
                            }
                            apply_headless_action.run(action);
                        };

                        let on_pointer_enter = move |_: ev::PointerEvent| {
                            let action = semantics
                                .get_untracked()
                                .handlers
                                .on_pointer_enter(index, point_count, is_disabled);
                            if !matches!(action, ChartKeyAction::Noop) {
                                set_active_interaction_source
                                    .set(logic::ChartInteractionSource::Pointer);
                            }
                            apply_headless_action.run(action);
                        };

                        let on_click = move |_| {
                            let action = semantics
                                .get_untracked()
                                .handlers
                                .on_click(index, point_count, is_disabled);
                            if !matches!(action, ChartKeyAction::Noop) {
                                set_active_interaction_source
                                    .set(logic::ChartInteractionSource::Pointer);
                            }
                            apply_headless_action.run(action);
                        };

                        let on_key_down = move |event: ev::KeyboardEvent| {
                            let action = semantics.get_untracked().handlers.on_key_down(
                                &event.key(),
                                index,
                                point_count,
                                is_disabled,
                            );
                            if !matches!(action, ChartKeyAction::Noop) {
                                set_active_interaction_source
                                    .set(logic::ChartInteractionSource::Keyboard);
                                event.prevent_default();
                            }
                            apply_headless_action.run(action);
                        };

                        view! {
                            <button
                                class="ui-chart__legend-item"
                                data-slot="chart-legend-item"
                                data-index=index
                                data-active=move || (state.get().active_index == index).then_some("true")
                                id=move || format!("{}-legend-{index}", id_base.get_value())
                                type="button"
                                disabled=is_disabled
                                aria-pressed=move || {
                                    if state.get().active_index == index {
                                        "true"
                                    } else {
                                        "false"
                                    }
                                }
                                on:focus=on_focus
                                on:pointerenter=on_pointer_enter
                                on:click=on_click
                                on:keydown=on_key_down
                            >
                                <span class="ui-chart__legend-label" data-slot="chart-legend-label">
                                    {point_label.get_value()}
                                </span>
                                <span class="ui-chart__legend-value" data-slot="chart-legend-value">
                                    {format!("{:.2}", point_value.get_value())}
                                </span>
                                <span class="ui-sr-only">{point_id.get_value()}</span>
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
pub fn Chart(
    points: Vec<ChartPoint>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] kind: ChartKind,
    #[prop(optional)] active_index: Option<Signal<usize>>,
    #[prop(optional)] default_active_index: Option<usize>,
    #[prop(optional)] on_active_index_change: Option<Callback<usize>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, default = true)] is_show_grid: bool,
    #[prop(optional)] motion: motion::ChartMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let aria_label = logic::resolve_aria_label_with_fallback(
        aria_label,
        Some(common.chart_aria_label.as_ref().into()),
    );
    let generated_id_base = use_ui_id_provider()
        .map(|id_provider| id_provider.next_prefixed_id(logic::DEFAULT_ID_BASE))
        .unwrap_or_else(|| logic::DEFAULT_ID_BASE.to_string());
    let id_base = logic::resolve_id_base(id_base, generated_id_base);

    let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {
        id_base: Some(id_base),
        class_name,
        aria_label: Some(aria_label),
        points,
        default_active_index,
    });
    let id_base = normalized.id_base;
    let class_name = normalized.class_name;
    let aria_label = normalized.aria_label;
    let points = normalized.points;
    let point_count = normalized.point_count;
    let domain = normalized.domain;
    let default_active_index = normalized.default_active_index;

    let is_controlled = active_index.is_some();
    let active_state = use_controllable_state(
        active_index,
        Some(default_active_index),
        on_active_index_change,
    );
    let active_index = active_state.value;
    let request_active_index_change = active_state.request_change;

    let (active_index_read, set_active_index_read) = signal(active_index.get_untracked());
    let (active_value_source, set_active_value_source) =
        signal(logic::initial_active_value_source(is_controlled));
    let (active_interaction_source, set_active_interaction_source) =
        signal(logic::ChartInteractionSource::None);
    Effect::new(move |_| {
        set_active_index_read.set(active_index.get());
    });

    let id_base = StoredValue::new(id_base);
    let aria_label = StoredValue::new(aria_label);
    let class_name = StoredValue::new(class_name);
    let points = StoredValue::new(points);
    let domain = StoredValue::new(domain);
    let on_action = StoredValue::new(on_action);
    let lang = StoredValue::new(lang);
    let dir = StoredValue::new(dir);

    let state = Signal::derive(move || {
        logic::derive_state_from_boundary(logic::ChartStateBoundary {
            kind,
            point_count,
            active_index: active_index.get(),
            is_disabled,
            is_show_grid,
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let semantics = Signal::derive(move || {
        use_chart(ChartOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir: dir.get_value(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let legend_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    let option_id =
        Callback::new(move |index: usize| format!("{}-legend-{index}", id_base.get_value()));
    let motion = motion::sanitize_motion(motion);
    motion::attach_motion(
        legend_ref,
        highlight_ref,
        active_index_read,
        option_id,
        motion,
    );
    let has_custom_motion = motion != motion::ChartMotion::default();
    let motion_source = if has_custom_motion {
        "custom"
    } else {
        "default"
    };
    let custom_motion = has_custom_motion.then_some("true");
    let agent_motion_source = logic::resolve_motion_source(has_custom_motion);
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::ChartAgentContractInput {
            state: state.get(),
            active_value_source: active_value_source.get(),
            interaction_source: active_interaction_source.get(),
            motion_source: agent_motion_source,
        })
    });

    let indices: StoredValue<Vec<usize>> = StoredValue::new((0..point_count).collect());

    let polyline_points = StoredValue::new(logic::polyline_points(
        points.get_value().as_ref(),
        domain.get_value(),
    ));

    let select_point = Callback::new(move |index: usize| {
        if let Some(index) = logic::normalize_interaction_index(index, point_count, is_disabled) {
            set_active_value_source.set(logic::interaction_active_value_source(is_controlled));
            request_active_index_change.run(index);
        }
    });

    let trigger_action = Callback::new(move |index: usize| {
        if let Some(index) = logic::normalize_interaction_index(index, point_count, is_disabled) {
            set_active_value_source.set(logic::interaction_active_value_source(is_controlled));
            request_active_index_change.run(index);

            if let Some(callback) = on_action.get_value()
                && let Some(point) = points.get_value().get(index)
            {
                callback.run(point.id.clone());
            }
        }
    });

    let apply_headless_action = Callback::new(move |action: ChartKeyAction| match action {
        ChartKeyAction::Noop => {}
        ChartKeyAction::MoveTo(next) => {
            select_point.run(next);
        }
        ChartKeyAction::Activate(current) => {
            trigger_action.run(current);
        }
    });

    let plot = render_chart_plot(ChartPlotRenderInput {
        kind,
        point_count,
        is_disabled,
        id_base,
        points,
        domain,
        indices,
        state,
        semantics,
        apply_headless_action,
        set_active_interaction_source,
        polyline_points,
    });

    let legend = render_chart_legend(ChartLegendRenderInput {
        legend_ref,
        highlight_ref,
        point_count,
        is_disabled,
        id_base,
        points,
        indices,
        state,
        semantics,
        apply_headless_action,
        set_active_interaction_source,
    });

    view! {
        <section
            class=move || class.get()
            data-slot="chart"
            data-kind=move || semantics.get().attrs.data_kind
            data-state=move || semantics.get().attrs.data_state
            data-empty=move || semantics.get().attrs.data_empty
            data-has-points=move || semantics.get().attrs.data_has_points
            data-disabled=move || semantics.get().attrs.data_disabled
            data-enabled=move || semantics.get().attrs.data_enabled
            data-show-grid=move || semantics.get().attrs.data_show_grid
            data-controlled=move || semantics.get().attrs.data_controlled
            data-uncontrolled=move || semantics.get().attrs.data_uncontrolled
            data-active-index=move || state.get().active_index.to_string()
            data-active-value-source=move || active_value_source.get().as_attr()
            data-active-interaction-source=move || active_interaction_source.get().as_attr()
            data-class-source=move || semantics.get().attrs.data_class_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-kind=move || agent_contract.get().kind.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-active-value-source=move || agent_contract.get().active_value_source.as_attr()
            data-ui-interaction-source=move || agent_contract.get().interaction_source.as_attr()
            data-ui-motion-source=move || agent_contract.get().motion_source.as_attr()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy
            role=move || semantics.get().attrs.role
            aria-label=move || semantics.get().attrs.aria_label
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
        >
            {plot}
            {legend}
        </section>
    }
}
