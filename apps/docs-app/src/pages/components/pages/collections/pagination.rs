use super::*;

pub(crate) fn pagination() -> AnyView {
    let total_pages_options = ["10".to_string(), "20".to_string()];
    let siblings_options = ["0".to_string(), "1".to_string(), "2".to_string()];
    let boundaries_options = ["1".to_string(), "2".to_string()];

    let (showcase_page, set_showcase_page) = signal(1_usize);
    let (showcase_last_change, set_showcase_last_change) = signal(None::<usize>);
    let on_showcase_page_change = Callback::new(move |next: usize| {
        set_showcase_page.set(next);
        set_showcase_last_change.set(Some(next));
    });

    let (workbench_total_pages_index, set_workbench_total_pages_index) = signal(Some(0_usize));
    let (workbench_siblings_index, set_workbench_siblings_index) = signal(Some(1_usize));
    let (workbench_boundaries_index, set_workbench_boundaries_index) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_enable_on_change, set_workbench_enable_on_change) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_page, set_workbench_page) = signal(3_usize);
    let (workbench_last_change, set_workbench_last_change) = signal(None::<usize>);

    let (matrix_first_page, set_matrix_first_page) = signal(1_usize);
    let (matrix_middle_page, set_matrix_middle_page) = signal(6_usize);
    let (matrix_disabled_page, set_matrix_disabled_page) = signal(1_usize);
    let on_matrix_first_page_change =
        Callback::new(move |next: usize| set_matrix_first_page.set(next));
    let on_matrix_middle_page_change =
        Callback::new(move |next: usize| set_matrix_middle_page.set(next));
    let on_matrix_disabled_page_change =
        Callback::new(move |next: usize| set_matrix_disabled_page.set(next));

    let workbench_total_pages = Signal::derive(move || {
        if workbench_total_pages_index.get().unwrap_or(0) == 1 {
            20
        } else {
            10
        }
    });
    let workbench_siblings =
        Signal::derive(move || match workbench_siblings_index.get().unwrap_or(1) {
            0 => 0_usize,
            2 => 2_usize,
            _ => 1_usize,
        });
    let workbench_boundaries = Signal::derive(move || {
        if workbench_boundaries_index.get().unwrap_or(0) == 1 {
            2
        } else {
            1
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench pagination".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-pagination-custom".to_string()
        } else {
            String::new()
        }
    });

    Effect::new(move |_| {
        let total_pages = workbench_total_pages.get().max(1);
        let current = workbench_page.get();
        if current > total_pages {
            set_workbench_page.set(total_pages);
        }
    });

    let on_workbench_page_change = Callback::new(move |next: usize| {
        set_workbench_page.set(next);
        if workbench_enable_on_change.get_untracked() {
            set_workbench_last_change.set(Some(next));
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"let (page, set_page) = signal(1_usize);
let on_page_change = Callback::new(move |next: usize| set_page.set(next));
<Pagination
  total_pages=12
  page=page
  on_page_change=on_page_change
  siblings=1
  boundaries=1
  aria_label="Pagination nav".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let on_page_change_expr = if workbench_enable_on_change.get() {
            "Some(on_workbench_page_change)"
        } else {
            "None"
        };
        format!(
            "<Pagination\n  total_pages={}\n  page=workbench_page\n  default_page=3\n  siblings={}\n  boundaries={}\n  is_disabled={}\n  on_page_change={on_page_change_expr}\n  aria_label={}\n  class_name={}\n/>",
            workbench_total_pages.get(),
            workbench_siblings.get(),
            workbench_boundaries.get(),
            bool_word(workbench_disabled.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "PaginationActualConfig {{\n  total_pages: {},\n  page: Some(workbench_page),\n  default_page: 3,\n  siblings: {},\n  boundaries: {},\n  is_disabled: {},\n  on_page_change: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_total_pages.get(),
            workbench_siblings.get(),
            workbench_boundaries.get(),
            workbench_disabled.get(),
            if workbench_enable_on_change.get() {
                "Some"
            } else {
                "None"
            },
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Pagination total_pages=12 page=matrix_first_page on_page_change=on_matrix_first_page_change siblings=1 boundaries=1 />
<Pagination
  total_pages=12
  page=matrix_middle_page
  on_page_change=on_matrix_middle_page_change
  siblings=2
  boundaries=2
  aria_label="Middle page".to_string()
  class_name="docs-pagination-custom".to_string()
/>
<Pagination
  total_pages=1
  page=matrix_disabled_page
  on_page_change=on_matrix_disabled_page_change
  is_disabled=true
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with real callback feedback and full API workbench coverage."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="pagination-showcase-playground">
                    <Pagination
                        total_pages=12
                        page=showcase_page
                        on_page_change=on_showcase_page_change
                        siblings=1
                        boundaries=1
                        aria_label="Pagination nav".to_string()
                    />
                    <span class="ui-muted">
                        "page: " {move || showcase_page.get()}
                        " · last change: "
                        {move || showcase_last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="pagination-workbench-controls">
                        <div class="docs-search__label">"total_pages"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_total_pages_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_total_pages_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {total_pages_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"siblings"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_siblings_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_siblings_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {siblings_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"boundaries"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_boundaries_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_boundaries_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {boundaries_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |event| set_workbench_disabled.set(event_target_checked(&event))
                            />
                            <span>"disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_enable_on_change.get()
                                on:change=move |event| set_workbench_enable_on_change.set(event_target_checked(&event))
                            />
                            <span>"enable on_page_change callback"</span>
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
                <div class="docs-stack docs-stack--tight" data-slot="pagination-workbench-playground">
                    <Pagination
                        total_pages=workbench_total_pages.get()
                        page=workbench_page
                        default_page=3_usize
                        siblings=workbench_siblings.get()
                        boundaries=workbench_boundaries.get()
                        is_disabled=workbench_disabled.get()
                        on_page_change=on_workbench_page_change
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "page: " {move || workbench_page.get()}
                        " · last_change: "
                        {move || workbench_last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Window / Disabled / Callback Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
            >
                <div class="docs-row docs-row--wrap" data-slot="pagination-matrix-playground">
                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=12
                            page=matrix_first_page
                            on_page_change=on_matrix_first_page_change
                            siblings=1
                            boundaries=1
                        />
                        <span class="ui-muted">"first window: " {move || matrix_first_page.get()}</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=12
                            page=matrix_middle_page
                            on_page_change=on_matrix_middle_page_change
                            siblings=2
                            boundaries=2
                            aria_label="Middle page".to_string()
                            class_name="docs-pagination-custom".to_string()
                        />
                        <span class="ui-muted">"middle window: " {move || matrix_middle_page.get()}</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=1
                            page=matrix_disabled_page
                            on_page_change=on_matrix_disabled_page_change
                            is_disabled=true
                        />
                        <span class="ui-muted">"disabled window: " {move || matrix_disabled_page.get()}</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
