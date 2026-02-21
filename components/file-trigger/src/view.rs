use crate::button::Button;
use crate::file_trigger::{FileTriggerFile, FileTriggerMotion};
use leptos::{ev, html, prelude::*};
use ui_headless::{A11yDirection, FileTriggerOptions, use_file_trigger};

#[component]
pub fn FileTrigger(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] is_multiple: Option<bool>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional, into)] accept: Option<String>,
    #[prop(optional)] is_accept_directory: Option<bool>,
    #[prop(optional)] accept_directory: Option<bool>,
    #[prop(optional, into)] capture: Option<String>,
    #[prop(optional)] motion: FileTriggerMotion,
    #[prop(optional)] on_files: Option<Callback<Vec<FileTriggerFile>>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let on_files = StoredValue::new(on_files);
    let motion = crate::file_trigger::motion::sanitize_motion(motion);
    let render_state =
        super::logic::resolve_render_state(super::logic::FileTriggerRenderStateInput {
            props: super::logic::FileTriggerPropsInput {
                is_disabled,
                disabled,
                is_multiple,
                multiple,
                is_accept_directory,
                accept_directory,
            },
            has_custom_motion: motion != FileTriggerMotion::default(),
        });
    let is_disabled = render_state.props.is_disabled;
    let is_multiple = render_state.props.selection_mode.is_multiple();
    let is_accept_directory = render_state.props.selection_mode.is_accept_directory();
    let state = render_state.state;
    let class_name = super::logic::compose_class_name_from_render_state(render_state);
    let agent_contract = super::logic::resolve_agent_contract(state);

    let semantics = use_file_trigger(FileTriggerOptions { state, lang, dir });
    let lang_attr = semantics.attrs.lang;
    let dir_attr = semantics.attrs.dir;
    let data_state_attr = semantics.attrs.data_state;
    let data_disabled_attr = semantics.attrs.data_disabled;
    let data_enabled_attr = semantics.attrs.data_enabled;
    let input_tabindex = semantics.attrs.input_tabindex;
    let input_aria_hidden = semantics.attrs.input_aria_hidden;

    let on_press = Callback::new(move |_| {
        if !is_disabled {
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
        if is_disabled {
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

            if is_accept_directory {
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
        std::hint::black_box(is_accept_directory);
        std::hint::black_box(capture);
    }

    view! {
        <span
            class=class_name
            lang=lang_attr
            dir=dir_attr
            data-slot="file-trigger"
            data-state=data_state_attr
            data-disabled=data_disabled_attr
            data-enabled=data_enabled_attr
            data-motion-source=state.motion_source_attr
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version
            data-ui-intent=agent_contract.intent.as_attr()
            data-ui-action=agent_contract.action.as_attr()
            data-ui-state=agent_contract.state
            data-ui-source=agent_contract.source.as_attr()
            data-ui-stream-support=agent_contract.stream_support.as_attr()
            data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()
            data-ui-output-status=agent_contract.output_status.as_attr()
        >
            <input
                class="ui-file-trigger__input"
                data-slot="file-trigger-input"
                node_ref=input_ref
                id=id
                type="file"
                accept=accept
                multiple=is_multiple
                disabled=is_disabled
                tabindex=input_tabindex
                aria-hidden=input_aria_hidden
                on:change=on_change
            />
            <Button is_disabled=is_disabled motion=motion.trigger on_press=on_press>
                {children()}
            </Button>
        </span>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
