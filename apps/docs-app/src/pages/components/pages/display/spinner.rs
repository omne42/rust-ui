use super::*;

pub(crate) fn spinner() -> AnyView {
    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_arabic_locale, set_workbench_arabic_locale) = signal(false);

    let workbench_size = Signal::derive(move || match workbench_size_key.get().as_str() {
        "sm" => SpinnerSize::Sm,
        "lg" => SpinnerSize::Lg,
        _ => SpinnerSize::Md,
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Syncing workspace data".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-spinner-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::spinner::SpinnerMotion {
                rotation_duration_ms: 640,
            }
        } else {
            ui::spinner::SpinnerMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_arabic_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_arabic_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Spinner aria_label="Loading activity".to_string() />"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        let size_expr = match workbench_size.get() {
            SpinnerSize::Sm => "SpinnerSize::Sm",
            SpinnerSize::Md => "SpinnerSize::Md",
            SpinnerSize::Lg => "SpinnerSize::Lg",
        };

        format!(
            "<Spinner\n  size={size_expr}\n  aria_label={}\n  class_name={}\n  motion=ui::spinner::SpinnerMotion {{ rotation_duration_ms: {} }}\n  lang={}\n  dir={}\n/>",
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            workbench_motion.get().rotation_duration_ms,
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SpinnerActualConfig {{\n  size: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_size.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Spinner
  size=SpinnerSize::Sm
  aria_label="Fetching notifications".to_string()
  class_name="docs-spinner-custom".to_string()
  motion=ui::spinner::SpinnerMotion { rotation_duration_ms: 480 }
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<Spinner
  size=SpinnerSize::Lg
  aria_label="Loading Arabic inbox".to_string()
  motion=ui::spinner::SpinnerMotion { rotation_duration_ms: 840 }
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Spinner"
            slug="spinner"
            group="Display"
            description="Spinner wraps CircularProgress with centralized size/label/class source attrs."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <Spinner aria_label="Loading activity".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="spinner-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_arabic_locale.get()
                                on:change=move |ev| set_workbench_arabic_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <Spinner
                        size=workbench_size.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <span class="ui-muted">
                        "Configured spinner updates size/label/class/motion/locale in one canvas."
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Size / Motion / Locale Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::{Spinner, SpinnerSize};".to_string()
            >
                <div class="docs-row">
                    <Spinner
                        size=SpinnerSize::Sm
                        aria_label="Fetching notifications".to_string()
                        class_name="docs-spinner-custom".to_string()
                        motion=ui::spinner::SpinnerMotion {
                            rotation_duration_ms: 480,
                        }
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <Spinner
                        size=SpinnerSize::Lg
                        aria_label="Loading Arabic inbox".to_string()
                        motion=ui::spinner::SpinnerMotion {
                            rotation_duration_ms: 840,
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
