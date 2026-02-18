use crate::chart::logic::{self, ChartKind, ChartPoint, ChartStateInput};
use crate::chart::motion;
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{A11yDirection, ChartKeyAction, ChartOptions, use_chart, use_controllable_state};

#[component]
pub fn Chart(
    points: Vec<ChartPoint>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] kind: ChartKind,
    #[prop(optional)] active_index: Option<Signal<usize>>,
    #[prop(optional)] default_active_index: Option<usize>,
    #[prop(optional)] on_active_index_change: Option<Callback<usize>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_grid: bool,
    #[prop(optional)] motion: motion::ChartMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_aria_label(aria_label);
    let resolved_disabled = is_disabled.unwrap_or(disabled);

    let points: Arc<[ChartPoint]> = logic::normalize_points(points).into();
    let point_count = points.len();
    let domain = logic::value_domain(points.as_ref());

    let default_active_index = logic::default_active_index(point_count, default_active_index);

    let is_controlled = active_index.is_some();
    let active_state = use_controllable_state(
        active_index,
        Some(default_active_index),
        on_active_index_change,
    );
    let active_index = active_state.value;
    let request_active_index_change = active_state.request_change;

    let (active_index_read, set_active_index_read) = signal(active_index.get_untracked());
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
        logic::resolve_state(ChartStateInput {
            kind,
            point_count,
            active_index: active_index.get(),
            disabled: resolved_disabled,
            show_grid,
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
    let motion_source = if motion == motion::ChartMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != motion::ChartMotion::default()).then_some("true");

    let indices: StoredValue<Vec<usize>> = StoredValue::new((0..point_count).collect());

    let polyline_points = StoredValue::new(logic::polyline_points(
        points.get_value().as_ref(),
        domain.get_value(),
    ));

    let select_point = Callback::new(move |index: usize| {
        if resolved_disabled || point_count == 0 {
            return;
        }

        let index = logic::clamp_active_index(index, point_count);
        request_active_index_change.run(index);
    });

    let trigger_action = Callback::new(move |index: usize| {
        if resolved_disabled || point_count == 0 {
            return;
        }

        let index = logic::clamp_active_index(index, point_count);
        request_active_index_change.run(index);

        if let Some(callback) = on_action.get_value()
            && let Some(point) = points.get_value().get(index)
        {
            callback.run(point.id.clone());
        }
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
            data-class-source=move || semantics.get().attrs.data_class_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=move || semantics.get().attrs.role
            aria-label=move || semantics.get().attrs.aria_label
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
        >
            <div class="ui-chart__plot-wrap" data-slot="chart-plot-wrap">
                <svg
                    class="ui-chart__plot"
                    data-slot="chart-plot"
                    viewBox="0 0 100 56"
                    role="img"
                    aria-label=move || semantics.get().attrs.aria_label
                >
                    <Show when=move || state.get().show_grid>
                        <g class="ui-chart__grid" data-slot="chart-grid">
                            <line class="ui-chart__grid-line" x1="8" y1="8" x2="92" y2="8"></line>
                            <line class="ui-chart__grid-line" x1="8" y1="22" x2="92" y2="22"></line>
                            <line class="ui-chart__grid-line" x1="8" y1="36" x2="92" y2="36"></line>
                            <line class="ui-chart__grid-line" x1="8" y1="52" x2="92" y2="52"></line>
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
                                    select_point.run(index);
                                };

                                let on_click = move |_| {
                                    trigger_action.run(index);
                                };

                                let on_key_down = move |event: ev::KeyboardEvent| {
                                    match semantics.get_untracked().handlers.on_key_down(
                                        &event.key(),
                                        index,
                                        point_count,
                                        resolved_disabled,
                                    ) {
                                        ChartKeyAction::Noop => {}
                                        ChartKeyAction::MoveTo(next) => {
                                            select_point.run(next);
                                            event.prevent_default();
                                        }
                                        ChartKeyAction::Activate(current) => {
                                            trigger_action.run(current);
                                            event.prevent_default();
                                        }
                                    }
                                };

                                if kind == ChartKind::Bar {
                                    let rect_x = x - bar_width / 2.0;
                                    let rect_h = (52.0 - y).max(0.5);
                                    view! {
                                        <rect
                                            class="ui-chart__bar"
                                            data-slot="chart-bar"
                                            data-index=index.to_string()
                                            data-active=move || is_active().then_some("true")
                                            id=move || format!("{}-plot-{index}", id_base.get_value())
                                            x=rect_x
                                            y=y
                                            width=bar_width
                                            height=rect_h
                                            tabindex=if resolved_disabled { -1 } else { 0 }
                                            role="button"
                                            aria-label=format!("{} {:.2}", point.label, point.value)
                                            aria-disabled=resolved_disabled.then_some("true")
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
                                            data-index=index.to_string()
                                            data-active=move || is_active().then_some("true")
                                            id=move || format!("{}-plot-{index}", id_base.get_value())
                                            cx=x
                                            cy=y
                                            r=move || if is_active() { 2.8 } else { 2.0 }
                                            tabindex=if resolved_disabled { -1 } else { 0 }
                                            role="button"
                                            aria-label=format!("{} {:.2}", point.label, point.value)
                                            aria-disabled=resolved_disabled.then_some("true")
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
                                select_point.run(index);
                            };

                            let on_pointer_enter = move |_: ev::PointerEvent| {
                                select_point.run(index);
                            };

                            let on_click = move |_| {
                                trigger_action.run(index);
                            };

                            let on_key_down = move |event: ev::KeyboardEvent| {
                                match semantics.get_untracked().handlers.on_key_down(
                                    &event.key(),
                                    index,
                                    point_count,
                                    resolved_disabled,
                                ) {
                                    ChartKeyAction::Noop => {}
                                    ChartKeyAction::MoveTo(next) => {
                                        select_point.run(next);
                                        event.prevent_default();
                                    }
                                    ChartKeyAction::Activate(current) => {
                                        trigger_action.run(current);
                                        event.prevent_default();
                                    }
                                }
                            };

                            view! {
                                <button
                                    class="ui-chart__legend-item"
                                    data-slot="chart-legend-item"
                                    data-index=index.to_string()
                                    data-active=move || (state.get().active_index == index).then_some("true")
                                    id=move || format!("{}-legend-{index}", id_base.get_value())
                                    type="button"
                                    disabled=resolved_disabled
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
        </section>
    }
}
