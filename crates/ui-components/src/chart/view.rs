use crate::active_highlight::attach_active_highlight_motion;
use crate::chart::ChartMotion;
use crate::chart::logic::{self, ChartKind, ChartPoint, ChartStateInput};
use crate::overlay_open;
use leptos::{ev, html, prelude::*};
use std::sync::Arc;

#[component]
pub fn Chart(
    points: Vec<ChartPoint>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] kind: ChartKind,
    #[prop(optional)] active_index: Option<Signal<usize>>,
    #[prop(optional)] default_active_index: Option<usize>,
    #[prop(optional)] on_active_index_change: Option<Callback<usize>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_grid: bool,
    #[prop(optional)] motion: ChartMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_aria_label(aria_label);

    let points: Arc<[ChartPoint]> = logic::normalize_points(points).into();
    let point_count = points.len();
    let domain = logic::value_domain(points.as_ref());

    let default_active_index = logic::default_active_index(point_count, default_active_index);

    let is_controlled = active_index.is_some();
    let active_state = overlay_open::use_controllable_state(
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

    let state = Signal::derive(move || {
        logic::resolve_state(ChartStateInput {
            kind,
            point_count,
            active_index: active_index.get(),
            disabled,
            show_grid,
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let legend_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    let option_id =
        Callback::new(move |index: usize| format!("{}-legend-{index}", id_base.get_value()));
    attach_active_highlight_motion(
        legend_ref,
        highlight_ref,
        active_index_read,
        option_id,
        motion,
    );

    let indices: StoredValue<Vec<usize>> = StoredValue::new((0..point_count).collect());

    let polyline_points = StoredValue::new(logic::polyline_points(
        points.get_value().as_ref(),
        domain.get_value(),
    ));

    let select_point = Callback::new(move |index: usize| {
        if disabled || point_count == 0 {
            return;
        }

        let index = logic::clamp_active_index(index, point_count);
        request_active_index_change.run(index);
    });

    let trigger_action = Callback::new(move |index: usize| {
        if disabled || point_count == 0 {
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
            data-kind=move || state.get().kind_attr
            data-state=move || state.get().state_attr
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-points=move || state.get().has_points.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-show-grid=move || state.get().show_grid.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-active-index=move || state.get().active_index.to_string()
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="region"
            aria-label=aria_label.get_value()
        >
            <div class="ui-chart__plot-wrap" data-slot="chart-plot-wrap">
                <svg
                    class="ui-chart__plot"
                    data-slot="chart-plot"
                    viewBox="0 0 100 56"
                    role="img"
                    aria-label=aria_label.get_value()
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
                                    if disabled {
                                        return;
                                    }

                                    if event.key() == "Enter" || event.key() == " " {
                                        trigger_action.run(index);
                                        event.prevent_default();
                                        return;
                                    }

                                    if let Some(next) = logic::next_index_for_key(
                                        &event.key(),
                                        state.get_untracked().active_index,
                                        point_count,
                                    ) {
                                        select_point.run(next);
                                        event.prevent_default();
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
                                            tabindex=if disabled { -1 } else { 0 }
                                            role="button"
                                            aria-label=format!("{} {:.2}", point.label, point.value)
                                            aria-disabled=disabled.then_some("true")
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
                                            tabindex=if disabled { -1 } else { 0 }
                                            role="button"
                                            aria-label=format!("{} {:.2}", point.label, point.value)
                                            aria-disabled=disabled.then_some("true")
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
                                if disabled {
                                    return;
                                }

                                if event.key() == "Enter" || event.key() == " " {
                                    trigger_action.run(index);
                                    event.prevent_default();
                                    return;
                                }

                                if let Some(next) = logic::next_index_for_key(
                                    &event.key(),
                                    state.get_untracked().active_index,
                                    point_count,
                                ) {
                                    select_point.run(next);
                                    event.prevent_default();
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
                                    disabled=disabled
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
