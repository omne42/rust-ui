use crate::button::Button;
use crate::file_trigger::{FileTriggerFile, FileTriggerMotion};
use leptos::{ev, html, prelude::*};

#[component]
pub fn FileTrigger(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] multiple: bool,
    #[prop(optional, into)] accept: Option<String>,
    #[prop(optional)] accept_directory: bool,
    #[prop(optional, into)] capture: Option<String>,
    #[prop(optional)] motion: FileTriggerMotion,
    #[prop(optional)] on_files: Option<Callback<Vec<FileTriggerFile>>>,
    children: Children,
) -> impl IntoView {
    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let on_files = StoredValue::new(on_files);
    let motion = crate::file_trigger::motion::sanitize_motion(motion);
    let has_custom_motion = motion != FileTriggerMotion::default();

    let state = Signal::derive(move || {
        super::logic::resolve_state(super::logic::FileTriggerStateInput {
            disabled,
            has_custom_motion,
        })
    });
    let class = Signal::derive(move || super::logic::compose_class_name(state.get()));

    let on_press = Callback::new(move |_| {
        if !disabled {
            #[cfg(target_arch = "wasm32")]
            if let Some(input) = input_ref.get_untracked() {
                if !input.value().is_empty() {
                    input.set_value("");
                }
                input.click();
            }
        }
    });

    let on_change = move |_ev: ev::Event| {
        if disabled {
            return;
        }
        let Some(input) = input_ref.get_untracked() else {
            return;
        };
        let files = super::logic::collect_files_from_input(&input);
        if let Some(cb) = on_files.get_value() {
            cb.run(files);
        }
    };

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;

        let input_ref = input_ref.clone();
        let capture = capture.clone();

        Effect::new(move |_| {
            let Some(input) = input_ref.get() else {
                return;
            };
            let input: leptos::web_sys::HtmlInputElement = input.unchecked_into();

            if accept_directory {
                drop(input.set_attribute("webkitdirectory", ""));
            } else {
                drop(input.remove_attribute("webkitdirectory"));
            }

            match capture.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
                Some(value) => {
                    drop(input.set_attribute("capture", value));
                }
                None => {
                    drop(input.remove_attribute("capture"));
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::hint::black_box(accept_directory);
        std::hint::black_box(capture);
    }

    view! {
        <span
            class=move || class.get()
            data-slot="file-trigger"
            data-state=move || state.get().state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
        >
            <input
                class="ui-file-trigger__input"
                data-slot="file-trigger-input"
                node_ref=input_ref
                id=id
                type="file"
                accept=accept
                multiple=multiple
                disabled=disabled
                tabindex="-1"
                aria-hidden="true"
                on:change=on_change
            />
            <Button is_disabled=disabled motion=motion.trigger on_press=on_press>
                {children()}
            </Button>
        </span>
    }
}
