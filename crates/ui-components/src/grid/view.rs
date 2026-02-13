use crate::grid::{
    GridStateInput,
    logic::{self, GridAlign, GridColumns, GridGap, GridJustify, GridRows},
};
use leptos::prelude::*;

#[component]
pub fn Grid(
    #[prop(optional)] columns: GridColumns,
    #[prop(optional)] rows: GridRows,
    #[prop(optional)] gap: GridGap,
    #[prop(optional)] justify: GridJustify,
    #[prop(optional)] align: GridAlign,
    #[prop(optional)] dense: bool,
    #[prop(optional)] inline: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(GridStateInput {
            columns,
            rows,
            gap,
            justify,
            align,
            dense,
            inline,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="grid"
            data-columns=move || state.get().columns_attr
            data-rows=move || state.get().rows_attr
            data-gap=move || state.get().gap_attr
            data-justify=move || state.get().justify_attr
            data-align=move || state.get().align_attr
            data-dense=move || state.get().is_dense.then_some("true")
            data-inline=move || state.get().is_inline.then_some("true")
            data-state=move || state.get().data_state_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
