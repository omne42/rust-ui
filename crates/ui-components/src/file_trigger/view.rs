use crate::button::Button;
use crate::file_trigger::{FileTriggerFile, FileTriggerMotion};
use leptos::{ev, html, prelude::*};

#[component]
pub fn FileTrigger(
    id: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] multiple: bool,
    #[prop(optional, into)] accept: Option<String>,
    #[prop(optional)] motion: FileTriggerMotion,
    #[prop(optional)] on_files: Option<Callback<Vec<FileTriggerFile>>>,
    children: Children,
) -> impl IntoView {
    let _ = motion;

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let on_files = StoredValue::new(on_files);

    let on_press = Callback::new(move |_| {
        if !disabled {
            #[cfg(target_arch = "wasm32")]
            if let Some(input) = input_ref.get_untracked() {
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
        let files = crate::file_trigger::logic::collect_files_from_input(&input);
        if let Some(cb) = on_files.get_value() {
            cb.run(files);
        }
    };

    view! {
        <span class="ui-file-trigger" data-slot="file-trigger">
            <input
                class="ui-file-trigger__input"
                data-slot="file-trigger-input"
                node_ref=input_ref
                id=id
                type="file"
                accept=accept
                multiple=multiple
                disabled=disabled
                on:change=on_change
            />
            <Button disabled=disabled on_press=on_press>
                {children()}
            </Button>
        </span>
    }
}
